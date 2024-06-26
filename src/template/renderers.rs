use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    // TODO: Add relevant fields here
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
