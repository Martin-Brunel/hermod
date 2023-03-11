use diesel::mysql::MysqlConnection as SQLConnection;

use diesel::prelude::*;
use dotenvy::dotenv;

use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok(); //charge les variables présente dans le .env dans l'environnement
    
    let database_url = env::var("DATABASE_URL") //on tente de récuperer l'url de la BDD depuis l'environnement
    .expect("DATABASE_URL must be set"); //si elle n'existe pas on lève une erreur
    
    SQLConnection::establish(&database_url) //on tente d'établir une connexion avec la BDD
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)) //on retourne cette connexion (ou une erreur si connexion impossible)
}