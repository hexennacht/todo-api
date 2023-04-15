#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetTodoListRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub sort: Option<String>,
    pub order: Option<String>,
}