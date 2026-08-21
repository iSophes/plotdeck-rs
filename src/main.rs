// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;
use gpui_component::{button::*, *};

mod ui;
use crate::ui::components::playback_button::playback_button;

pub struct PlotDeck;

impl Render for PlotDeck {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .h_full()
                        .items_center()
                        .child(div().w(px(80.)).flex_shrink_0())
                        .child(div().flex_1().flex().justify_center().child("PlotDeck")),
                ),
            )
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .top_2()
                    .left_2()
                    .h_full()
                    .w_96()
                    .child(
                        Button::new("play")
                            .primary()
                            .label("PLAY")
                            .on_click(|_, _, _| println!("Clicked!"))
                            .h_32()
                            .w_full(),
                    )
                    .child(
                        div()
                            .h_flex()
                            .w_full()
                            .gap_1()
                            .h_24()
                            .justify_between()
                            .children([
                                playback_button("Previous"),
                                playback_button("Restart"),
                                playback_button("Next"),
                                playback_button("Reset All"),
                            ]),
                    ),
            )
    }
}

// credits to zed for this code

pub fn build_window_options() -> WindowOptions {
    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: None,
        focus: false,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        app_owns_titlebar_drag: true,
        window_decorations: Some(WindowDecorations::Client),
        window_min_size: Some(gpui::Size {
            width: px(360.0),
            height: px(240.0),
        }),
        ..Default::default()
    }
}

fn run_app() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(build_window_options(), |window, cx| {
                let view = cx.new(|_| PlotDeck);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}

fn main() {
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024) // 16MB, tune as needed
        .spawn(run_app)
        .unwrap()
        .join()
        .unwrap();
}
