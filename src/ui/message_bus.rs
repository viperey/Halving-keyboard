use crate::gamepad::GamepadMessage;
use iced::futures::stream;
use iced::futures::stream::BoxStream;
use iced::Event;
use iced_futures::core::Hasher;
use iced_futures::subscription::{EventStream, Recipe};
use iced_futures::Subscription;
use std::hash::Hash;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::yield_now;

#[derive(Debug, Clone)]
pub enum HalvingKeyboardMessage {
    EventOccurred(Event),
    Gamepad(GamepadMessage),
    Tick,
}

pub fn gamepad_subscription(
    rx: Arc<Mutex<mpsc::Receiver<GamepadMessage>>>,
) -> Subscription<HalvingKeyboardMessage> {
    Subscription::from_recipe(GamepadChannel { rx })
}

pub struct GamepadChannel {
    rx: Arc<Mutex<mpsc::Receiver<GamepadMessage>>>,
}

impl Recipe for GamepadChannel {
    type Output = HalvingKeyboardMessage;

    fn hash(&self, state: &mut Hasher) {
        "gamepad_channel".hash(state);
    }

    fn stream(self: Box<GamepadChannel>, _input: EventStream) -> BoxStream<'static, Self::Output> {
        Box::pin(stream::unfold(self.rx, |rx| async move {
            loop {
                if let Ok(msg) = rx.lock().unwrap().try_recv() {
                    println!("Received gamepad message: {:?}", msg);
                    return Some((HalvingKeyboardMessage::Gamepad(msg), rx.clone()));
                }
                yield_now();
            }
        }))
    }
}
