use std::fs;

use crate::store::load;

pub fn run(to_file: Option<&str>) -> Result<(), String> {
    let store = load()?;

    let json = serde_json::to_string_pretty(&store)
        .map_err(|e| format!("Sérialisation échouée: {e}"))?;

    match to_file {
        Some(path) => {
            fs::write(path, &json)
                .map_err(|e| format!("Écriture {path} échouée: {e}"))?;
            println!("Exporté → {path}");
        }
        None => println!("{json}"),
    }

    Ok(())
}
