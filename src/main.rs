mod lib;
mod tests;
mod onnx;
mod image;
mod transfer;

use actix_web::middleware::Logger;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use serde::Deserialize;
use std::sync::Once;
use actix_web::rt::Runtime;
use actix_files::Files;
use actix_cors::Cors;
use std::mem::drop;
use actix_multipart::{Field, Multipart};
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use actix_web::web::Json;


extern crate log;

use log::{debug, error, log_enabled, info, Level};

use exitfailure::ExitFailure;
use std::thread;
use rust_bert::pipelines::common::ModelType;
use tch::Device;


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Deserialize)]
struct Info {
    context: String,
    minlength: i64,
    model: ModelType,
    is_gpu: bool,
}

#[derive(Deserialize)]
struct InfoAlbert {
    context: String,
}


#[get("/api/health")]
async fn api_health_handler() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


#[post("/api/albert")]
async fn api_albert(info: web::Json<InfoAlbert>) -> impl Responder {
    info!("request for albert");
    let output = onnx::abert_onnx(&info.context.to_owned()).unwrap();
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: output.to_string(),
    };

    info!("Response message: {}", response_json.message);

    HttpResponse::Ok().json(response_json)
}


#[post("/api/summary")]
async fn api_summary_handler(info: web::Json<Info>) -> impl Responder {
    let summarization_model = lib::init_summarization_model(info.model, info.minlength, info.is_gpu);
    info!("init model success");
    let this_device = Device::cuda_if_available();
    match this_device {
        Device::Cuda(_) => info!("Using GPU"),
        Device::Cpu => info!("Using CPU"),
        _ => {}
    }


    let mut input = [String::new(); 1];
    input[0] = info.context.to_owned();

    let _output = summarization_model.summarize(&input);
    let mut result = String::from(_output.join(" "));
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: result.to_string(),
    };

    info!("Response message: {}", response_json.message);

    HttpResponse::Ok().json(response_json)
}

async fn save_file(mut field: Field) -> Result<String, std::io::Error> {
    let mut upload_dir = PathBuf::from(std::env::current_dir().unwrap());
    upload_dir.push("upload");

    if !upload_dir.exists() {
        match fs::create_dir(&upload_dir) {
            Ok(_) => println!("Created directory: {:?}", upload_dir),
            Err(e) => panic!("Failed to create directory {:?}: {}", upload_dir, e),
        }
    }
    let mut file_name = None;
    let content_disposition = field.content_disposition();
    if let Some(name) = content_disposition.get_filename() {
        let upload_dir_string = upload_dir.to_string_lossy().to_string();

        file_name = Some(format!("{}/{}", upload_dir_string, name));
    }
    let file_path = file_name.unwrap();

    println!("{}", file_path);

    let mut file = std::fs::File::create(file_path.clone())?;

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        file.write_all(&data)?;
    }

    Ok(file_path)
}


#[derive(Serialize)]
struct FileResult {
    message: String,
}

#[post("/api/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    let mut results = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        match save_file(field).await {
            Ok(file_path) => {
                let result = image::label(file_path.clone()).expect("TODO: panic message");
                results.push(FileResult {
                    message: result.to_string(),
                });
            }
            Err(err) => {
                results.push(FileResult {
                    message: format!("Error: {}", err),
                });
            }
        }
    }

    if results.is_empty() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::Ok().json(Json(results.into_iter().next().unwrap()))
    }
}


#[post("/api/transfer/upload")]
async fn transfer_upload(mut payload: Multipart) -> impl Responder {
    let mut results = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        match save_file(field).await {
            Ok(file_path) => {
                let result = transfer::label_transfer(file_path.clone()).expect("TODO: panic message");
                results.push(FileResult {
                    message: result.to_string(),
                });
            }
            Err(err) => {
                results.push(FileResult {
                    message: format!("Error: {}", err),
                });
            }
        }
    }

    if results.is_empty() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::Ok().json(Json(results.into_iter().next().unwrap()))
    }
}

#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Server started successfully");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            // .wrap(cors) // Add the CORS middleware to the app
            .service(api_health_handler)
            .service(upload)
            .service(api_albert)
            .service(api_summary_handler)
            .service(transfer_upload)
            .service(Files::new("/", "./dist").index_file("index.html"))

            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;
    Ok(())
}