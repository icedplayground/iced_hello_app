// 🧊 iced_hello_app
// src/main.rs
use iced::application;
use iced::widget::{center, text};
use iced::{Element, Result, Task};

// ============================== //

// fn main
fn main() -> Result {
    application(
        MY_ICED_HELLO_WORLD_STRUCT::title,
        MY_ICED_HELLO_WORLD_STRUCT::update,
        MY_ICED_HELLO_WORLD_STRUCT::view,
    )
    .run_with(|| (MY_ICED_HELLO_WORLD_STRUCT::default(), Task::none()))
}

// ============================== //
// iced magic happens here

// struct - important for state
#[allow(non_camel_case_types)]
#[derive(Default)]
struct MY_ICED_HELLO_WORLD_STRUCT;

// enum
// The message defines any events or interactions that your program will care about.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
enum MY_ICED_HELLO_WORLD_MESSAGE_ENUM {}

// impl
impl MY_ICED_HELLO_WORLD_STRUCT {
    // fn title
    fn title(&self) -> String {
        String::from("👋 Iced • Hello")
    }

    // fn update
    fn update(&mut self, _message: MY_ICED_HELLO_WORLD_MESSAGE_ENUM) -> Task<MY_ICED_HELLO_WORLD_MESSAGE_ENUM> {
        Task::none()
    }

    // fn view
    fn view(&self) -> Element<'_, MY_ICED_HELLO_WORLD_MESSAGE_ENUM> {
        center(text("Hello, world!")).into()
    }
}

// ==================================
// copyright 2025 by nonresistant.near
