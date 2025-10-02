// 🧊 iced_hello_app
// src/main.rs
use iced::widget::{center, text};
use iced::{Element, Result, Task};
use iced::application;

#[derive(Default)]
struct MY_ICED_HELLO_WORLD;

#[derive(Debug, Clone, Copy)]
enum Message {}

fn main() -> Result {
    application(MY_ICED_HELLO_WORLD::title, MY_ICED_HELLO_WORLD::update, MY_ICED_HELLO_WORLD::view)
        .run_with(|| (MY_ICED_HELLO_WORLD::default(), Task::none()))
}

// Title: takes &MY_ICED_HELLO_WORLD
impl MY_ICED_HELLO_WORLD {
    fn title(&self) -> String {
        String::from("Iced • Hello")
    }

    // Update: takes &mut MY_ICED_HELLO_WORLD and a Message, returns Task<Message>
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    // View: takes &MY_ICED_HELLO_WORLD, returns Element<Message>
    fn view(&self) -> Element<'_, Message> {
        center(text("Hello, world!")).into()
    }
}
