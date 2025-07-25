use std::{collections::HashMap, env::current_dir, fs, path::PathBuf, sync::Arc};

use bincode::{
    config::standard,
    serde::{decode_from_slice, encode_to_vec},
};
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    config::Config,
    env_config::scarb_target_path,
    lang::proc_macros::{
        client::plain_request_response::{
            PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
        },
        db::ProcMacroGroup,
    },
};

pub fn save_proc_macro_cache(db: &dyn ProcMacroGroup, config: &Config) {
    if !config.enable_experimental_proc_macro_cache {
        return;
    }

    let Some(cache_path) = cache_path() else { return };

    let resolution = Resolution {
        attr: db.attribute_macro_resolution(),
        derive: db.derive_macro_resolution(),
        inline: db.inline_macro_resolution(),
    };

    let buffer = encode_to_vec(resolution, standard()).expect("serialize should not fail");

    let _ = fs::create_dir_all(cache_path.parent().expect("parent must exist"));

    if let Err(err) = fs::write(&cache_path, buffer) {
        error!("failed to save proc macro cache to disk {err:?}");
    }
}

pub fn load_proc_macro_cache(db: &mut dyn ProcMacroGroup, config: &Config) {
    let mut resolution = if config.enable_experimental_proc_macro_cache
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
            let map = Arc::get_mut(&mut resolution.$prop).unwrap();
            for (key, value) in db.$query().iter() {
                map.insert(key.clone(), value.clone());
            }
            db.$set_query(resolution.$prop);
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
    attr: Arc<HashMap<PlainExpandAttributeParams, ProcMacroResult>>,
    derive: Arc<HashMap<PlainExpandDeriveParams, ProcMacroResult>>,
    inline: Arc<HashMap<PlainExpandInlineParams, ProcMacroResult>>,
}
