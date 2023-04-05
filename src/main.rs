use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::process::exit;
use mongodb::bson::Document;
use rocket::futures::StreamExt;
// use mongodb::bson::Document;
// use rocket::futures::StreamExt;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return "Hello";
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    if let Err(_err) = options {
        exit(1);
    }

    let options = options.unwrap();
    let client = Client::with_options(options);

    if let Err(_err) = client {
        exit(1);
    }

    let client = client.unwrap();

    let transactions_collection: Collection<Document> = client.database("vybe").collection("transactions");
    // let cursors: Cursor<Document> = transactions_collection.find(None, None).ok().expect("Error getting trasnactions");
    let cursor = transactions_collection.find(None, None).await;
    if let Err(_err) = cursor {
        exit(1);
    }

    let mut cursor = cursor.unwrap();

    // REFERENCE: ChatGPT
    while let Some(document) = cursor.next().await {
        match document {
            Ok(doc) => {
                println!("Document: {:?}", doc);
            }
            Err(err) => {
                println!("Failed to retrieve document: {:?}", err)
            }
        }
    }

    // let docs = cursors.collect();
    // let transactions = cursors.map(|doc| doc.unwrap()).collect();

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    // for name in client.list_database_names(None, None).await? {
    //     println!("- {}", name);
    // }
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}