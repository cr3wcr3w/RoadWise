use iced::{
    Color, Element, Event, Fill, Task, event, theme,
    widget::{container, text},
    window,
};
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};

#[derive(Debug, Clone)]
enum Message {
    WindowOpened,
}

struct MyApp {
    tray: Option<TrayIcon>,
}

fn main() -> iced::Result {
    iced::application(MyApp::new, MyApp::update, MyApp::view)
        .subscription(|_| {
            let ui_events = event::listen_with(|event, _status, id| match event {
                Event::Window(window::Event::Opened { .. }) => {
                    let _ = id;
                    Some(Message::WindowOpened)
                }

                _ => None,
            });

            ui_events
        })
        .style(|_, _| theme::Style {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        })
        .window(window::Settings {
            decorations: false,
            transparent: false,
            level: window::Level::AlwaysOnTop,
            closeable: true,
            minimizable: true,
            maximized: true,
            resizable: true,
            exit_on_close_request: true,
            size: iced::Size::new(160.0, 50.0),
            position: window::Position::Centered,
            platform_specific: window::settings::PlatformSpecific {
                skip_taskbar: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}

impl MyApp {
    fn new() -> Self {
        Self { tray: None }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened => {
                if self.tray.is_none() {
                    if let Some(tray) = create_tray_icon() {
                        self.tray = Some(tray);
                    }
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            text("RoadWise")
                .size(24)
                .color(Color::from_rgb(0.0, 1.0, 0.0)),
        )
        .center(Fill)
        .into()
    }
}

fn create_tray_icon() -> Option<TrayIcon> {
    let icon = Icon::from_rgba(
        vec![
            0x22, 0x22, 0x22, 0xFF, // dark pixel
            0x22, 0x22, 0x22, 0xFF, // dark pixel
            0x22, 0x22, 0x22, 0xFF, // dark pixel
            0x00, 0xCC, 0x66, 0xFF, // green pixel
        ],
        2,
        2,
    )
    .ok()?;

    let tray_menu = Menu::new();
    let exit_item = MenuItem::new("Exit", true, None);
    tray_menu.append(&exit_item).ok()?;
    let exit_id = exit_item.id().clone();

    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        if event.id == exit_id {
            std::process::exit(0);
        }
    }));

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("RoadWise")
        .with_icon(icon)
        .build()
        .ok()?;

    Some(tray)
}
