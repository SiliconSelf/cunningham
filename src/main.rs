#![doc = include_str!("../README.md")]

use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use rten_imageio::read_image;
use rten_tensor::prelude::*;

mod toolbox;

use toolbox::levenshtein_distance;

/// Symbols that can appear in the code matrix
#[derive(Debug)]
enum CodeMatrix {
    /// 1C
    _1C,
    /// 55
    _55,
    /// 7A
    _7A,
    /// BD
    _BD,
    /// E9
    _E9,
    /// FF
    _FF,
}

impl From<&str> for CodeMatrix {
    fn from(value: &str) -> Self {
        let distances: [usize; 6] = [
            levenshtein_distance("1C", value),
            levenshtein_distance("55", value),
            levenshtein_distance("7A", value),
            levenshtein_distance("BD", value),
            levenshtein_distance("E9", value),
            levenshtein_distance("FF", value),
        ];
        let mut least_distance: &usize = &distances[0];
        let mut index = 0;
        for (i, x) in distances.iter().enumerate() {
            if x < least_distance {
                least_distance = x;
                index = i;
            }
        }
        match index {
            0 => Self::_1C,
            1 => Self::_55,
            2 => Self::_7A,
            3 => Self::_BD,
            4 => Self::_E9,
            5 => Self::_FF,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let detection_model_data = include_bytes!("models/text-detection.rten");
    let rec_model_data = include_bytes!("models/text-recognition.rten");

    let detection_model = Model::load(detection_model_data)
        .expect("Couldn't create detection model");
    let recognition_model =
        Model::load(rec_model_data).expect("Couldn't create recognition model");

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })
    .expect("Couldn't create engine");

    let image =
        read_image("res/matrixtest.jpg").expect("Couldn't load screenshot");
    let ocr_input =
        engine.prepare_input(image.view()).expect("Couldn't prep OCR input");

    let word_rects =
        engine.detect_words(&ocr_input).expect("Couldn't detect words");
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);
    let line_texts = engine
        .recognize_text(&ocr_input, &line_rects)
        .expect("Failed to recognize text");

    let mut code_matrix: Vec<Vec<CodeMatrix>> = Vec::new();

    let lines = line_texts
        .iter()
        .flatten()
        .map(std::string::ToString::to_string)
        .filter(|l| l.len() > 1);
    for line in lines {
        let mut matrix_line = Vec::new();
        for symbol in line.split(' ') {
            let symbol = CodeMatrix::from(symbol);
            println!("Detected {symbol:?}");
            matrix_line.push(symbol);
        }
        code_matrix.push(matrix_line);
    }
    println!("{code_matrix:#?}");
}
