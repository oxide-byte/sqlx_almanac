use chrono::NaiveDateTime;
use futures::executor::block_on;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow, Debug)]
struct TodoDAO {
    id: Option<i32>,
    name: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgis:postgis@localhost:5431/postgis").await?;
    Ok(pool)
}

async fn migrate(pool: &PgPool) {
    let _ = sqlx::migrate!()
        .run(pool)
        .await
        .unwrap();
}

async fn query_map(pool: &PgPool) {
    let todos: Vec<TodoDAO> = sqlx::query!(r#"SELECT * FROM todo"#)
        .fetch_all(pool)
        .await
        .unwrap()
        .iter()
        .map(|row| TodoDAO {
            id: Some(row.id),
            name: Some(row.name.clone()),
            created_at: Some(row.created_at),
            updated_at: Some(row.updated_at),
        })
        .collect();
    println!("todos: {:?}", todos);
}

async fn query_embedded(pool: &PgPool) {
    let todos: Vec<TodoDAO> = sqlx::query_as!(TodoDAO, r#"SELECT * FROM todo"#)
        .fetch_all(pool)
        .await
        .unwrap();
    println!("todos: {:?}", todos);
}

async fn query_file(pool: &PgPool) {
    let todos: Vec<TodoDAO> = sqlx::query_file_as!(TodoDAO, "sql/query.sql")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("todos: {:?}", todos);
}

async fn find_by_id(pool: &PgPool, id:i32) {
    let todo: TodoDAO = sqlx::query_file_as!(TodoDAO, "sql/find_by_id.sql",id)
        .fetch_one(pool)
        .await
        .unwrap();
    println!("todos: {:?}", todo);
}

async fn add_todo(pool: &PgPool, name: &str) {
    let _ = sqlx::query!(r#"INSERT INTO todo (name) VALUES ($1)"#, name)
        .execute(pool)
        .await
        .unwrap();
}

async fn main_async() {
    let pool = create_pool().await.unwrap();
    migrate(&pool).await;
    query_embedded(&pool).await;
    add_todo(&pool, "test").await;
    query_map(&pool).await;
    query_file(&pool).await;
    find_by_id(&pool, 1).await;
}

fn main() {
    block_on(main_async());
}