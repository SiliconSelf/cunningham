use actix::prelude::*;
use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use rten_tensor::AsView;

use crate::{actors::codematrix::CodeMatrix, toolbox::ocr::ImageSource};
use crate::actors::daemon::Daemon;

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
        let recognition_model_data =
            include_bytes!("../models/text-recognition.rten");
        let detection_model = Model::load(detection_model_data)
            .expect("Couldn't create detection model");
        let recognition_model = Model::load(recognition_model_data)
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
    /// Recognize text in a provided image source
    fn recognize_text<I: ImageSource>(&self, source: &I) -> Vec<String> {
        // Load the image into a 3D float array for processing
        let image = source.load();
        // Prepare the image for input
        let ocr_input = self
            .engine
            .prepare_input(image.view())
            .expect("Couldn't prep OCR input");
        // Search the image for text
        let word_rects = self
            .engine
            .detect_words(&ocr_input)
            .expect("Couldn't detect words");
        let line_rects = self.engine.find_text_lines(&ocr_input, &word_rects);
        let line_texts = self
            .engine
            .recognize_text(&ocr_input, &line_rects)
            .expect("Failed to recognize text");
        let lines: Vec<String> = line_texts
            .iter()
            .flatten()
            .map(std::string::ToString::to_string)
            .filter(|l| l.len() > 1)
            .collect();
        lines
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
        // Recognize the text in the image
        let lines = self.recognize_text(&msg.source);
        // Process the detected text to find the code matrix
        let mut code_matrix = Vec::new();
        let mut grid_size = 0;
        let mut first_push = false;
        for line in lines {
            let mut matrix_line = Vec::new();
            for symbol in line.split(' ') {
                if symbol.len() != 2 {
                    continue;
                }
                if let Ok(symbol) = CodeMatrix::try_from(symbol) {
                    matrix_line.push(symbol);
                }
            }
            if !first_push {
                first_push = true;
                grid_size = matrix_line.len();
            }
            if matrix_line.len() == grid_size {
                code_matrix.push(matrix_line);
            }
        }

        code_matrix
    }
}

/// A message to find the daemons in a screenshot
#[derive(Message)]
#[rtype(result = "Vec<Daemon>")]
pub(crate) struct RecognizeDaemons<I: ImageSource> {
    /// The image data source
    source: I
}

impl<I: ImageSource> Handler<RecognizeDaemons<I>> for OcrActor {
    type Result = Vec<Daemon>;
    fn handle(&mut self, msg: RecognizeDaemons<I>, _ctx: &mut Self::Context) -> Self::Result {
        log::trace!("OcrActor received RecognizeDaemons");
        let lines = self.recognize_text(&msg.source);
        todo!();
    }
}