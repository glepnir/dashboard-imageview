use fltk::enums::Color;
use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window};
use std::env;
use std::process::Command;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Size {
    rows: i32,
    cols: i32,
    pixel_width: i32,
    pixel_height: i32,
    dpi: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WezData {
    window_id: i32,
    tab_id: i32,
    pane_id: i32,
    workspace: String,
    size: Size,
    title: String,
    cwd: String,
    cursor_x: i32,
    cursor_y: i32,
    cursor_shape: String,
    cursor_visibility: String,
    left_col: i32,
    top_row: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let image_width_pixel: i32 = args[2].trim().parse().unwrap();
    let image_height_pixel: i32 = args[3].trim().parse().unwrap();

    // wezterm data
    let child = Command::new("wezterm")
        .args(&["cli", "list", "--format", "json"])
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&child.stdout);
    let wez_data: Vec<WezData> = serde_json::from_str(&output).unwrap();

    // let cell_pixel_width = wez_data[0].size.pixel_width / wez_data[0].size.cols;
    let cell_pixel_height = wez_data[0].size.pixel_height / wez_data[0].size.rows;

    // center pos
    let mut image_pos_x = (wez_data[0].size.pixel_width >> 1) - (image_width_pixel >> 1);
    let image_pos_y = wez_data[0].cursor_y * cell_pixel_height;

    // tmux data
    let tmux = env::var("TERM_PROGRAM").unwrap_or("none".to_string());
    // check the cursor x pos when use some tools like tmux
    if wez_data[0].cursor_x > wez_data[0].size.cols >> 1 {
        image_pos_x = image_pos_x + (wez_data[0].size.pixel_width >> 2);
    }

    if tmux == "tmux" {
        let tmux_panes = Command::new("tmux")
            .args(&["list-panes"])
            .output()
            .expect("Failed to get tmux panes");
        let tmux_panes_data = String::from_utf8(tmux_panes.stdout).unwrap();
        let mut panes_count: Vec<&str> = tmux_panes_data.split("\n").collect();
        let mut length = panes_count.len();
        if panes_count[length -1].chars().count() == 0 {
            panes_count.remove(length - 1);
            length = length - 1
        }

        if wez_data[0].cursor_x < wez_data[0].size.cols >> 1 && length >= 2{
            image_pos_x = image_pos_x - (wez_data[0].size.pixel_width >> 2)
        }
    }

    // image
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = Window::default()
        .with_size(image_width_pixel, image_height_pixel)
        .with_pos(image_pos_x, image_pos_y);
    let mut frame = Frame::default().size_of(&win);

    let mut image = SharedImage::load(filename).unwrap();
    image.scale(image_width_pixel, image_height_pixel, true, true);

    frame.set_image(Some(image));
    win.make_resizable(false);
    win.set_color(Color::from_rgb(38,42,51));
    win.set_border(false);
    win.end();
    win.show();

    app.run().unwrap();
}
