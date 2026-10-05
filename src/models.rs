pub struct User {
    pub id: String,
    pub name: String,
}

pub struct ChatMessage {
    pub id: String,
    pub author: User,
    pub content: String,
}
