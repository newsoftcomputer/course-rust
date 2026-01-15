
use iced::{Widget::text, Element};

#[derive(Debug)]
struct AppState {
    
}

#[derive(Debug)]
enum Message {
    Exit,
}

fn update(state: &Appstate, message: Message) {

}

fn view(state: &AppState) -> Element<Message>  {
    text("Hello Iced").into()
}

fn main() {
    iced::apli
}
