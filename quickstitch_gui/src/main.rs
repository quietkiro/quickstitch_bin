use gui::icons::ICON_FONT;
use iced::{
    Size,
    window::{Settings, icon, settings::PlatformSpecific},
};

pub mod _gui;
mod gui;

static ICON: &[u8] = include_bytes!("../assets/quickstitch.ico");

fn main() -> iced::Result {
    let icon_image = image::load_from_memory(ICON).unwrap();
    iced::application(
        gui::Quickstitch::default,
        gui::Quickstitch::update,
        gui::Quickstitch::view,
    )
    .window(Settings {
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific {
            application_id: "quickstitch-gui".to_string(),
            ..Default::default()
        },
        size: Size {
            width: 600.0,
            height: 800.0,
        },
        icon: Some(icon::from_rgba(icon_image.as_bytes().to_vec(), 256, 256).unwrap()),
        ..Default::default()
    })
    .title("Quickstitch")
    .theme(gui::Quickstitch::get_theme)
    .font(ICON_FONT)
    .run()
}
