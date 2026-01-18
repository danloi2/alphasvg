use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Embed translations into the binary for fallback/standalone use
static SUB_LOCALE_ES: &str = include_str!("../locales/es.json");
static SUB_LOCALE_EN: &str = include_str!("../locales/en.json");
static SUB_LOCALE_EU: &str = include_str!("../locales/eu.json");
static SUB_LOCALE_LA: &str = include_str!("../locales/la.json");

#[derive(Clone)]
pub struct LanguageManager {
    translations: Arc<Mutex<HashMap<String, String>>>,
    current_lang: Arc<Mutex<String>>,
}

impl Default for LanguageManager {
    fn default() -> Self {
        let mut manager = Self {
            translations: Arc::new(Mutex::new(HashMap::new())),
            current_lang: Arc::new(Mutex::new("en".to_string())),
        };
        // Load default English immediately
        manager.load_language("en"); 
        manager
    }
}

impl LanguageManager {
    pub fn load_language(&mut self, lang_code: &str) {
        // Try to load from external file "locales/{code}.json" to allow user editing.
        // If not found, use the embedded version (compile-time).
        
        let path = format!("locales/{}.json", lang_code);
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| {
                // Fallback to embedded files
                match lang_code {
                    "es" => SUB_LOCALE_ES.to_string(),
                    "en" => SUB_LOCALE_EN.to_string(),
                    "eu" => SUB_LOCALE_EU.to_string(),
                    "la" => SUB_LOCALE_LA.to_string(),
                    _ => "{}".to_string()
                }
            });

        if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&content) {
            *self.translations.lock().unwrap() = map;
            *self.current_lang.lock().unwrap() = lang_code.to_string();
        } 
    }

    pub fn t(&self, key: &str) -> String {
        let guard = self.translations.lock().unwrap();
        guard.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
    
    #[allow(dead_code)]
    pub fn current_lang(&self) -> String {
        self.current_lang.lock().unwrap().clone()
    }
}
