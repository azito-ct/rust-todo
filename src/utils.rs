use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn save<T: Serialize>(dest: &str, data: &T) -> Result<(), String> {
    let path = Path::new(dest);
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    let data = serde_json::to_string(data).map_err(|e| e.to_string())?;

    file.write_all(data.as_bytes()).map_err(|e| e.to_string())
}

pub fn load<T: DeserializeOwned>(source: &str) -> Result<Option<T>, String> {
    let path = Path::new(source);
    match File::open(path).map_err(|e| e.to_string()) {
        Ok(mut file) => {
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

            serde_json::from_slice::<T>(&buffer).map_err(|e| e.to_string())
            .map(Some)
        }

        Err(_) => Ok(None),
    }
}