use std::collections::HashMap;

use actix_web::{get, post, web, HttpResponse, Responder};

use askama::Template;
use log::debug;

use crate::template::renderers::DemoTimeShift;

#[derive(Template)]
#[template(path = "parts/play_demo.part.html")]
pub struct VoiceOverDemos<'a> {
    demo: &'a str,
}

#[post("/play_demo")]
pub async fn play_demo(track_to_play: web::Json<HashMap<String, String>>) -> impl Responder {
    debug!("Playing demo: {:#?}", track_to_play);
    let demo = track_to_play;

    let voice_over_demo = VoiceOverDemos {
        demo: demo.get("demo").unwrap(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(voice_over_demo.render().unwrap())
}

#[get("/demo_time_shift")]
pub async fn demo_time_shift() -> impl Responder {
    let template = DemoTimeShift {
        demo: (
            "Linked image of the share icon",
            "Linked image of a music note hamburger menu",
        ),
    };

    let response_body = template.render().unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}
