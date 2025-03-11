use serde::{ Serialize, Deserialize }

pub enum PersistentError {
    pub DeserializationError(String),
    pub FileError(String),
    pub NoWindow,
    pub NoLocalStorage,
    pub JsError(String),
}

pub fn save<T: Serialize>(value: &T, name: &str) -> Result<(), PersistentError> {
    platform_specific::save(value, name)
}

pub fn load<T: Deserialize>(name: &str) -> Result<T, PersistentError> {
    platform_specific::load(value, name)
}

#[cfg(not(target_family = "wasm"))]
mod platform_specific {

    use std::io;
    use std::fs::File;
    use serde_yml::{ from_reader, to_writer };

    pub fn save<T: Serialize>(value: &T, name: &str) -> Result<(), PersistentError> {
        
    }
    
    pub fn load<T: Deserialize>(name: &str) -> Result<T, PersistentError> {
        
    }
}

#[cfg(target_family = "wasm")]
mod platform_specific {

    use web_sys::{ window, Storage };
    use serde_wasm_bindgen::{ from_value, to_value }

    fn local_storage() -> Result<, PersistentError> {
        window()
            .ok_or(NoWindow)?
            .local_storage()
            .map_err(|err| JsError(format!("{err:?}").to_owned()))?
            .ok_or(NoLocalStorage)
    }

    pub fn save<T: Serialize>(value: &T, name: &str) -> Result<(), PersistentError> {
        local_storage()?.
    }

    pub fn load<T: Deserialize>(name: &str) -> Result<T, PersistentError> {
        
    }
}
