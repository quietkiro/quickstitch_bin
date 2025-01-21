use iced::{widget::text, Element};

pub struct SimpleScreen;

pub enum SimpleScreenMessage {}

impl SimpleScreen {
    pub fn view(&self) -> Element<SimpleScreenMessage> {
        text("hi").into()
    }
    pub fn update(&mut self) {}
}
