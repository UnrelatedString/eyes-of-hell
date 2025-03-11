use serde::{ Serialize, Deserialize }

pub fn save<T: Serialize>(value: &T, name: &str) -> () {
    platform_specific::save(value, name)
}

pub fn load<T: Deserialize>(name: &str) -> T {
    platform_specific::load(value, name)
}

#[cfg(not(target_family = "wasm"))]
mod platform_specific {

    use std::io;
    use std::fs::File;

    pub fn save<T: Serialize>(value: &T, name: &str) -> () {
        
    }
    
    pub fn load<T: Deserialize>(name: &str) -> T {
        
    }
}

#[cfg(target_family = "wasm")]
mod platform_specific {
    pub fn save<T: Serialize>(value: &T, name: &str) -> () {
        
    }

    pub fn load<T: Deserialize>(name: &str) -> T {
        
    }
}
