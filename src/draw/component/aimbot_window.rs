use crate::{
    draw::{event::Event, frame::Frame},
    error::OxideResult,
    s,
    util::arcm::Arcm,
};

use super::{
    base::{checkbox::Checkbox, float_input::FloatInput, key_input::KeyInput, window::Window},
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct AimbotWindow {
    window: Window,
}

impl AimbotWindow {
    pub fn new(visible: Arcm<bool>) -> AimbotWindow {
        let mut window = Window::new("AIMBOT".to_owned(), visible);
        let mut y = 10;
        macro_rules! a {
            ($e:expr) => {
                window.add($e);
                #[allow(unused_assignments)]
                y += $e.get_base().h + 8
            };
        }

        a!(Checkbox::new("enable", s!().aimbot.enabled.clone(), 10, y));
        a!(Checkbox::new(
            "draw fov",
            s!().aimbot.draw_fov.clone(),
            10,
            y,
        ));
        a!(FloatInput::new(
            20,
            y,
            "aimbot fov",
            s!().aimbot.fov.clone(),
            None
        ));
        a!(KeyInput::new(10, y, "aimbot key", s!().aimbot.key.clone()));
        a!(Checkbox::new(
            "multipoint",
            s!().aimbot.multipoint.clone(),
            10,
            y
        ));
        a!(FloatInput::new(
            20,
            y,
            "hitbox scale",
            s!().aimbot.hitbox_scale.clone(),
            Some(|val| { val <= 1.0 && val >= 0.0 })
        ));
        a!(Checkbox::new(
            "autoshoot",
            s!().aimbot.autoshoot.clone(),
            10,
            y
        ));
        a!(Checkbox::new("silent", s!().aimbot.silent.clone(), 10, y));
        a!(Checkbox::new(
            "target sentries",
            s!().aimbot.target_sentries.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "target invisible",
            s!().aimbot.target_invisible.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "target disguised",
            s!().aimbot.target_disguised.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "target stickies",
            s!().aimbot.target_stickies.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "ambasador wait for hs",
            s!().aimbot.ambasador_wait_for_hs.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "wait for charge",
            s!().aimbot.wait_for_charge.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "baim if lethal",
            s!().aimbot.baim_if_lethal.clone(),
            10,
            y
        ));
        a!(Checkbox::new(
            "auto zoom",
            s!().aimbot.auto_zoom.clone(),
            10,
            y
        ));

        AimbotWindow { window }
    }
}

impl Component for AimbotWindow {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.draw(frame)
    }
    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
    fn get_draw_order(&self) -> super::DrawOrder {
        self.window.get_draw_order()
    }
    fn set_draw_order(&mut self, order: super::DrawOrder) {
        self.window.set_draw_order(order)
    }
    fn get_base(&mut self) -> &mut ComponentBase {
        self.window.get_base()
    }
}
