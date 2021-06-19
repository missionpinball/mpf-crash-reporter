extern crate env_logger;
use actix_web::error;
use uuid::Uuid;
use std::{io::Write, fs};
use std::collections::HashMap;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::{Serialize, Deserialize};

async fn greet(_req: HttpRequest) -> impl Responder {
    format!("MPF Crash Reporter Service!")
}

async fn readiness(_req: HttpRequest) -> impl Responder {
    format!("Ready!")
}

async fn liveness(_req: HttpRequest) -> impl Responder {
    format!("Alive!")
}

#[derive(Deserialize, Serialize, Debug)]
struct CrashTrace {
    file: String,
    error_line_number: u32,
    module: String,
    error_code: String,
    module_line_number: u32,
    custom_inspection: HashMap<String, String>,
    source_code: String,
    local_variables: Vec<(String, String)>,
    object_variables: Vec<(String, String)>
}

#[derive(Deserialize, Serialize, Debug)]
struct CrashReport {
    error_no: Option<u32>,
    error_context: Option<String>,
    error_logger_name: Option<String>,
    timestamp: String,
    location: String,
    exception_type: String,
    trace: Vec<CrashTrace>,
    version: String,
}

async fn handle_crash_report(report: web::Json<CrashReport>) -> impl Responder {
    let report_uuid = Uuid::new_v4();
    let file_path_string = format!("/reports/{}", report_uuid.to_string());
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Err(error::ErrorInternalServerError(e)),
    };
    web::block::<_,_,()>(move || {
        let buffer = serde_json::to_string(&report.0).unwrap();
        file.write_all(buffer.as_bytes()).unwrap();
        Ok(())
    }).await.unwrap();

    Ok(format!("Thanks for your report! Report ID: {} Report URL: {}/{}",
    report_uuid.to_string(), "https://crashes.missionpinball.org/report", report_uuid.to_string()))
}

async fn show_crash_report(req: HttpRequest) -> impl Responder {
    let report_id_str = req.match_info().get("report_id").unwrap();
    let report_uuid = Uuid::parse_str(report_id_str).unwrap();
    format!("You should see report {} rendered here.", report_uuid.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(greet))
            .route("/liveness/", web::get().to(liveness))
            .route("/readiness/", web::get().to(readiness))
            .route("/report/{report_id}", web::get().to(show_crash_report))
            .route("/submit/", web::post().to(handle_crash_report))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
