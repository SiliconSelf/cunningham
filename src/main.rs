#![doc = include_str!("../README.md")]

use actors::{codematrix::CodeMatrix, ocr::{OcrActor, RecognizeCodeMatrix}};

use actix::prelude::*;

use crate::toolbox::LocalImage;

mod toolbox;
mod actors;

#[actix::main]
async fn main() {
    simple_logger::init().expect("Failed to initialize logging");

    let ocr_actor_addr = OcrActor::new().start();

    let path = "res/matrixtest.jpg";
    let code_matrix: Vec<Vec<CodeMatrix>> = ocr_actor_addr.send(RecognizeCodeMatrix { source: LocalImage { path } }).await.unwrap();

    println!("{code_matrix:#?}");

    std::thread::sleep(std::time::Duration::from_secs(5));
}
