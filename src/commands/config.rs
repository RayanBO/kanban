use crate::store::{load_config, save_config};

pub fn run(set_pairs: Vec<(String, String)>) -> Result<(), String> {
    let mut config = load_config()?;

    for (key, value) in set_pairs {
        match key.as_str() {
            "use_trash" | "use-trash" => {
                config.use_trash = value
                    .parse::<bool>()
                    .map_err(|_| "use_trash doit être true ou false".to_string())?;
            }
            "theme_dashboard" | "theme-dashboard" => {
                config.theme_dashboard = value;
            }
            _ => return Err(format!("Clé de config inconnue: {key}")),
        }
    }

    save_config(&config)?;

    println!("=== Configuration ===");
    println!("  use_trash:       {}", config.use_trash);
    println!("  theme_dashboard: {}", config.theme_dashboard);

    Ok(())
}
