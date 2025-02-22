use gilrs::{Button, EventType, Gilrs};
use std::collections::HashSet;
use std::fmt;
use std::{sync::mpsc::Sender, thread, time::Duration};

#[derive(Debug, Clone, Copy)]
pub enum GamepadMessage {
    OneLeft,
    OneRight,
    HalvingOn,
    HalvingOff,
    HalvingLeft,
    HalvingRight,
    Enter,
    Delete,
    Lowercase,
    Uppercase,
    Space,
}

impl GamepadMessage {
    pub fn icon(&self) -> &str {
        match self {
            GamepadMessage::OneLeft => "←",
            GamepadMessage::OneRight => "→",
            GamepadMessage::HalvingOn => "½",
            GamepadMessage::HalvingOff => "1",
            GamepadMessage::HalvingLeft => "↶",
            GamepadMessage::HalvingRight => "↷",
            GamepadMessage::Enter => "⏎",
            GamepadMessage::Delete => "⌫",
            GamepadMessage::Lowercase => "a/z",
            GamepadMessage::Uppercase => "A/Z",
            GamepadMessage::Space => "␣",
        }
    }
}

impl fmt::Display for GamepadMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GamepadMessage::OneLeft => write!(f, "OneLeft"),
            GamepadMessage::OneRight => write!(f, "OneRight"),
            GamepadMessage::HalvingOn => write!(f, "HalvingOn"),
            GamepadMessage::HalvingOff => write!(f, "HalvingOff"),
            GamepadMessage::HalvingLeft => write!(f, "HalvingLeft"),
            GamepadMessage::HalvingRight => write!(f, "HalvingRight"),
            GamepadMessage::Enter => write!(f, "Enter"),
            GamepadMessage::Delete => write!(f, "Delete"),
            GamepadMessage::Lowercase => write!(f, "Lowercase"),
            GamepadMessage::Uppercase => write!(f, "Uppercase"),
            GamepadMessage::Space => write!(f, "Space"),
        }
    }
}

pub fn run_gamepad_loop(tx: Sender<GamepadMessage>) {
    thread::spawn(move || {
        let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs");
        let mut pressed_buttons = HashSet::new();
        let mut dpad_left_reported = false;
        let mut dpad_right_reported = false;
        let mut south_reported = false;
        let mut west_reported = false;
        let mut north_reported = false;
        let mut left_trigger_reported = false;
        let mut right_trigger_reported = false;

        println!("Gamepad thread started, listening for events...");

        loop {
            while let Some(event) = gilrs.next_event() {
                match event.event {
                    EventType::ButtonPressed(button, _) => {
                        pressed_buttons.insert(button);
                    }
                    EventType::ButtonReleased(button, _) => {
                        pressed_buttons.remove(&button);
                        match button {
                            Button::DPadLeft => dpad_left_reported = false,
                            Button::North => north_reported = false,
                            Button::DPadRight => dpad_right_reported = false,
                            Button::South => south_reported = false,
                            Button::West => west_reported = false,
                            Button::RightTrigger => {
                                let _ = tx.send(GamepadMessage::HalvingOff);
                                right_trigger_reported = false
                            }
                            Button::LeftTrigger => {
                                let _ = tx.send(GamepadMessage::Lowercase);
                                left_trigger_reported = false
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            if pressed_buttons.contains(&Button::DPadLeft) && !dpad_left_reported {
                if pressed_buttons.contains(&Button::RightTrigger) {
                    let _ = tx.send(GamepadMessage::HalvingLeft);
                } else {
                    let _ = tx.send(GamepadMessage::OneLeft);
                }
                dpad_left_reported = true;
            }

            if pressed_buttons.contains(&Button::DPadRight)
                && pressed_buttons.contains(&Button::RightTrigger)
                && !dpad_right_reported
            {
                let _ = tx.send(GamepadMessage::HalvingRight);
                dpad_right_reported = true;
                let _ = tx.send(GamepadMessage::HalvingOn);
                right_trigger_reported = true;
            }

            if pressed_buttons.contains(&Button::DPadRight)
                && !pressed_buttons.contains(&Button::RightTrigger)
                && !dpad_right_reported
            {
                let _ = tx.send(GamepadMessage::OneRight);
                dpad_right_reported = true;
            }

            if !pressed_buttons.contains(&Button::DPadRight)
                && pressed_buttons.contains(&Button::RightTrigger)
                && !right_trigger_reported
            {
                let _ = tx.send(GamepadMessage::HalvingOn);
                right_trigger_reported = true;
            }

            if pressed_buttons.contains(&Button::South) && !south_reported {
                let _ = tx.send(GamepadMessage::Enter);
                south_reported = true;
            }

            if pressed_buttons.contains(&Button::West) && !west_reported {
                let _ = tx.send(GamepadMessage::Delete);
                west_reported = true;
            }
            if pressed_buttons.contains(&Button::North) && !north_reported {
                let _ = tx.send(GamepadMessage::Space);
                north_reported = true;
            }

            if pressed_buttons.contains(&Button::LeftTrigger) && !left_trigger_reported {
                let _ = tx.send(GamepadMessage::Uppercase);
                left_trigger_reported = true;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
}
