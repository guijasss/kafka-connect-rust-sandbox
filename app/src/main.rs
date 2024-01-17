use crate::models::transaction::{Address, Payment, Product, Profile, Purchase, Transaction};
use chrono::Utc;
use mongodb::{Client, Database};
use mongodb::bson::{DateTime, to_document};
use rand::Rng;

pub mod models;

async fn insert_data_to_mongodb(data: &Transaction, db: &Database) -> mongodb::error::Result<()> {
    let collection = db.collection("transactions");

    let document = to_document(data)?;

    collection.insert_one(document, None).await?;

    Ok(())
}

fn generate_random_tenant_id() -> String {
    let mut rng = rand::thread_rng();
    let tenant_number = rng.gen_range(1..=3); // Adjust the range based on your requirement
    format!("tenant{}", tenant_number)
}

#[tokio::main]
async fn main() {
    let data = Transaction {
        id: "12345678".to_string(),
        email: "example@mail.com".to_string(),
        username: "exampleUser".to_string(),
        profile: Profile {
            name: "John Doe".to_string(),
            address: Address {
                street: "123 Main St".to_string(),
                city: "Cityville".to_string(),
                state: "ST".to_string(),
            },
        },
        purchase: Purchase {
            id: "abcd-efgh-ijkl-mnop".to_string(),
            employee: "Employee Name".to_string(),
            store: "Store Name".to_string(),
            products: vec![
                Product {
                    name: "Product 1".to_string(),
                    size: "M".to_string(),
                    extra: vec!["Onion".to_string(), "Cheese".to_string()],
                },
                Product {
                    name: "Product 2".to_string(),
                    size: "L".to_string(),
                    extra: vec!["Meat".to_string(), "French Fries".to_string()],
                },
            ],
        },
        created_at: DateTime(Utc::now()),
        updated_at: DateTime(Utc::now()),
        payment: Payment {
            method: "Credit Card".to_string(),
            date: DateTime(Utc::now())
        }
    };

    let mongodb_uri: String = "mongodb://127.0.0.1:27017/?replicaSet=rs0&directConnection=true".to_string();
    let client: Client = Client::with_uri_str(&mongodb_uri).await.expect("Failed to connect to MongoDB");
    let tenant_id = generate_random_tenant_id();
    let db: Database = client.database(&tenant_id);

    match insert_data_to_mongodb(&data, &db).await {
        Ok(_) => println!("Data inserted into MongoDB successfully"),
        Err(e) => eprintln!("Error inserting data into MongoDB: {}", e),
    }
}
