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

/// Effectue un upsert générique : met à jour si le document existe, sinon insère
async fn upsert_doc(collection: &Collection<Document>, id: Bson, doc: Document) -> Result<()> {
    let filter = doc! { "_id": id.clone() };
    let update = doc! { "$set": doc };
    let opts = UpdateOptions::builder().upsert(true).build();

    collection
        .update_one(filter, update, opts)
        .await
        .context("Erreur lors de l'upsert")?;
    Ok(())
}


#[derive(Debug, Deserialize)]
struct StudentCsv {
    ine: i32,
    nom: String,
    prenom: String,
    age: Option<i32>,
    promo: Option<i32>,
    groupe_td: Option<i32>,
    groupe_tp: Option<i32>,
    groupe: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TeacherCsv {
    id: String,
    nom: String,
    prenom: String,
    numero_ressources: String,
}



async fn import_students(collection: Collection<Document>, path: &Path) -> Result<()> {
    println!("Import des étudiants depuis {}", path.display());
    let mut rdr = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
    let mut count = 0;

    for rec in rdr.deserialize::<StudentCsv>() {
        let rec = rec.context("Erreur de parsing CSV étudiant")?;
        let doc = doc! {
            "_id": rec.INE,
            "nom": rec.nom.trim(),
            "prenom": rec.prenom.trim(),
            "age": rec.age.unwrap_or_default(),
            "promo": rec.promo.unwrap_or_default(),
            "groupeTD": rec.groupeTD.unwrap_or_default(),
            "groupeTP": rec.groupeTP.unwrap_or_default(),
            "groupe": rec.groupe.unwrap_or_default(),
        };

        upsert_doc(&collection, Bson::Int32(rec.INE), doc).await?;
        count += 1;
    }

    println!("{count} étudiants importés/mis à jour !");
    Ok(())
}


async fn import_teachers(collection: Collection<Document>, path: &Path) -> Result<()> {
    println!("Import des professeurs depuis {}", path.display());
    let mut rdr = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
    let mut count = 0;

    for rec in rdr.deserialize::<TeacherCsv>() {
        let rec = rec.context("Erreur de parsing CSV professeur")?;
        let ressources: Vec<String> = rec
            .numero_ressources
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let doc = doc! {
            "_id": rec.id.trim(),
            "nom": rec.nom.trim(),
            "prenom": rec.prenom.trim(),
            "numero_ressources": ressources,
        };

        upsert_doc(&collection, Bson::String(rec.id.trim().to_string()), doc).await?;
        count += 1;
    }

    println!("{count} professeurs importés/mis à jour !");
    Ok(())
}
