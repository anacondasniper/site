use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub month: u32,
    pub day: u32,
    pub year: i32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub day_count: u32,
}
