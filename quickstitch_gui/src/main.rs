pub mod _gui;
mod gui;

fn main() -> iced::Result {
    iced::application(
        "Quickstitch",
        gui::Quickstitch::update,
        gui::Quickstitch::view,
    )
    .theme(gui::Quickstitch::get_theme)
    .run()
}
