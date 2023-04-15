#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetTodoListRequest {
    pub name: Option<String>,
    pub status: Option<bool>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

impl GetTodoListRequest {
    pub fn get_name(&self) -> Option<String> {
        return self.name.clone();
    }

    pub fn get_status(&self) -> Option<bool> {
        return self.status;
    }

    pub fn get_page(&self) -> i32 {
        return self.page
            .clone()
            .unwrap_or(1_i32);
    }

    pub fn get_page_size(&self) -> i32 {
        return self.page_size
            .clone()
            .unwrap_or(10_i32);
    }

    pub fn get_sort(&self) -> String {
        return self.sort
            .clone()
            .unwrap_or("id".to_string());
    }

    pub fn get_order(&self) -> String {
        return self.order
            .clone()
            .unwrap_or("desc".to_string());
    }

    pub fn get_offset(&self) -> i32 {
        return (self.get_page() - 1) * self.get_page_size();
    }

    pub fn get_total_page(&self, total: i32) -> i32 {
        return total / self.get_page_size();
    }
}