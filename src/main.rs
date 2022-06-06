extern crate dotenv;
mod configuration;
mod model;

use configuration::db;
use model::uf::Entity as Uf;
use dotenv::dotenv;
use sea_orm::{EntityTrait};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = db::estabilish_connection().await;
    let ufs = Uf::find().all(&db).await.unwrap();
    println!("{:?}", ufs);
}
