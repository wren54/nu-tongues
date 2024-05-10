use serde::Deserialize;
use toml::Table as TomlTable;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct LanguageToml {
    language: String,
    territory: String,
    modifier: String,
    fallback: String,
    messages: TomlTable,
}

#[allow(dead_code)]
impl LanguageToml {
    pub fn get_language(&self) -> String {
        self.language.clone()
    }
    pub fn get_territory(&self) -> String {
        self.territory.clone()
    }
    pub fn get_modifier(&self) -> String {
        self.modifier.clone()
    }
    pub fn get_fallback(&self) -> String {
        self.fallback.clone()
    }
    pub fn get_messages(&self) -> TomlTable {
        self.messages.clone()
    }
}
