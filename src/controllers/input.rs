use piston_window::{ControllerButton, ControllerAxisArgs, Key};
use super::time;
use std::f64;

#[derive(Default)]
pub struct InputController {
    actions: Actions,
    auto_mode: bool,
    auto_actions: AutoAction,
}

struct AutoAction {
    time: f64,
    rotate_time: f64,
    actions: Actions,
}

impl Default for AutoAction {
    fn default() -> Self {
        AutoAction {
            time: 0.0,
            rotate_time: time::ROTATE_SPEED / 2.0 / f64::consts::PI * 0.25,
            actions: Actions {
                shoot: true,
                ..Actions::default()
            },
        }
    }
}

/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool,
}

impl InputController {
    /// Create a new `InputController`
    pub fn new() -> InputController {
        InputController::default()
    }

    /// Returns a shared reference to the underlying actions
    pub fn actions(&self) -> &Actions {
        if self.auto_mode {
            &self.auto_actions.actions
        } else {
            &self.actions
        }
    }

    /// Update ai_action according delta time
    pub fn update(&mut self, dt: f64) {
        if self.auto_mode {
            //            self.ai_actions.boost = true;
            self.auto_actions.time += dt;

            if ((self.auto_actions.time / self.auto_actions.rotate_time) as i64) % 2 == 1 {
                self.auto_actions.actions.rotate_left = true;
                self.auto_actions.actions.rotate_right = false;
            } else {
                self.auto_actions.actions.rotate_left = false;
                self.auto_actions.actions.rotate_right = true;
            }
        }
    }

    /// Processes a key press
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        self.auto_mode = if key == Key::LCtrl { true } else { false };
        if self.auto_mode {
            return
        }
        match key {
            Key::Left => self.actions.rotate_left = pressed,
            Key::Right => self.actions.rotate_right = pressed,
            Key::Up => self.actions.boost = pressed,
            Key::Space => self.actions.shoot = pressed,
            _ => (),
        }
    }

    /// Processes a button press
    pub fn button_press(&mut self, controller: ControllerButton) {
        self.handle_button(controller, true);
    }

    /// Processes a button release
    pub fn button_release(&mut self, controller: ControllerButton) {
        self.handle_button(controller, false);
    }

    /// Handles a button press or release
    fn handle_button(&mut self, controller: ControllerButton, pressed: bool) {
        // Button 10 is A button on XInput
        if controller.button == 10 {
            self.actions.shoot = pressed
        }
    }

    /// Handles a controller axis input
    pub fn handle_axis(&mut self, controller: ControllerAxisArgs) {
        // Axis 0 is left stick (XInput). -1.0 left to 1.0 right
        if controller.axis == 0 {
            match controller.position {
                -1.0 ... -0.2 => {
                    self.actions.rotate_left = true;
                    self.actions.rotate_right = false;
                }
                0.2 ... 1.0 => {
                    self.actions.rotate_left = false;
                    self.actions.rotate_right = true;
                }
                -0.199 ... 0.199 => {
                    self.actions.rotate_left = false;
                    self.actions.rotate_right = false;
                }
                _ => {}
            }
        }

        // Axis 5 is right trigger (XInput). -1.0 is not pressed, 1.0 is fully pressed
        if controller.axis == 5 {
            match controller.position {
                -0.8 ... 1.0 => {
                    self.actions.boost = true;
                }
                -1.0 ... -0.799 => {
                    self.actions.boost = false;
                }
                _ => {}
            }
        }
    }
}
