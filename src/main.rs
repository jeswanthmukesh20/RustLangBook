use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Error};
use std::fs::OpenOptions;
use std::io::{Write};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{
    bson::{Document, doc},
    Client,
    Collection
};
use mongodb::bson;
use serde_json::Value;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct NoUsers {
    no_users: i32
}

fn json_bytes<T: Serialize>(structure: &T) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    serde_json::to_writer_pretty(&mut bytes, structure).unwrap();
    bytes
}

#[get("/upload_users")]
async fn manual_hello(data: web::Query<NoUsers>) -> impl Responder {
    let mut users: Vec<Value> = Vec::new();
    get_users(data.no_users, &mut users).await.unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("response.json")
        .unwrap();
    let uri = "mongodb://localhost:27017";

    let client = Client::with_uri_str(uri).await.unwrap();

    let database = client.database("RandomUser");
    let my_coll: Collection<Document> = database.collection("Users");
    let mut docs: Vec<Document> = Vec::new();
    for user in &users {
        docs.push(bson::to_document(&user).unwrap())
    }
    let resp = my_coll.insert_many(docs, None).await.unwrap();
    println!("{:?}", resp.inserted_ids);
    let bytes: Vec<u8> = json_bytes(&users);
    file.write_all(&bytes).unwrap();
    return web::Json(users);
}
async fn get_users(no_users: i32, users: &mut Vec<Value>) -> Result<(), Error>{
    let link = format!("https://randomuser.me/api/?results={no_users}");
    let response = reqwest::get(link.as_str()).await.unwrap();
    println!("{}", response.status());
    let body: serde_json::Value  = response.json().await.unwrap();
    let results =  body["results"].as_array().unwrap();
    users.extend_from_slice(results);
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()

            .service(hello)
            .service(manual_hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}