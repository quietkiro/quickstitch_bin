use iced::{
    widget::{button, column, row, text},
    Element, Theme,
};
mod simple_screen;
use simple_screen::SimpleScreen;

#[derive(Default)]
pub struct Quickstitch {
    theme: Theme,
    current_screen: Screen,
    // config: Config,
}

// #[derive(Default)]
// struct Config {
//     current_screen: Screen,
// }

#[derive(Default, PartialEq, Debug, Clone)]
pub enum Screen {
    #[default]
    Simple,
    Advanced,
    Config,
}

impl Screen {
    fn get_label(&self) -> String {
        match self {
            Self::Simple => "Simple".to_string(),
            Self::Advanced => "Advanced".to_string(),
            Self::Config => "Configuration".to_string(),
        }
    }
    // fn get_screen(&self) -> Element<Message> {
    //     match self {
    //         Screen::Simple => SimpleScreen.view(),
    //         Screen::Advanced => text!("test").into(),
    //         Screen::Config => text!("test").into(),
    //     }
    // }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
}

impl Quickstitch {
    pub fn view(&self) -> Element<Message> {
        let section_button = |screen: Screen, current_screen: &Screen| {
            button(text(screen.get_label()))
                .style(if screen == *current_screen {
                    button::primary
                } else {
                    button::text
                })
                .on_press(Message::ChangeScreen(screen))
                .padding(8)
        };

        column![
            row![
                section_button(Screen::Simple, &self.current_screen),
                section_button(Screen::Advanced, &self.current_screen),
                section_button(Screen::Config, &self.current_screen),
            ]
            .spacing(10),
            match self.current_screen {
                Screen::Simple => SimpleScreen.view(),
                Screen::Advanced => text!("test").into(),
                Screen::Config => text!("test").into(),
            }
        ]
        .padding(20)
        .into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeScreen(screen) => self.current_screen = screen,
        }
    }
    pub fn get_theme(&self) -> Theme {
        self.theme.clone()
    }
}
