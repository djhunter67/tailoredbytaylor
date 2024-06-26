mod endpoints;
mod media;
mod payment;
mod template;

use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

use endpoints::routes;
use std::{fs::File, io, process};

use actix_web::{guard, http::KeepAlive, middleware, web, App, HttpServer};
use log::{debug, error, info};
use mongodb::Client;

/// The local IP address of the server.
const HOST_IP: &str = "0.0.0.0"; // Local connection
/// The port that the server will listen on.
const PORT: u16 = 8000;
// const API_RS: &str = "127.0.0.1:8090"; // Communication with the signal generator
/// The name of the database that will be used.

pub const MONGO_PATH: &str = "mongodb://10.20.30.20:27017/"; // Local connection

const DB_NAME: &str = "tbytvo"; // Local connection

#[actix_web::main]
async fn main() -> io::Result<()> {
    // This is a macro that allows for multiple loggers to be used at once
    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Always,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("tbytvo.log").unwrap(),
        ),
    ]) {
        Ok(()) => debug!("Logger initialized"),
        Err(err) => {
            error!("Error initializing logger: {err:#?}");
            process::exit(1);
        }
    }

    info!("establishing database path");
    let client = Client::with_uri_str(MONGO_PATH)
        .await
        .expect("Failed to connect to MongoDB");

    debug!(
        "establishing connection to database at port: {}",
        MONGO_PATH
            .split(':')
            .last()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .strip_suffix('/')
            .unwrap()
    );
    client
        .start_session()
        .await
        .expect("Failed to start session");

    info!("Starting server on port {}", PORT);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(DB_NAME))
            .service(
                actix_files::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(routes::index)
            .default_service(
                // Takes every not found to the 404 page and 404 response code
                web::route()
                    .guard(
                        guard::Any(guard::Patch())
                            .or(guard::Put())
                            .or(guard::Delete())
                            .or(guard::Get())
                            .or(guard::Post())
                            .or(guard::Head())
                            .or(guard::Options()),
                    )
                    .to(routes::not_found),
            )
    })
    .keep_alive(KeepAlive::Os) // Keep the connection alive; OS handled
    .bind((HOST_IP, PORT))
    .unwrap_or_else(|_| {
        error!("Error binding to port {}.", PORT);
        process::exit(1); // This is expected behavior if the port is already in use
    })
    .disable_signals() // Disable the signals to allow the OS to handle the signals
    .shutdown_timeout(13)
    .workers(2)
    .run()
    .await
}
