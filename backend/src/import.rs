use anyhow::{Context, Result};
use bson::{doc, Bson, Document};
use clap::Parser;
use csv::ReaderBuilder;
use mongodb::{options::{ClientOptions, UpdateOptions}, Client, Collection};
use serde::Deserialize;
use std::path::Path;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "Import CSV")]
#[command(about = "Importe des données CSV vers MongoDB avec upsert automatique", long_about = None)]
pub struct Args {
    /// Nom de la collection (students, teachers, resources, secretaries)
    #[arg(short, long)]
    pub collection: String,

    /// Chemin vers le fichier CSV
    #[arg(short, long)]
    pub file: String,
}

/// Connecte à MongoDB via DATABASE_URL ou valeur par défaut
async fn connect_mongo() -> Result<Client> {
    let mongo_uri = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mongodb://mongodb:27017/uniliste".into());

    let opts = ClientOptions::parse(&mongo_uri)
        .await
        .context("Erreur lors du parsing de l'URI MongoDB")?;
    let client = Client::with_options(opts)?;
    Ok(client)
}