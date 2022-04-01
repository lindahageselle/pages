use aragog::*;
use moon::*;
// use shared::{DownMsg, UpMsg, User};
use aragog::query::QueryResult;
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
    return db_connection;
}

async fn aragog_getAllQueryResult(conn: &DatabaseConnection) -> QueryResult<test_collection> {
    let query = test_collection::query();
    let user_records = test_collection::get(query, conn).await.unwrap();
    user_records
}

async fn aragog_create(conn: &DatabaseConnection) {
    // Define a document
    let mut user = test_collection {
        username: String::from("linda"),
        password: String::from("password"),
    };

    let mut user_record = DatabaseRecord::create(user, conn).await.unwrap();
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    let connection = aragog_connect().await;

    // works, creates new entry
    // aragog_create(&connection).await;

    // works, gets all entries as a QueryResult
    let records = aragog_getAllQueryResult(&connection).await;
    println!("{:?}", records);

    start(frontend, up_msg_handler, |_| {}).await
}
