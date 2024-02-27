use actix::prelude::*;
use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use rten_tensor::AsView;

use crate::{actors::codematrix::CodeMatrix, toolbox::ocr::ImageSource};

/// An actor for all OCR requests
pub(crate) struct OcrActor {
    /// The engine used for OCR
    engine: OcrEngine,
}

impl Actor for OcrActor {
    type Context = Context<Self>;
}

impl OcrActor {
    /// Creates a new OCR Actor
    pub(crate) fn new() -> Self {
        let detection_model_data =
            include_bytes!("../models/text-detection.rten");
        let rec_model_data = include_bytes!("../models/text-recognition.rten");
        let detection_model = Model::load(detection_model_data)
            .expect("Couldn't create detection model");
        let recognition_model = Model::load(rec_model_data)
            .expect("Couldn't create recognition model");
        let engine = OcrEngine::new(OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(recognition_model),
            ..Default::default()
        })
        .expect("Couldn't create engine");
        Self {
            engine,
        }
    }
}

/// A request to
#[derive(Message, Debug)]
#[rtype(result = "Vec<Vec<CodeMatrix>>")]
pub(crate) struct RecognizeCodeMatrix<I: ImageSource> {
    /// The image data source
    pub(crate) source: I,
}

impl<I: ImageSource> Handler<RecognizeCodeMatrix<I>> for OcrActor {
    type Result = Vec<Vec<CodeMatrix>>;

    fn handle(
        &mut self,
        msg: RecognizeCodeMatrix<I>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::trace!("OcrActor received RecognizeCodeMatrix");
        let image = msg.source.load();

        let ocr_input = self
            .engine
            .prepare_input(image.view())
            .expect("Couldn't prep OCR input");
        let word_rects = self
            .engine
            .detect_words(&ocr_input)
            .expect("Couldn't detect words");
        let line_rects = self.engine.find_text_lines(&ocr_input, &word_rects);
        let line_texts = self
            .engine
            .recognize_text(&ocr_input, &line_rects)
            .expect("Failed to recognize text");
        let lines = line_texts
            .iter()
            .flatten()
            .map(std::string::ToString::to_string)
            .filter(|l| l.len() > 1);
        let mut code_matrix = Vec::new();
        for line in lines {
            let mut matrix_line = Vec::new();
            for symbol in line.split(' ') {
                let symbol = CodeMatrix::from(symbol);
                matrix_line.push(symbol);
            }
            code_matrix.push(matrix_line);
        }

        code_matrix
    }
}
