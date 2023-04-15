use sqlx::{Pool, Postgres, postgres};

pub async fn connect_to_postgres(database_url: String) -> Pool<Postgres> {
    let connection = postgres::PgPool::connect(&database_url).await;

    connection.expect("cannot connect to database")
}