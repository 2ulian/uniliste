

use anyhow::{Context, Result};
use bson::{doc, Bson, Document};
use clap::Parser;
use csv::ReaderBuilder;
use mongodb::{options::ClientOptions, Client, Collection};
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
    use std::env;

    // On lit d’abord la variable DATABASE_URL si elle existe.
    // Par défaut on se connecte sur localhost (utile quand on lance le binaire depuis l'hôte
    // et non depuis le réseau docker où le hostname `mongodb` serait résolu).
    let mongo_uri = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mongodb://127.0.0.1:27017/uniliste".into());

    println!("🧭 Tentative de connexion à MongoDB : {}", mongo_uri);

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

    
    // On active l'upsert afin d'insérer si le document n'existe pas.
    let res = collection
        .update_one(filter, update)
        .upsert(true)
        .await
        .context("Erreur lors de l'upsert")?;

    println!("🗃️  update_one result: matched={}, modified={}, upserted={:?}", res.matched_count, res.modified_count, res.upserted_id);
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

#[derive(Debug, Deserialize)]
struct ResourceCsv {
    id: String,
    nom_de_matiere: String,
}

#[derive(Debug, Deserialize)]
struct SecretaryCsv {
    nom: String,
    prenom: String,
}



async fn import_students(collection: Collection<Document>, path: &Path) -> Result<()> {
    println!("Import des étudiants depuis {}", path.display());
    let mut rdr = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
    let mut count = 0;

    for rec in rdr.deserialize::<StudentCsv>() {
        let rec = rec.context("Erreur de parsing CSV étudiant")?;
        let doc = doc! {
            "_id": rec.ine,
            "nom": rec.nom.trim(),
            "prenom": rec.prenom.trim(),
            "age": rec.age.unwrap_or_default(),
            "promo": rec.promo.unwrap_or_default(),
            "groupeTD": rec.groupe_td.unwrap_or_default(),
            "groupeTP": rec.groupe_tp.unwrap_or_default(),
            "groupe": rec.groupe.unwrap_or_default(),
        };

        println!("➡️ Record parsed: INE={:?}, nom={}, prenom={}", rec.ine, rec.nom, rec.prenom);
        upsert_doc(&collection, Bson::Int32(rec.ine), doc).await?;
        println!("➡️ Record parsed: INE={:?}, nom={}, prenom={}", rec.ine, rec.nom, rec.prenom);
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

async fn import_resources(collection: Collection<Document>, path: &Path) -> Result<()> {
    println!("📙 Import des ressources depuis {}", path.display());
    let mut rdr = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
    let mut count = 0;

    for rec in rdr.deserialize::<ResourceCsv>() {
        let rec = rec.context("Erreur de parsing CSV ressource")?;
        let doc = doc! {
            "_id": rec.id.trim(),
            "nom_de_matiere": rec.nom_de_matiere.trim(),
        };

        upsert_doc(&collection, Bson::String(rec.id.trim().to_string()), doc).await?;
        count += 1;
    }

    println!("✅ {count} ressources importées/mises à jour !");
    Ok(())
}


pub async fn run_import(args: Args) -> Result<()> {
    let client = connect_mongo().await?;
    let db = client.database("uniliste");
    let collection = db.collection::<Document>(&args.collection);
    let path = Path::new(&args.file);

    match args.collection.as_str() {
        "students" => import_students(collection, path).await?,
        "teachers" => import_teachers(collection, path).await?,
        "resources" => import_resources(collection, path).await?,
        "secretaries" => import_secretaries(collection, path).await?,
        _ => eprintln!(" Collection inconnue : {}", args.collection),
    }

    Ok(())
}

async fn import_secretaries(collection: Collection<Document>, path: &Path) -> Result<()> {
    println!("📕 Import des secrétaires depuis {}", path.display());
    let mut rdr = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
    let mut count = 0;

    for rec in rdr.deserialize::<SecretaryCsv>() {
        let rec = rec.context("Erreur de parsing CSV secrétaire")?;

        // Génère un ID lisible et stable : nom_prenom en minuscules
        let id_str = format!(
            "{}_{}",
            rec.nom.trim().to_lowercase(),
            rec.prenom.trim().to_lowercase()
        );

        let doc = doc! {
            "_id": id_str.clone(),
            "nom": rec.nom.trim(),
            "prenom": rec.prenom.trim(),
        };

        upsert_doc(&collection, Bson::String(id_str), doc).await?;
        count += 1;
    }

    println!("✅ {count} secrétaires importées/mises à jour !");
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    run_import(args).await?;
    Ok(())
}