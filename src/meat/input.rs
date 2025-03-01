use three_d::renderer::control::{Event, Key};
use functor_derive::Functor;
use std::fmt::{ Display, Formatter };
use std::fmt;

#[derive(Debug)]
pub struct KeyHoldState {
    pub key: Key,
    pub pressed: bool,
}

impl KeyHoldState {
    pub fn new(key: Key) -> KeyHoldState {
        KeyHoldState {
            key,
            pressed: false,
        }
    }

    pub fn update(&mut self, event: &Event) -> () {
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

impl Display for KeyHoldState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.pressed {
            write!(f, "[{:?}]", self.key)
        } else {
            // I kinda want to make the space fit the whole name but I'm not using that rn so
            write!(f, "[ ]")
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

    pub fn hardup(self) -> bool {
        self.up > self.down
    }

    pub fn harddown(self) -> bool {
        self.up < self.down
    }

    pub fn hardleft(self) -> bool {
        self.left > self.right
    }

    pub fn hardright(self) -> bool {
        self.left < self.right
    }

}

impl Cardinals<KeyHoldState> {
    pub fn update(&mut self, event: &Event) -> () {
        self.up.update(event);
        self.down.update(event);
        self.left.update(event);
        self.right.update(event);
    }
}

impl <T: Display> Display for Cardinals<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.up, self.left, self.down, self.right)
    }
}

impl From<Cardinals<KeyHoldState>> for Cardinals<bool> {
    fn from(states: Cardinals<KeyHoldState>) -> Self {
        states.fmap(|state| state.pressed)
    }
}

impl From<&Cardinals<KeyHoldState>> for Cardinals<bool> {
    fn from(states: &Cardinals<KeyHoldState>) -> Self {
        Cardinals {
            up: states.up.pressed,
            down: states.down.pressed,
            left: states.left.pressed,
            right: states.right.pressed,
        }
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
