#[derive(Debug)]
pub struct Book{
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub translator: String,
    pub year_published: i32,
    pub year_translated: i32,
    pub publisher_id: i32,
}