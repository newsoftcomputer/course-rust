
use iced::{Fill, Element};
use iced::widget::{column, container, row};

#[derive(Debug, Clone)]
enum Message {
    Value,
}

#[derive(Default)]
struct State {
    //value: u64,
}

fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(state: &mut State, message: Message) {
    // match message {
    //     text("Hola")
    //     //Message::Increment => counter.value += 1,
    // }
}

fn view(state: &State) -> Element<'_, Message> {
    container(
        column![
            "Top",
            row!["Left", "Right"].spacing(10),
            "Bottom"
        ]
        .spacing(10)
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
