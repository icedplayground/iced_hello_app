// 🧊 iced_hello_app
// src/main.rs
use iced::widget::{button, center, column, text};
use iced::{Element, Task};

// ============================== //

// The message defines any events or interactions that your program will care about.
#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

// struct - important for state
#[allow(non_camel_case_types)]
#[derive(Default)]
struct MY_ICED_HELLO_WORLD_STRUCT {
    button_text: String,
}

// fn main
#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    use iced::application;
    application(
        "👋 Iced • Hello",
        update,
        view,
    )
    .run()
}

#[cfg(target_arch = "wasm32")]
fn main() -> iced::Result {
    // For web target, Iced should work with the same application function
    iced::application(
        "👋 Iced • Hello",
        update,
        view,
    )
    .run()
}

// Update function
fn update(state: &mut MY_ICED_HELLO_WORLD_STRUCT, message: Message) -> Task<Message> {
    match message {
        Message::ButtonPressed => {
            state.button_text = "Button was pressed!".to_string();
            Task::none()
        }
    }
}

// View function
fn view(state: &MY_ICED_HELLO_WORLD_STRUCT) -> Element<'_, Message> {
    let current_text = if state.button_text.is_empty() {
        "Hello, world!"
    } else {
        &state.button_text
    };

    let button_widget = button(text("Click me!"))
        .on_press(Message::ButtonPressed);

    center(
        column![
            text(current_text),
            button_widget,
        ]
        .spacing(10)
    )
    .into()
}

// ==================================
// copyright 2025 by nonresistant.near
