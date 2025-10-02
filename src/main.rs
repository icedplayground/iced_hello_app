// 🧊 iced_hello_app
// src/main.rs
use iced::widget::{center, text};
use iced::{Element, Result, Task};
use iced::application;

#[allow(non_camel_case_types)]
#[derive(Default)]
struct MY_ICED_HELLO_WORLD_STRUCT;

#[derive(Debug, Clone, Copy)]
enum Message {}

// fn main
fn main() -> Result {
    application(MY_ICED_HELLO_WORLD_STRUCT::title, MY_ICED_HELLO_WORLD_STRUCT::update, MY_ICED_HELLO_WORLD_STRUCT::view)
        .run_with(|| (MY_ICED_HELLO_WORLD_STRUCT::default(), Task::none()))
}

// impl
impl MY_ICED_HELLO_WORLD_STRUCT {
    
    // fn title
    fn title(&self) -> String {
        String::from("Iced • Hello")
    }

    // fn update
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    // fn view
    fn view(&self) -> Element<'_, Message> {
        center(text("Hello, world!")).into()
    }
}
