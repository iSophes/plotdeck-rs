use gpui::*;
use gpui_component::button::{Button, ButtonVariants};

pub fn playback_button(button_text: &str) -> impl IntoElement {
    Button::new(button_text.to_string())
        .primary()
        .label(button_text.to_owned())
        .on_click(|_, _, _| println!("Clicked button"))
        .h_full()
        .flex_shrink_0()
        .flex_1()
        .aspect_square()
}
