use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    hash::Hash,
    path::PathBuf,
    sync::{
        Arc, Once,
        atomic::{AtomicBool, Ordering},
    },
};

use bincode::{
    config::standard,
    serde::{decode_from_slice, encode_to_vec},
};
use salsa::{Database, Setter};
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use serde::{Deserialize, Serialize, ser::SerializeMap};
use tracing::error;

use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::{
    env_config::scarb_target_path,
    lang::proc_macros::client::plain_request_response::{
        PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
    },
};

/// [`HashMap`] wrapper with tracking of read values
///
/// Each newly added value is considered unread. Will be marked as read after first fetch with [`Self::get`].
/// All unread values can be removed with [`Self::erase_not_used`]
///
/// Its [`Hash`] implementation will produce equal results to this of regular [`HashMap`]
#[derive(Debug, Clone)]
pub struct ProcMacroCache<K, V> {
    /// [`Self::get`] takes not mutable reference so use internal mutability (thread safe) for read tracking.
    inner: HashMap<K, (V, Arc<AtomicBool>)>,
}

impl<K: Hash + Eq, V> ProcMacroCache<K, V> {
    fn new(map: HashMap<K, V>) -> Self {
        Self {
            inner: map
                .into_iter()
                .map(|(key, value)| (key, (value, AtomicBool::new(false).into())))
                .collect(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, (value, AtomicBool::new(false).into()));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key).map(|(value, was_used)| {
            was_used.store(true, Ordering::Relaxed);
            value
        })
    }

    fn erase_unused(&mut self) {
        self.inner.retain(|_, (_, was_used)| was_used.load(Ordering::Relaxed));
    }

    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter().map(|(key, (value, _))| (key, value))
    }
}

impl<K: Hash + Eq, V> Default for ProcMacroCache<K, V> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<K: Serialize, V: Serialize> Serialize for ProcMacroCache<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let entries: Vec<_> = self
            .inner
            .iter()
            .filter(|(_, (_, was_used))| was_used.load(Ordering::Relaxed))
            .map(|(key, (value, _))| (key, value))
            .collect();

        let mut serialize_map = serializer.serialize_map(Some(entries.len()))?;
        entries
            .into_iter()
            .try_for_each(|(key, value)| serialize_map.serialize_entry(key, value))?;

        serialize_map.end()
    }
}

impl<'de, K: Deserialize<'de> + Hash + Eq, V: Deserialize<'de>> Deserialize<'de>
    for ProcMacroCache<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::new(Deserialize::deserialize(deserializer)?))
    }
}

pub fn save_proc_macro_cache(db: &dyn Database) {
    let Some(cache_path) = cache_path() else { return };

    let mut resolution = Resolution {
        attr: db.proc_macro_input().attribute_macro_resolution(db).clone(),
        derive: db.proc_macro_input().derive_macro_resolution(db).clone(),
        inline: db.proc_macro_input().inline_macro_resolution(db).clone(),
    };

    static START: Once = Once::new();

    // Stale values are cleared only after the initial analysis pass.
    // In later passes, [`salsa`] may skip certain queries, so some still-relevant
    // values might not be visited.
    START.call_once(|| {
        resolution.attr.erase_unused();
        resolution.derive.erase_unused();
        resolution.inline.erase_unused();
    });

    let buffer = encode_to_vec(resolution, standard()).expect("serialize should not fail");

    let _ = fs::create_dir_all(cache_path.parent().expect("parent must exist"));

    if let Err(err) = fs::write(&cache_path, buffer) {
        error!("failed to save proc macro cache to disk {err:?}");
    }
}

pub fn try_load_proc_macro_cache(db: &mut dyn Database) {
    let resolution = if let Some(cache_path) = cache_path()
        && let Ok(buffer) = fs::read(&cache_path)
        && let Ok((resolution, _)) = decode_from_slice::<Resolution, _>(&buffer, standard())
    {
        resolution
    } else {
        return;
    };

    macro_rules! override_with_local {
        ($prop:ident, $query:ident, $set_query:ident) => {
            let mut map = resolution.$prop;
            for (key, value) in db.proc_macro_input().$query(db).iter() {
                map.insert(key.clone(), value.clone());
            }
            db.proc_macro_input().$set_query(db).to(map);
        };
    }

    override_with_local!(attr, attribute_macro_resolution, set_attribute_macro_resolution);
    override_with_local!(derive, derive_macro_resolution, set_derive_macro_resolution);
    override_with_local!(inline, inline_macro_resolution, set_inline_macro_resolution);
}

fn cache_path() -> Option<PathBuf> {
    scarb_target_path().or_else(current_dir_target).map(|mut cache_path| {
        cache_path.push("cairo-language-server");
        cache_path.push(cache_file_name());
        cache_path
    })
}

fn cache_file_name() -> String {
    let pkg = env!("CARGO_PKG_VERSION");
    // Use commit for file name, so each LS version (even dev ones) will use different cache
    // It is unavailable when building from `crates.io` (there is no `.git` dir). Use only version in this case.
    let commit = option_env!("LS_COMMIT_HASH");
    let separator = if commit.is_some() { "-" } else { "" };
    let commit = commit.unwrap_or_default();

    format!("{pkg}{separator}{commit}_proc_macro.cache",)
}

fn current_dir_target() -> Option<PathBuf> {
    current_dir().ok().map(|mut cache_path| {
        cache_path.push("target");
        cache_path
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Resolution {
    attr: ProcMacroCache<(PlainExpandAttributeParams, u64), ProcMacroResult>,
    derive: ProcMacroCache<(PlainExpandDeriveParams, u64), ProcMacroResult>,
    inline: ProcMacroCache<(PlainExpandInlineParams, u64), ProcMacroResult>,
}
