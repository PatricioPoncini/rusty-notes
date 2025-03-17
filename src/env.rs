use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;

static ENV_STORE: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn init_env() -> Result<(), String> {
    dotenv().ok();

    let required_vars = ["PORT", "DATABASE_URL"];
    let mut env_map = HashMap::new();

    for var_name in required_vars.iter() {
        match env::var(var_name) {
            Ok(value) => {
                env_map.insert(var_name.to_string(), value);
            }
            Err(_) => return Err(format!("Missing environment variable: {}", var_name)),
        }
    }

    match ENV_STORE.set(env_map) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to initialize environment store".to_string()),
    }
}

pub fn get_env(key: &str) -> String {
    ENV_STORE
        .get()
        .expect("Environment store not initialized")
        .get(key)
        .cloned()
        .unwrap_or_else(|| panic!("Environment variable '{}' not found", key))
}
