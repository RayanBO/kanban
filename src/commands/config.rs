use crate::store::{load, save};

pub fn run(set_pairs: Vec<(String, String)>) -> Result<(), String> {
    let mut store = load()?;

    for (key, value) in set_pairs {
        match key.as_str() {
            "use_trash" | "use-trash" => {
                store.config.use_trash = value
                    .parse::<bool>()
                    .map_err(|_| "use_trash doit être true ou false".to_string())?;
            }
            "theme_dashboard" | "theme-dashboard" => {
                store.config.theme_dashboard = value;
            }
            _ => return Err(format!("Clé de config inconnue: {key}")),
        }
    }

    save(&store)?;

    println!("=== Configuration ===");
    println!("  use_trash:       {}", store.config.use_trash);
    println!("  theme_dashboard: {}", store.config.theme_dashboard);

    Ok(())
}
