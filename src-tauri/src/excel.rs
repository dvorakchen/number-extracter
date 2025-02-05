use rust_xlsxwriter::*;
use std::{error::Error, path::PathBuf};

pub struct ExtractedImg {
    pub id: String,
    pub bytes: Vec<u8>,
    pub track_number: String,
}

pub fn create_write_excel(list: Vec<ExtractedImg>, to: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut excel = Workbook::new();

    let sheet = excel
        .add_worksheet()
        .set_default_row_height(30)
        .set_column_width(1, 30)?;

    let center = Format::new()
        .set_align(FormatAlign::Center)
        .set_font_size(16)
        .set_bold();

    for (i, entry) in list.iter().enumerate() {
        let img = Image::new_from_buffer(&entry.bytes)?;
        // sheet.insert_image(i as u32, 0, &img)?;
        sheet.insert_image_fit_to_cell(i as u32, 0, &img, true)?;
        sheet.write_string_with_format(i as u32, 1, &entry.track_number, &center)?;
    }

    excel.save(to)?;
    Ok(())
}
