use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};

mod gamepad;
mod ui;

use crate::gamepad::GamepadMessage;
use crate::ui::app::KeyboardApp;
use iced::{Application, Settings};

fn main() {
    let (tx, rx) = mpsc::channel();
    gamepad::run_gamepad_loop(tx);

    let gamepad_rx = Arc::new(Mutex::new(rx));

    let settings: Settings<Arc<Mutex<Receiver<GamepadMessage>>>> = Settings::with_flags(gamepad_rx);
    let _ = KeyboardApp::run(settings);
}
