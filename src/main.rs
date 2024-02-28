#![doc = include_str!("../README.md")]

use actix::prelude::*;
use actors::{
    codematrix::CodeMatrix, daemon::Daemon, ocr::{OcrActor, RecognizeCodeMatrix}
};

use crate::{actors::ocr::RecognizeDaemons, toolbox::ocr::LocalImage};

mod actors;
mod toolbox;

#[actix::main]
async fn main() {
    simple_logger::init().expect("Failed to initialize logging");

    let ocr_actor_addr = OcrActor::new().start();

    let path = "res/matrixtest_fullspace.jpg";
    let code_matrix: Vec<Vec<CodeMatrix>> = ocr_actor_addr
        .send(RecognizeCodeMatrix {
            source: LocalImage {
                path,
            },
        })
        .await
        .expect("Sending message to OCR actor failed");

    let path = "res/daemontest_fullspace.jpg";
    let daemons: Vec<Daemon> = ocr_actor_addr.send(RecognizeDaemons { source: LocalImage { path }}).await.expect("Sending message to OCR actor failed");
    println!("{code_matrix:?}");
    println!("{daemons:?}");
}
