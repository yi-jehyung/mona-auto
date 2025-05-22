use image::imageops::crop_imm;
use imageproc::template_matching::{find_extremes, match_template_parallel, MatchTemplateMethod};

use crate::project::Roi;

pub fn match_template(
    image: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    template: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    roi: &Roi,
) -> (u32, u32, f32) {
    let roi_image = crop_imm(image, roi.x, roi.y, roi.width, roi.height).to_image();

    let result = match_template_parallel(&roi_image, template, MatchTemplateMethod::CrossCorrelationNormalized);
    let extremes = find_extremes(&result);
    let score = extremes.max_value;
    let (x, y) = extremes.max_value_location;
    
    //실제 위치 계산
    let center_x = roi.x + x + template.width() / 2;
    let center_y = roi.y + y + template.height() / 2;

    (center_x, center_y, score)
}