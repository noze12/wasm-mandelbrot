use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use js_sys::{Function, Uint8Array};
use serde::{Serialize, Deserialize};

const MAX_ITERATION: u32 = 5000;

#[derive(Serialize, Deserialize)]
pub struct Range {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    dispaly_range_callback: &Function,
    max_iteration: Option<u32>,
) -> Result<(), JsValue> {
    let canvas = ctx.canvas().unwrap();
    let width = canvas.width();
    let height = canvas.height();
    let max_iteration = max_iteration.unwrap_or(MAX_ITERATION);
    let (x1, y1, x2, y2) = adjust_range(width, height, x1, y1, x2, y2);
    let display_range = Range { x1, y1, x2, y2 };
    dispaly_range_callback.call1(&JsValue::NULL, &JsValue::from_serde(&display_range).unwrap()).ok();

    let mut data = get_mandelbrot_set(width, height, x1, y1, x2, y2, max_iteration);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn adjust_range(
    width: u32,
    height: u32,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> (f64, f64, f64, f64) {
    let canvas_ratio = width as f64 / height as f64;
    let target_ratio = (x2 - x1) / (y2 - y1);
    if canvas_ratio < target_ratio {
        let m = (y2 + y1) / 2.0;
        let h = (x2 - x1) / canvas_ratio / 2.0; 
        return (x1, m - h, x2, m + h);
    } else {
        let m = (x2 + x1) / 2.0;
        let w = (y2 - y1) * canvas_ratio / 2.0;
        return (m - w, y1, m + w, y2);
    }
}

fn get_mandelbrot_set(
    width: u32,
    height: u32,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    max_iteration: u32,
) -> Vec<u8> {
    let mut data = Vec::new();
    let x_unit = (x2 - x1) / (width as f64);
    let y_unit = (y2 - y1) / (height as f64);
    for y in 0..height {
        for x in 0..width {
            let iter_index: u32 = calc_iter_index(x1 + (x as f64) * x_unit, y1 + (y as f64) * y_unit, max_iteration);
            let (r, g, b) = index_to_color(iter_index, max_iteration);
            data.push(r);
            data.push(g);
            data.push(b);
            data.push(255);
        }
    }
    return data;
}


#[wasm_bindgen]
pub fn calc(x: f64, y: f64, max_iteration: u32) -> Uint8Array {
    let i = calc_iter_index(x, y, max_iteration);
    let color = index_to_color(i, max_iteration);
    let array = [color.0, color.1, color.2];
    Uint8Array::from(&array[..])
}

fn calc_iter_index(x: f64, y: f64, max_iteration: u32) -> u32 {
    let mut iter_index: u32 = 0;
    let mut xn: f64 = 0.0;
    let mut yn: f64 = 0.0;
    while iter_index < max_iteration {
        let xt = xn;
        xn = xn * xn - yn * yn + x;
        yn = 2.0 * xt * yn + y;
        if (xn * xn + yn * yn) > 4.0 {
            break;
        }
        iter_index += 1;
    }
    return iter_index;
}

fn index_to_color(i: u32, max_iteration: u32) -> (u8, u8, u8) {
    if i >= max_iteration {
        return (0, 0, 0);
    } 
    let h = (360 * 4 - (360 * 4 * i) / max_iteration + 240 ) % 360;
    if h < 60 {
        return (255, (h as f64 / 60.0 * 255.0) as u8, 0);
    } else if h < 120 {
        return (((120 - h) as f64 / 60.0 * 255.0) as u8, 255, 0);
    } else if h < 180 {
        return (0, 1, ((h - 120) as f64 / 60.0 * 255.0) as u8)
    } else if h < 240 {
        return (0, ((240 - h) as f64 / 60.0 * 255.0) as u8, 255)
    } else if h < 300 {
        return (((h - 240) as f64 / 60.0 * 255.0) as u8, 0, 255)
    } else {
        return (255, 0, ((360 - h) as f64 / 60.0 * 255.0) as u8)
    }
}