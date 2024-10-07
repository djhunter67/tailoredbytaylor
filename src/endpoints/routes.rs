use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use log::{error, info, warn};
use mongodb::{bson::doc, Client};

use crate::template::renderers::{
    Contact, ContentCreation, HomePage, Illustration, NotFoundTemplate, VoiceOver,
};

#[get("/")]
pub async fn home(mongo_client: web::Data<Client>) -> impl Responder {
    let db = mongo_client.database("test");
    let collection = db.collection("test");

    let doc = doc! {
        "title": "Tailored by Taylor!",
        "author": "Cousin Hunter, Christerpher",
    };

    let _ = collection.insert_one(doc).await.map_err(|e| {
        error!("Error inserting document: {}", e);
        HttpResponse::InternalServerError()
    });

    let template = HomePage {
        title: "Home | Tailored by Taylor",
        body: "WELCOME!",
    };

    let response_body = template.render().unwrap();

    info!("Index template rendered");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}

pub async fn not_found() -> impl Responder {
    warn!("404 Not Found");
    let not_found_template = NotFoundTemplate {
        title: "404",
        body: "Page not found!",
    };

    let response_body = not_found_template.render().unwrap();

    info!("404 Not Found template rendered");

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}

#[get("/contact")]
pub async fn contact() -> impl Responder {
    let template = Contact {
        title: "Contact Page Content!",
        body: "Contact Page body Content!",
    };

    let response_body = template.render().unwrap();

    info!("Contact template rendered");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}

#[get("/voice_over")]
pub async fn voice_over() -> impl Responder {
    let template = VoiceOver {
        title: "Voice Over Services",
        demos: vec![
            ("Booking.com - Comm. Demo", "4:12"),
            ("Florine... - Commercial", "2:22"),
            ("OFF! - Comm. Demo", "4:28"),
            ("Audible - Comm. Demo", "1:22"),
            ("Royal - Comm. Demo", "6:22"),
            ("BetterHelp - Comm. Demo", "8:12"),
            ("Expedia - Comm. Demo", "4:02"),
            ("Spirit - Comm. Demo", "4:29"),
            ("Carnival - Comm. Demo", "4:42"),
            ("Bose - Comm. Demo", "0:52"),
            ("Samsung - Comm. Demo", "5:02"),
            ("Pepsi - Comm. Demo", "5:29"),
            ("Coca-Cola - Comm. Demo", "3:32"),
            ("McDonald's - Comm. Demo", "7:52"),
            ("Burger King - Comm. Demo", "5:52"),
            ("Wendy's - Comm. Demo", "1:27"),
            ("Subway - Comm. Demo", "6:33"),
            ("Taco Bell - Comm. Demo", "4:44"),
            ("KFC - Comm. Demo", "2:22"),
        ],
    };

    let response_body = template.render().unwrap();

    info!("Voice Over template rendered");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}

#[get("/illustration")]
pub async fn illustration() -> impl Responder {
    let template = Illustration {
        title: "Illustration Page Content!",
        body: "Illustration Page body Content!",
    };

    let response_body = template.render().unwrap();

    info!("Illustration template rendered");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}

#[get("/content_creation")]
pub async fn content_creation() -> impl Responder {
    let template = ContentCreation {
        title: "Content Creation Page Content!",
        body: "Content Creation Page body Content!",
    };

    let response_body = template.render().unwrap();

    info!("Content Creation template rendered");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body)
}
