use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub size: String,
    pub extra: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Purchase {
    pub id: String,
    pub employee: String,
    pub store: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub address: Address,
}

#[derive(Serialize, Deserialize)]
pub struct Payment {
    pub method: String,
    pub date: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub email: String,
    pub username: String,
    pub profile: Profile,
    pub purchase: Purchase,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub payment: Payment,
}
