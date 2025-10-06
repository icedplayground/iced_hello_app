// 🧊 {{project-name-kebab}}
// src/main.rs
use iced::application;
use iced::widget::{center, text};
use iced::{Element, Result, Task};

// ============================== //

// fn main
fn main() -> Result {
    application(
        {{project-struct-name}}::title,
        {{project-struct-name}}::update,
        {{project-struct-name}}::view,
    )
    .run_with(|| ({{project-struct-name}}::default(), Task::none()))
}

// ============================== //
// iced magic happens here

// struct - important for state
#[allow(non_camel_case_types)]
#[derive(Default)]
struct {{project-struct-name}};

// enum
// The message defines any events or interactions that your program will care about.
#[derive(Debug, Clone, Copy)]
enum Message {}

// impl
impl {{project-struct-name}} {
    // fn title
    fn title(&self) -> String {
        String::from("👋 Iced • {{project-display-name}}")
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

// ==================================
// copyright 2025 by {{project-author}}
