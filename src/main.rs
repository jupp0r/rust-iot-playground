use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Stereo {
    volume: u16,
}

impl Stereo {
    fn volume_up(&mut self) {
        self.volume = cmp::min(self.volume + 1, 100);
    }
    fn volume_down(&mut self) {
        self.volume = cmp::max(self.volume - 1, 0);
    }
}

#[derive(Debug, Default)]
struct HotTub {
    bubbles_on: bool,
    heat_on: bool,
}

impl HotTub {
    fn enable_bubbles(&mut self) {
        self.bubbles_on = true
    }
    fn disable_bubbles(&mut self) {
        self.bubbles_on = false
    }
    fn set_temperature(&mut self) {
        self.heat_on = true
    }
    fn turn_off_heat(&mut self) {
        self.heat_on = false
    }
}

trait RemoteControllable: fmt::Debug {
    fn on_button_pressed(&mut self);
    fn off_button_pressed(&mut self);
}

impl RemoteControllable for Stereo {
    fn on_button_pressed(&mut self) {
        self.volume_up();
    }

    fn off_button_pressed(&mut self) {
        self.volume_down();
    }
}

impl RemoteControllable for HotTub {
    fn on_button_pressed(&mut self) {
        self.enable_bubbles();
        self.set_temperature();
    }

    fn off_button_pressed(&mut self) {
        self.turn_off_heat();
        self.disable_bubbles();
    }
}

#[derive(Debug, Default)]
struct RemoteControl {
    devices: HashMap<String, Rc<dyn RemoteControllable>>,
}

impl RemoteControl {
    pub fn add_device<T: RemoteControllable + 'static>(&mut self, id: &str, device: Rc<T>) {
        self.devices.insert(id.into(), device);
    }

    pub fn turn_on(&mut self, id: &str) {
        if let Some(ref mut device) = self.devices.get_mut(id.into()) {
            if let Some(ref mut d) = Rc::get_mut(device) {
                d.on_button_pressed();
            }
        }
    }

    pub fn turn_off(&mut self, id: &str) {
        if let Some(ref mut device) = self.devices.get_mut(id.into()) {
            if let Some(ref mut d) = Rc::get_mut(device) {
                d.off_button_pressed();
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remote() {
        let hot_tub = Rc::new(HotTub::default());
        let sound_blaster = Rc::new(Stereo::default());
        let mut control = RemoteControl::default();
        control.add_device("hot_tub", hot_tub.clone());
        control.add_device("sound_blaster", sound_blaster.clone());

        control.turn_on("hot_tub");
        control.turn_on("sound_blaster");

        assert_eq!(hot_tub.bubbles_on, true);
        assert_eq!(hot_tub.heat_on, true);
        assert_eq!(sound_blaster.volume, 1);
    }
}
