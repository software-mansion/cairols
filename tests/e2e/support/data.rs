use std::path::PathBuf;

macro_rules! data {
    ($file:expr) => {
        $crate::support::data::load_data(file!(), $file)
    };
}

pub(crate) use data;

#[doc(hidden)]
pub fn load_data(current_file: &str, file_to_include: impl Into<PathBuf>) -> String {
    let current_file = PathBuf::from(current_file);
    let file_to_include = file_to_include.into();

    let current_dir = current_file.parent().unwrap();

    let file_to_include = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(current_dir.join("data").join(file_to_include));

    std::fs::read_to_string(file_to_include).unwrap()
}
