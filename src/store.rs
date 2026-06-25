use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::models::{Config, Store};

pub fn kanban_dir() -> PathBuf {
    Path::new(".kanban").to_path_buf()
}

pub fn data_path() -> PathBuf {
    kanban_dir().join("kb-data.yaml")
}

fn legacy_data_path() -> PathBuf {
    kanban_dir().join("kanban.md")
}

fn legacy_config_path() -> PathBuf {
    kanban_dir().join("kb-config.yaml")
}

#[allow(dead_code)]
pub fn load_config() -> Result<Config, String> {
    if data_path().exists() {
        return Ok(load()?.config);
    }

    if legacy_config_path().exists() {
        let content = fs::read_to_string(legacy_config_path())
            .map_err(|e| format!("Lecture config échouée: {e}"))?;
        return serde_yaml::from_str(&content)
            .map_err(|e| format!("Parse config échoué: {e}"));
    }

    Ok(Config::default())
}

fn ensure_kanban_dir() -> Result<(), String> {
    let dir = kanban_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Création dossier .kanban échouée: {e}"))?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn save_config(config: &Config) -> Result<(), String> {
    let mut store = load().unwrap_or_default();
    store.config = config.clone();
    save(&store)
}

pub fn is_initialized() -> bool {
    data_path().exists() || legacy_data_path().exists()
}

pub fn load() -> Result<Store, String> {
    if data_path().exists() {
        let content = fs::read_to_string(data_path())
            .map_err(|e| format!("Lecture kb-data.yaml échouée: {e}"))?;
        return parse_store_yaml(&content).map_err(|e| format!("Parse kb-data.yaml échoué: {e}"));
    }

    if legacy_data_path().exists() {
        return load_legacy_store();
    }

    if legacy_config_path().exists() {
        let mut store = Store::default();
        store.config = load_legacy_config()?;
        return Ok(store);
    }

    Err("kb-data.yaml non trouvé. Lance `kb init` d'abord.".to_string())
}

pub fn save(store: &Store) -> Result<(), String> {
    ensure_kanban_dir()?;
    let yaml = serde_yaml::to_string(store)
        .map_err(|e| format!("Sérialisation YAML échouée: {e}"))?;
    fs::write(data_path(), yaml)
        .map_err(|e| format!("Écriture kb-data.yaml échouée: {e}"))
}

fn load_legacy_store() -> Result<Store, String> {
    let content = fs::read_to_string(legacy_data_path())
        .map_err(|e| format!("Lecture kanban.md échouée: {e}"))?;
    let yaml = extract_frontmatter(&content)?;
    let mut store: Store = parse_store_yaml(&yaml)
        .map_err(|e| format!("Parse YAML échoué: {e}"))?;

    if legacy_config_path().exists() {
        store.config = load_legacy_config()?;
    }

    Ok(store)
}

fn load_legacy_config() -> Result<Config, String> {
    let content = fs::read_to_string(legacy_config_path())
        .map_err(|e| format!("Lecture config échouée: {e}"))?;
    serde_yaml::from_str(&content)
        .map_err(|e| format!("Parse config échoué: {e}"))
}

fn parse_store_yaml(content: &str) -> Result<Store, serde_yaml::Error> {
    let content = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut docs = serde_yaml::Deserializer::from_str(content);
    let doc = docs.next().ok_or_else(|| <serde_yaml::Error as serde::de::Error>::custom("empty YAML document"))?;
    Store::deserialize(doc)
}

fn extract_frontmatter(content: &str) -> Result<String, String> {
    let normalized = content
        .strip_prefix('\u{feff}')
        .unwrap_or(content)
        .replace("\r\n", "\n");
    let after_first = normalized
        .strip_prefix("---\n")
        .ok_or("kanban.md: frontmatter YAML manquant (doit commencer par ---)")?;
    let end = after_first
        .find("\n---\n")
        .ok_or("kanban.md: frontmatter non fermé")?;
    Ok(after_first[..end + 1].to_string())
}
