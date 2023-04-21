use iced::widget::{button, column, text, text_input, row, container};
use iced::{Alignment, Element, Sandbox, Length, Settings};
use iced::window;

use ebcdict_converter::ebcdic_hex_to_ascii;

pub fn main() -> iced::Result {
    let window_settings = window::Settings{min_size: Some((600, 400)), ..Default::default()};
    Counter::run(Settings{window: window_settings, ..Default::default()})
}

struct Counter {
    value: String,
    copied_value: String
}

#[derive(Debug, Clone)]
enum Message {
    ValueEdited(String),
    TranslateHex,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { 
            value: "".to_string(),
            copied_value: "".to_string() 
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TranslateHex => {
                self.copied_value = ebcdic_hex_to_ascii(self.value.clone());
            }
            Message::ValueEdited(updated_str) => {
                self.value = updated_str;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Element<Message> = row![
            text_input("Input 1", &self.value)
                .on_input(Message::ValueEdited)
                .on_submit(Message::TranslateHex).width(200),

            button("Translate").on_press(Message::TranslateHex),

            text(&self.copied_value),
        ]
            .spacing(50)
            .padding(20)
            .align_items(Alignment::Center)
            .into();

        container(content)
            .width(Length::FillPortion(2))
            .height(Length::FillPortion(3))
            .center_x()
            .center_y()
            .max_width(800)
            .into()
    }
}