use std::{collections::HashMap, env::current_dir, fs, path::PathBuf};

use bincode::{
    config::standard,
    serde::{decode_from_slice, encode_to_vec},
};
use salsa::{Database, Setter};
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::{
    config::Config,
    env_config::scarb_target_path,
    lang::proc_macros::client::plain_request_response::{
        PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
    },
};

pub fn save_proc_macro_cache(db: &dyn Database, config: &Config) {
    if !config.enable_experimental_proc_macro_cache {
        return;
    }

    let Some(cache_path) = cache_path() else { return };

    let resolution = Resolution {
        attr: db.proc_macro_input().attribute_macro_resolution(db).clone(),
        derive: db.proc_macro_input().derive_macro_resolution(db).clone(),
        inline: db.proc_macro_input().inline_macro_resolution(db).clone(),
    };

    let buffer = encode_to_vec(resolution, standard()).expect("serialize should not fail");

    let _ = fs::create_dir_all(cache_path.parent().expect("parent must exist"));

    if let Err(err) = fs::write(&cache_path, buffer) {
        error!("failed to save proc macro cache to disk {err:?}");
    }
}

pub fn try_load_proc_macro_cache(db: &mut dyn Database, config: &Config) {
    let resolution = if config.enable_experimental_proc_macro_cache
        && let Some(cache_path) = cache_path()
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
        cache_path.push("proc_macro.cache");
        cache_path
    })
}

fn current_dir_target() -> Option<PathBuf> {
    current_dir().ok().map(|mut cache_path| {
        cache_path.push("target");
        cache_path
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Resolution {
    attr: HashMap<PlainExpandAttributeParams, ProcMacroResult>,
    derive: HashMap<PlainExpandDeriveParams, ProcMacroResult>,
    inline: HashMap<PlainExpandInlineParams, ProcMacroResult>,
}
