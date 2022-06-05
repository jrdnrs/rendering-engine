#![allow(dead_code)]

use std::collections::HashMap;

use glutin::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode};

pub enum KeyAction {
    Held,
    Pressed,
    Released,
}

pub struct Input {
    pub key_states: HashMap<VirtualKeyCode, (KeyAction, ElementState)>,
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Input {
            key_states: HashMap::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn handle_input(&mut self, input: KeyboardInput) {
        if let KeyboardInput {
            virtual_keycode: Some(key),
            state: new_state,
            ..
        } = input
        {
            let new_key_action = if new_state == ElementState::Released {
                KeyAction::Released
            } else {
                match self.key_states.get(&key) {
                    Some((KeyAction::Held, ..)) => KeyAction::Held,
                    Some((KeyAction::Pressed, ..)) => KeyAction::Held,
                    Some((KeyAction::Released, ..)) => KeyAction::Pressed,
                    None => KeyAction::Pressed,
                }
            };

            self.key_states.insert(key, (new_key_action, new_state));
        }
    }

    pub fn is_key_held(&self, key: VirtualKeyCode) -> bool {
        if let Some((KeyAction::Held, ..)) = self.key_states.get(&key) {
            true
        } else {
            false
        }
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        if let Some((KeyAction::Pressed, ..)) = self.key_states.get(&key) {
            true
        } else {
            false
        }
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        if let Some((.., ElementState::Pressed)) = self.key_states.get(&key) {
            true
        } else {
            false
        }
    }
}

pub struct Mouse {
    pub delta_x: f64,
    pub delta_y: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub on_window: bool,
    pub moved: bool,
    pub grabbed: bool,
    pub button_states: HashMap<MouseButton, (KeyAction, ElementState)>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            delta_x: 0.0,
            delta_y: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
            on_window: false,
            moved: false,
            grabbed: false,
            button_states: HashMap::new(),
        }
    }

    pub fn handle_input(&mut self, button: MouseButton, new_state: ElementState) {
        let new_button_action = if new_state == ElementState::Released {
            KeyAction::Released
        } else {
            match self.button_states.get(&button) {
                Some((KeyAction::Held, ..)) => KeyAction::Held,
                Some((KeyAction::Pressed, ..)) => KeyAction::Held,
                Some((KeyAction::Released, ..)) => KeyAction::Pressed,
                None => KeyAction::Pressed,
            }
        };

        self.button_states
            .insert(button, (new_button_action, new_state));
    }

    pub fn is_button_held(&self, button: MouseButton) -> bool {
        if let Some((KeyAction::Held, ..)) = self.button_states.get(&button) {
            true
        } else {
            false
        }
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        if let Some((KeyAction::Pressed, ..)) = self.button_states.get(&button) {
            true
        } else {
            false
        }
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        if let Some((.., ElementState::Pressed)) = self.button_states.get(&button) {
            true
        } else {
            false
        }
    }
}
