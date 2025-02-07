use std::{
    error::Error,
    io::Cursor,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use image::{ImageFormat, ImageReader};
use lazy_static::lazy_static;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams, TextItem};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref engine: Extracter = Extracter::new();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageFile {
    id: String,
    bytes: Vec<u8>,
}

#[tauri::command]
pub async fn extract(images: Vec<ImageFile>) -> ExtractResult {
    engine.extract_track_number(images)
}

struct Extracter {
    engine: OcrEngine,
}

impl Extracter {
    pub fn new() -> Self {
        let detection_model_path = PathBuf::from("./rtens/text-detection.rten");
        let rec_model_path = PathBuf::from("./rtens/text-recognition.rten");

        let detection_model = rten::Model::load_file(detection_model_path).unwrap();
        let recognition_model = rten::Model::load_file(rec_model_path).unwrap();

        Self {
            engine: OcrEngine::new(OcrEngineParams {
                detection_model: Some(detection_model),
                recognition_model: Some(recognition_model),
                ..Default::default()
            })
            .unwrap(),
        }
    }

    pub fn extract_track_number(&self, images: Vec<ImageFile>) -> ExtractResult {
        let extracted_list = Arc::new(Mutex::new(vec![]));
        let failed_list = Arc::new(Mutex::new(vec![]));

        images.into_par_iter().for_each(|image| {
            let extracted_list = Arc::clone(&extracted_list);
            let mut ex_list_lock = extracted_list.lock().unwrap();

            let failed_list = Arc::clone(&failed_list);
            let mut fa_list_lock = failed_list.lock().unwrap();

            let id = image.id.to_string();
            match self.extract_per_image(image) {
                Ok(Some(extracted_img)) => ex_list_lock.push(extracted_img),
                Err(e) => {
                    fa_list_lock.push(id);
                    println!("{:?}", e);
                }
                _ => fa_list_lock.push(id),
            }
        });

        let success = {
            let t = Arc::into_inner(extracted_list).unwrap();
            t.into_inner().unwrap()
        };
        let fail = {
            let t = Arc::into_inner(failed_list).unwrap();
            t.into_inner().unwrap()
        };
        ExtractResult { success, fail }
    }

    fn extract_per_image(&self, file: ImageFile) -> Result<Option<SuccessResp>, Box<dyn Error>> {
        let mut img = ImageReader::new(Cursor::new(&file.bytes));
        img.set_format(ImageFormat::Jpeg);
        let img = img.decode().map(|image| image.into_rgb8())?;

        let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
        let ocr_input = self.engine.prepare_input(img_source)?;

        let word_rects = self.engine.detect_words(&ocr_input)?;

        let line_rects = self.engine.find_text_lines(&ocr_input, &word_rects);

        let line_texts = self.engine.recognize_text(&ocr_input, &line_rects)?;

        const KEY_WORD: &str = "Sendungs";
        let mut check_next_line = false;

        for line in line_texts
            .iter()
            .flatten()
            .filter(|s| s.to_string().len() >= KEY_WORD.len())
        {
            let s = line.to_string();

            if check_next_line {
                if is_number(&s) {
                    return Ok(Some(SuccessResp {
                        id: file.id.to_string(),
                        track_number: s,
                        rect: line.words().next().unwrap().into(),
                    }));
                } else {
                    return Ok(None);
                }
            }

            if s.contains(KEY_WORD) {
                let splited: Vec<_> = s.split(':').filter(|word| !word.is_empty()).collect();
                if splited.len() != 2 {
                    check_next_line = true;
                    continue;
                }

                let rest = splited.get(1).unwrap().trim();

                if rest.len() != 14 {
                    return Ok(None);
                }

                if rest.is_empty() || !is_number(&rest) {
                    check_next_line = true;
                    continue;
                }

                if is_number(&rest) {
                    return Ok(Some(SuccessResp {
                        id: file.id.to_string(),
                        track_number: rest.to_string(),
                        rect: line.words().next().unwrap().into(),
                    }));
                }
            }
        }

        Ok(None)
    }

    // fn draw_return(line: &TextLine, img: DynamicImage) {}
}

pub type Id = String;

#[derive(Serialize, Deserialize)]
pub struct SuccessResp {
    pub id: Id,
    pub track_number: String,
    pub rect: Rectangle,
}

#[derive(Serialize, Deserialize)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl<'a> From<ocrs::TextWord<'a>> for Rectangle {
    fn from(value: ocrs::TextWord<'a>) -> Self {
        let rect = value.bounding_rect();

        Self {
            top_left: Point {
                x: rect.top_left().x as usize,
                y: rect.top_left().y as usize,
            },
            bottom_right: Point {
                x: rect.bottom_right().x as usize,
                y: rect.bottom_right().y as usize,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ExtractResult {
    pub success: Vec<SuccessResp>,
    pub fail: Vec<Id>,
}

fn is_number(text: &str) -> bool {
    if text.len() == 0 {
        false
    } else {
        !text.chars().any(|c| !char::is_numeric(c))
    }
}
