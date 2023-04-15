use sqlx::{Pool, PgPool};

pub async fn connect_to_postgres(database_url: String) -> PgPool {
    let connection = Pool::connect(&database_url).await;

    connection.expect("cannot connect to database")
}