use crate::application::{self, dto::Todo};

pub struct Repository {
    db: sqlx::PgPool,
}

impl Repository {
    pub fn new(db: sqlx::PgPool) -> Self {
        return Repository { db };
    }

    pub async fn get_todo_by_query(
        self,
        req: application::request::GetTodoListRequest,
    ) -> anyhow::Result<Vec<Todo>, anyhow::Error> {
        // TODO: revamp using seq-query https://github.com/SeaQL/sea-query

        let q = r#"
            SELECT * FROM todos
            WHERE name like '$1%'
            AND ($2::bool is NULL OR completed = $2)
            ORDER BY $3 desc
            LIMIT $5
            OFFSET $6
        "#;

        let todos = sqlx::query_as::<_, Todo>(q)
            .bind(req.get_name().clone())
            .bind(req.get_status())
            .bind(req.get_status())
            .bind(req.get_sort().clone())
            .bind(req.get_page_size().clone())
            .bind(req.get_offset().clone())
            .fetch_all(&self.db.clone())
            .await?;

        anyhow::Ok(todos)
    }
}
