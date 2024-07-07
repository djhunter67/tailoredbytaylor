use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use log::{error, info, warn};
use mongodb::{bson::doc, Client};

use crate::template::renderers::{IndexTemplate, NotFoundTemplate};

#[get("/")]
pub async fn index(mongo_client: web::Data<Client>) -> impl Responder {
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

    let template = IndexTemplate {
        title: "Home Page Content!",
        body: "Home Page body Content!",
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
