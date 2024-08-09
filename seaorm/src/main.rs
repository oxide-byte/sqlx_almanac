mod dao;

use futures::executor::block_on;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use dao::todo_dao::Entity as TodoDao;

async fn main_async() {
    let db: DatabaseConnection = Database::connect("postgres://postgis:postgis@localhost:5431/postgis").await.unwrap();

    let mut result = TodoDao::find()
        .all(&db)
        .await
        .expect("Error");

    println!("{:?}", result);

    db.close().await.expect("DB closing error");
}

fn main() {
    block_on(main_async());
}