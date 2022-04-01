use aragog::*;
use moon::*;
// use shared::{DownMsg, UpMsg, User};
use aragog::query::{Comparison, Filter, QueryResult};
use moon::actix_web::web::Data;
use serde::de::Unexpected::Str;
use std::borrow::Borrow;

async fn frontend() -> Frontend {
    Frontend::new().title("Pages example").append_to_head(
        "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }
        </style>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

// ------ ARAGOG STUFF ------

#[derive(Serialize, Deserialize, Clone, Record, Debug)]
#[serde(crate = "serde")]
pub struct test_collection {
    pub username: String,
    pub password: String,
}

async fn aragog_connect() -> DatabaseConnection {
    let db_connection = DatabaseConnection::builder()
        .with_credentials("http://localhost:8529", "DB_TEST", "root", "password")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    db_connection
}

async fn aragog_get_all_query_result(conn: &DatabaseConnection) -> QueryResult<test_collection> {
    let query = test_collection::query();
    let user_records = test_collection::get(query, conn).await.unwrap();
    user_records
}

async fn aragog_get_all(conn: &DatabaseConnection) -> Vec<DatabaseRecord<test_collection>> {
    let query = test_collection::query();
    let user_records = test_collection::get(query, conn).await.unwrap();
    let records: Vec<DatabaseRecord<test_collection>> = user_records.to_vec();
    records
}

async fn aragog_create_linda(conn: &DatabaseConnection) {
    // Define a document
    let mut user = test_collection {
        username: String::from("linda"),
        password: String::from("password"),
    };

    let mut user_record = DatabaseRecord::create(user, conn).await.unwrap();
}

async fn aragog_update_linda_to_anna(conn: &DatabaseConnection) {
    let query = test_collection::query().filter(Filter::new(
        Comparison::field("username").equals_str("linda"),
    ));
    let mut user_record = test_collection::get(query, conn)
        .await
        .unwrap()
        .uniq()
        .unwrap();
    user_record.username = String::from("Anna");
    user_record.save(conn).await.unwrap();
}

async fn aragog_delete_anna(conn: &DatabaseConnection) {
    let query = test_collection::query().filter(Filter::new(
        Comparison::field("username").equals_str("Anna"),
    ));
    let mut user_record = test_collection::get(query, conn)
        .await
        .unwrap()
        .uniq()
        .unwrap();
    user_record.delete(conn).await.unwrap();
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    let connection = aragog_connect().await;

    // Creates new entry with username "linda"
    // aragog_create_linda(&connection).await;

    // updates entry with username "linda" to username "Anna"
    // aragog_update_linda_to_anna(&connection).await;

    // Deletes entry with username "Anna"
    // aragog_delete_anna(&connection).await;

    // Gets all entries as a QueryResult
    let records = aragog_get_all_query_result(&connection).await;
    println!("{:?}", records);

    // works, gets all entries as object
    // first get the result as a vector from aragog_get_all
    // Make a vector that will hold the test_collection objects
    // for each Vec<DatabaseRecord<test_collection>>, print the result, then push the object user.record to the vector
    let result = aragog_get_all(&connection).await;
    let mut records: Vec<test_collection> = vec![];
    for user in &result {
        println!("{:?}", &user.record);
        records.push(user.record.clone());
    }

    // gets username of each object
    for user in &records {
        println!("Username: {}", user.username)
    }

    start(frontend, up_msg_handler, |_| {}).await
}
