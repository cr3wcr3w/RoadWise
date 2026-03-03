use iced::widget::text;
use iced::{Element, Task};

type Message = ();

fn main() -> iced::Result {
    iced::application(MyApp::new, MyApp::update, MyApp::view)
        .window_size((800, 600))
        .run()
}

struct MyApp;

impl MyApp {
    fn new() -> Self {
        Self
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello World!").into()
    }
}
