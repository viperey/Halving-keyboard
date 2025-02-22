use crate::gamepad::GamepadMessage;
use crate::ui;
use crate::ui::message_bus::HalvingKeyboardMessage;
use crate::ui::style::KeyboardKeyStyle;
use iced::font::{Family, Stretch, Style, Weight};
use iced::widget::{text, Column, Container, Row, Text};
use iced::{
    executor, Alignment, Application, Color, Command, Element, Font, Length, Padding, Renderer,
    Subscription, Theme,
};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

type Flags = Arc<Mutex<mpsc::Receiver<GamepadMessage>>>;

pub struct KeyboardApp {
    selected_index: usize,
    text: String,
    gamepad_message: Option<GamepadMessage>,
    gamepad_rx: Arc<Mutex<mpsc::Receiver<GamepadMessage>>>,
    uppercase: bool,
    event_history: Vec<(GamepadMessage, Instant)>,
    halving_on: bool,
}

impl Application for KeyboardApp {
    type Executor = executor::Default;
    type Message = HalvingKeyboardMessage;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<HalvingKeyboardMessage>) {
        (
            KeyboardApp {
                selected_index: 12,
                text: String::new(),
                gamepad_message: None,
                gamepad_rx: flags,
                uppercase: false,
                event_history: Vec::new(),
                halving_on: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Experimental Keyboard with Gamepad")
    }

    fn update(&mut self, message: HalvingKeyboardMessage) -> Command<HalvingKeyboardMessage> {
        match message {
            HalvingKeyboardMessage::EventOccurred(_) => {}
            HalvingKeyboardMessage::Gamepad(msg) => {
                self.gamepad_message = Some(msg.clone());
                self.event_history.push((msg.clone(), Instant::now()));

                match msg {
                    GamepadMessage::OneLeft if self.selected_index > 0 => {
                        self.selected_index -= 1;
                    }
                    GamepadMessage::HalvingLeft if self.selected_index > 0 => {
                        self.selected_index = (self.selected_index as f32 / 2.0).floor() as usize;
                    }
                    GamepadMessage::OneRight if self.selected_index < 25 => {
                        self.selected_index += 1;
                    }
                    GamepadMessage::HalvingRight if self.selected_index < 25 => {
                        let jump = ((25 - self.selected_index) as f32 / 2.0).ceil() as usize;
                        self.selected_index += jump;
                        if self.selected_index > 25 {
                            self.selected_index = 25;
                        }
                    }
                    GamepadMessage::Enter => self.enter_letter(),
                    GamepadMessage::Space => self.enter_space(),
                    GamepadMessage::Delete => {
                        self.text.pop();
                    }
                    GamepadMessage::Uppercase => {
                        self.uppercase = true;
                    }
                    GamepadMessage::Lowercase => {
                        self.uppercase = false;
                    }
                    GamepadMessage::HalvingOn => {
                        self.halving_on = true;
                    }
                    GamepadMessage::HalvingOff => {
                        self.halving_on = false;
                    }
                    _ => {}
                }
            }
            HalvingKeyboardMessage::Tick => {
                let now = Instant::now();
                self.event_history
                    .retain(|&(_, timestamp)| (now - timestamp) <= Duration::from_secs(1));
                if self.event_history.len() > 5 {
                    self.event_history.drain(0..self.event_history.len() - 5);
                }
            }
        }

        let now = Instant::now();
        self.event_history
            .retain(|&(_, timestamp)| (now - timestamp) <= Duration::from_secs(1));
        if self.event_history.len() > 5 {
            self.event_history.drain(0..self.event_history.len() - 5);
        }
        Command::none()
    }

    fn view(&self) -> Element<HalvingKeyboardMessage> {
        let header_text: Text<Theme, Renderer> = Text::new("Halving Keyboard")
            .size(40)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        let header_container: Container<HalvingKeyboardMessage> = Container::new(header_text)
            .center_x()
            .width(Length::Fill)
            .padding(10);

        let central_text_text: Text<Theme, Renderer> =
            Text::new(&self.text).size(50).width(Length::Fill);
        let central_text_container = Container::new(central_text_text)
            .style(iced::theme::Container::Custom(Box::new(KeyboardKeyStyle {
                background: Color::WHITE,
                border_color: Color::BLACK,
                border_width: 2.0,
            })))
            .width(Length::Fill)
            .center_x();
        let central_text_parent_container = Container::new(central_text_container)
            .padding(20)
            .width(Length::Fill)
            .center_x();

        let tutorial_text: Text<Theme, Renderer> = Text::new(
            "\n\
                Halving: cut the distance to any ends of the keyboard by half.\n\
                \n\
                Actions:\n\
                 - Left/Right to navigate normally.\n\
                 - Hold R1 to enter halving mode. You'll see the candidate halving letters highlighted.\n\
                 - R1 + direction to navigate in halving mode.\n\
                 - Hold L1 for uppercase.\n\
                 - South: enter.\n\
                 - West: delete.\n\
                 - North: space.\n\
        ",
        )
        .size(40)
        .font(Font {
            family: Family::Monospace,
            weight: Weight::Semibold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        });
        let gamepad_messages: Vec<_> = self
            .event_history
            .iter()
            .map(|(msg, _timestamp)| {
                let text = Text::new(msg.icon())
                    .font(Font {
                        family: Family::Monospace,
                        weight: Weight::Semibold,
                        stretch: Stretch::Normal,
                        style: Style::Normal,
                    })
                    .size(20)
                    .shaping(text::Shaping::Advanced);
                text.into()
            })
            .collect();

        let gamepad_strokes_row = Row::with_children(gamepad_messages).spacing(18).height(30);
        let gamepad_strokes_row_container = Container::new(gamepad_strokes_row)
            .padding(Padding {
                top: 0.0,
                right: 20.0,
                bottom: 0.0,
                left: 20.0,
            })
            .width(Length::Fill)
            .center_y();

        let left_halve_position = self.get_left_jump_position();
        let right_halve_position = self.get_right_jump_position();
        let letters: Vec<_> = (0..26)
            .map(|i| {
                let letter = if self.uppercase {
                    (b'A' + i as u8) as char
                } else {
                    (b'a' + i as u8) as char
                };
                let bg = if i == self.selected_index {
                    Color::from_rgba8(255, 200, 0, 1.0)
                } else if self.halving_on && self.is_next_jump_position(i) {
                    Color::from_rgba8(255, 150, 0, 0.4)
                } else {
                    Color::WHITE
                };
                let letter_text = Text::new(letter.to_string()).size(30).font(Font {
                    family: Family::Monospace,
                    weight: Weight::Semibold,
                    stretch: Stretch::Normal,
                    style: Style::Normal,
                });
                Container::new(letter_text)
                    .width(60)
                    .height(60)
                    .center_x()
                    .center_y()
                    .style(iced::theme::Container::Custom(Box::new(KeyboardKeyStyle {
                        background: bg,
                        border_color: Color::BLACK,
                        border_width: 1.0,
                    })))
                    .into()
            })
            .collect();

        let keyboard_row = Row::with_children(letters)
            .spacing(5)
            .align_items(Alignment::Center);
        let keyboard_row_container = Container::new(keyboard_row).width(Length::Fill).center_x();

        let tutorial_text_container: Container<HalvingKeyboardMessage> =
            Container::new(tutorial_text).height(Length::Fill);
        let content = Column::with_children(vec![
            header_container.into(),
            tutorial_text_container.into(),
            central_text_parent_container.into(),
            gamepad_strokes_row_container.into(),
            keyboard_row_container.into(),
        ])
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Start);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(KeyboardKeyStyle {
                background: Color::from_rgb8(0xC2, 0xD6, 0xE9),
                border_color: Color::TRANSPARENT,
                border_width: 0.0,
            })))
            .into()
    }

    fn subscription(&self) -> Subscription<HalvingKeyboardMessage> {
        let keyboard_subscription: Subscription<HalvingKeyboardMessage> =
            iced::event::listen().map(HalvingKeyboardMessage::EventOccurred);
        Subscription::batch(vec![
            keyboard_subscription,
            ui::message_bus::gamepad_subscription(self.gamepad_rx.clone()),
            iced::time::every(Duration::from_millis(100)).map(|_| HalvingKeyboardMessage::Tick),
        ])
    }
}

impl KeyboardApp {
    fn enter_letter(&mut self) {
        let initial_letter: u8 = {
            if self.uppercase {
                b'A'
            } else {
                b'a'
            }
        };
        let letter: char = (initial_letter + self.selected_index as u8) as char;
        self.text.push(letter);
    }

    fn enter_space(&mut self) {
        self.text.push(' ');
    }

    fn is_next_jump_position(&self, position: usize) -> bool {
        if self.selected_index > position {
            self.get_left_jump_position() == position
        } else if self.selected_index < position {
            self.get_right_jump_position() == position
        } else {
            false
        }
    }

    fn get_left_jump_position(&self) -> usize {
        if self.selected_index > 0 {
            (self.selected_index as f32 / 2.0).floor() as usize
        } else {
            0
        }
    }

    fn get_right_jump_position(&self) -> usize {
        if self.selected_index < 25 {
            ((25 - self.selected_index) as f32 / 2.0).ceil() as usize + self.selected_index
        } else {
            25
        }
    }
}
