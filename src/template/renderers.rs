use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomePage<'a> {
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

#[derive(Template)]
#[template(path = "contact.html")]
pub struct Contact<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(path = "content_creation.html")]
pub struct ContentCreation<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(path = "illustration.html")]
pub struct Illustration<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(path = "voice_over.html")]
pub struct VoiceOver<'a> {
    pub title: &'a str,
    pub demos: Vec<(&'a str, &'a str)>,
}

#[derive(Template)]
#[template(path = "parts/demo_time_shift.part.html")]
pub struct DemoTimeShift<'a> {
    pub demo: (&'a str, &'a str),
}
