use dotenvy::dotenv;
use std::env;

fn main() {
    // Charge le fichier .env
    dotenv().ok();

    // Récupère les variables d'environnement
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:healthtrack.db".to_string());

    let _api_key = env::var("API_KEY").expect("API_KEY must be set in .env file");

    let debug_mode = env::var("DEBUG_MODE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    println!("Database: {}", database_url);
    println!("Debug mode: {}", debug_mode);
    // N'affiche JAMAIS l'API key dans les logs !

    // Ton code ici...
    println!("Hello, world !")
}
