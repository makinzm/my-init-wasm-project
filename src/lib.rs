use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, length: f64, initial_angle_degrees: f64) {
    let num_points = 3; // 三角形
    let angle_increment_degrees = 120.0; // 各頂点間の角度差
    let initial_angle_radians = initial_angle_degrees * std::f64::consts::PI / 180.0; // 初期の角度をラジアンに変換

    context.begin_path();
    let mut current_angle_radians = initial_angle_radians;
    let mut previous_x = x + length * current_angle_radians.cos(); // 初期点の x 座標を計算
    let mut previous_y = y + length * current_angle_radians.sin(); // 初期点の y 座標を計算
    context.move_to(previous_x, previous_y);

    // 各頂点を描画
    for _ in 1..=num_points {
        current_angle_radians += angle_increment_degrees * std::f64::consts::PI / 180.0; // 次の点の角度を計算
        let next_x = previous_x + length * current_angle_radians.cos(); // 次の点の x 座標を計算
        let next_y = previous_y + length * current_angle_radians.sin(); // 次の点の y 座標を計算
        context.line_to(next_x, next_y);
        console::log_1(&JsValue::from_str(&format!("next_x: {}, next_y: {}", next_x, next_y)));
        previous_x = next_x;
        previous_y = next_y;
    }

    context.close_path();
    context.stroke();
}

fn draw_star(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, length: f64, initial_angle_degrees: f64) {
    let num_points: i32 = 10; // 10角形
    let initial_angle_radians = initial_angle_degrees * std::f64::consts::PI / 180.0; // 初期の角度をラジアンに変換

    context.begin_path();
    let mut current_angle_radians = initial_angle_radians;
    let mut previous_x = x + length * current_angle_radians.cos();
    let mut previous_y = y + length * current_angle_radians.sin();
    context.move_to(previous_x, previous_y);

    // 各頂点を描画
    for i in 1..=num_points {
        current_angle_radians += if i % 2 == 0 {
            (144.0 * std::f64::consts::PI / 180.0)
        } else {
            (288.0 * std::f64::consts::PI / 180.0)
        };
        let next_x = previous_x + length * current_angle_radians.cos();
        let next_y = previous_y + length * current_angle_radians.sin();
        context.line_to(next_x, next_y);
        console::log_1(&JsValue::from_str(&format!("next_x: {}, next_y: {}", next_x, next_y)));
        previous_x = next_x;
        previous_y = next_y;
    }

    context.close_path();
    context.stroke();
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Set up.
    let document = web_sys::window().unwrap().document().unwrap();

    // todo
    // Set up css.
    // let style = document.create_element("style")?;
    // style.set_inner_html(include_str!("../assets/css/style.css"));
    let canvas_triangle = document.get_element_by_id("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context_triangle = canvas_triangle.get_context("2d")?.unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    draw_triangle(&context_triangle, 150.0, 150.0, 100.0, 240.0);

    let canvas_star = document.get_element_by_id("canvas-star").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context_star = canvas_star.get_context("2d")?.unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    draw_star(&context_star, 150.0, 150.0, 100.0, 0.0);

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
