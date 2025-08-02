use gui::icons::ICON_FONT;

pub mod _gui;
mod gui;

fn main() -> iced::Result {
    iced::application(
        gui::Quickstitch::default,
        gui::Quickstitch::update,
        gui::Quickstitch::view,
    )
    .title("Quickstitch")
    .theme(gui::Quickstitch::get_theme)
    .window_size((600.0, 800.0))
    .font(ICON_FONT)
    .run()
}
