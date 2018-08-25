//! The map editor proper, with a GUI and everything.

extern crate glium;
#[macro_use] extern crate imgui;
extern crate imgui_glium_renderer;
extern crate dreammaker as dm;

mod support;

pub use glium::glutin;
use imgui::*;

const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

fn main() {
    support::run("SpacemanDMM".to_owned(), CLEAR_COLOR, |ui: &Ui| -> bool {
        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));
            });
        true
    });
}