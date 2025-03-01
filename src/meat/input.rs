use three_d::renderer::control::{Event, Key};
use functor_derive::Functor;

struct KeyHoldState {
    pub key: Key,
    pub pressed: bool,
}

impl KeyHoldState {
    fn new(key: Key) -> KeyHoldState {
        KeyHoldState {
            key,
            pressed: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Functor)]
pub struct Cardinals<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
}

impl Cardinals<bool> {

    fn hardup(self) -> bool {
        self.up > self.down
    }

    fn harddown(self) -> bool {
        self.up < self.down
    }

    fn hardleft(self) -> bool {
        self.left > self.right
    }

    fn hardright(self) -> bool {
        self.left < self.right
    }

}

impl Cardinals<KeyHoldState> {
    fn update(&mut self, event: &Event) -> () {
        match event {
            Event::KeyPress {
                kind: key, ..
            } if *key == self.key => { self.pressed = true; },
            Event::KeyRelease {
                kind: key, ..
            } if *key == self.key => { self.pressed = true; },
            _ => {}
        }
    }
}

impl From<Cardinals<KeyHoldState>> for Cardinals<bool> {
    fn from(states: Cardinals<KeyHoldState>) -> Self {
        states.fmap(|state| state.pressed)
    }
}

pub fn was_pressed(event: &Event, key: Key) -> bool {
    match event {
        Event::KeyPress {
            kind: ekey, ..
        } => *ekey == key,
        _ => false
    }
}
