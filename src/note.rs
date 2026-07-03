#[derive(serde::Serialize, serde::Deserialize)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub hidden: bool,
}

impl Note {
    pub fn new(id: u64, title: String, content: String) -> Self {
        Note {
            id,
            title,
            content,
            hidden: false,
        }
    }
}
