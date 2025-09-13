use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn mime_from_file_path(path: &PathBuf) -> Option<String> {
    let known_types = HashMap::from([("wav", "audio/wav")]);

    let path = Path::new(&path);
    let extension = path.extension();

    match extension {
        Some(extension) => {
            let ext = extension.to_str().unwrap();
            let mime = known_types.get(ext);
            match mime {
                Some(mime) => {
                    return Some(mime.to_string());
                }
                None => None,
            }
        }
        None => None,
    }
}
