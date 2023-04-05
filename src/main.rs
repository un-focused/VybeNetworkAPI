use mongodb::{Client, Collection};
use rocket::futures::StreamExt;
use rocket::{serde::json::Json};
use std::env;
use std::process::exit;
use mongodb::bson::de::Error;
use mongodb::bson::{doc, Document};
use rocket::http::Status;
use rocket::{State};

// use mongodb::bson::Document;
// use rocket::futures::StreamExt;
// extern crate dotenv;

use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index(
    repo: &State<MongoRepo>
) -> Result<Json<Vec<Document>>, Status> {
    let transactions = repo.get_transactions().await.unwrap();
    let res = Json(transactions);

    Ok(res)
}

struct MongoRepo {
    collection: Collection::<Document>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        // Load the MongoDB connection string from an environment variable:
        let client_uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading evn variable"),
        };

        let client = Client::with_uri_str(client_uri).await;

        if let Err(_err) = client {
            exit(1);
        }

        let client = client.unwrap();

        let transactions_collection = client.database("vybe").collection::<Document>("transactions");

        MongoRepo {
            collection: transactions_collection
        }
    }

    pub async fn get_transactions(&self) -> Result<Vec<Document>, Error> {
        let cursor = self.collection.find(None, None).await.ok();

        let mut cursor = cursor.unwrap();

        let mut docs: Vec<Document> = Vec::new();

        // REFERENCE: ChatGPT
        while let Some(document) = cursor.next().await {
            match document {
                Ok(doc) => {
                    docs.push(doc)
                }
                Err(err) => {
                    println!("Failed to retrieve document: {:?}", err)
                }
            }
        }

        Ok(docs)
    }
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let mongo_repo = MongoRepo::init().await;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    // for name in client.list_database_names(None, None).await? {
    //     println!("- {}", name);
    // }
    let rocket = rocket::build().manage(mongo_repo).mount("/", routes![index]);

    Ok(rocket.into())
}