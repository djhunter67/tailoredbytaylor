use std::collections::HashMap;

use actix_web::{post, web, HttpResponse, Responder};

use askama::Template;
use log::debug;

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
