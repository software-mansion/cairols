use crate::lang::proc_macros::{
    client::plain_request_response::{
        PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
    },
    db::ProcMacroGroup,
};
use bincode::config::standard;
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub fn save_proc_macro_cache(db: &dyn ProcMacroGroup) {
    let Some(cache_path) = cache_path() else { return };

    let resolution = Resolution {
        attr: db.attribute_macro_resolution(),
        derive: db.derive_macro_resolution(),
        inline: db.inline_macro_resolution(),
    };

    let buffer =
        bincode::serde::encode_to_vec(resolution, standard()).expect("serialize should not fail");
    let _ = std::fs::write(&cache_path, buffer);
}

pub fn load_proc_macro_cache(db: &mut dyn ProcMacroGroup) {
    let Some(cache_path) = cache_path() else { return };

    if let Ok(buffer) = std::fs::read(&cache_path) {
        if let Ok((resolution, _)) =
            bincode::serde::decode_from_slice::<Resolution, _>(&buffer, standard())
        {
            db.set_attribute_macro_resolution(resolution.attr);
            db.set_derive_macro_resolution(resolution.derive);
            db.set_inline_macro_resolution(resolution.inline);
        }
    }
}

fn cache_path() -> Option<PathBuf> {
    std::env::current_dir().ok().map(|mut cache_path| {
        cache_path.push("proc_macro.cache");
        cache_path
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Resolution {
    attr: Arc<HashMap<PlainExpandAttributeParams, ProcMacroResult>>,
    derive: Arc<HashMap<PlainExpandDeriveParams, ProcMacroResult>>,
    inline: Arc<HashMap<PlainExpandInlineParams, ProcMacroResult>>,
}
