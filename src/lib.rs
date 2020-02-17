use std::cmp;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
pub struct Stereo {
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
pub struct HotTub {
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

pub trait RemoteControllable: fmt::Debug {
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
pub struct RemoteControl<'a> {
    devices: HashMap<&'static str, &'a mut dyn RemoteControllable>,
}

impl<'a> RemoteControl<'a> {
    pub fn add_device<T: RemoteControllable + 'static>(&mut self, id: &'static str, device: &'a mut T) {
        self.devices.insert(id, device);
    }

    pub fn turn_on(&mut self, id: &str) {
        if let Some(ref mut device) = self.devices.get_mut(id) {
            device.on_button_pressed();
        }
    }

    pub fn turn_off(&mut self, id: &str) {
        if let Some(ref mut device) = self.devices.get_mut(id) {
            device.off_button_pressed();
        }
    }
    pub fn on<T: RemoteControllable + 'static>(device: &'a mut T) {
        device.on_button_pressed();
    }

    pub fn off<T: RemoteControllable + 'static>(device: &'a mut T) {
        device.off_button_pressed();
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remote() {
        let mut hot_tub = HotTub::default();
        let mut sound_blaster = Stereo::default();

        let mut control = RemoteControl::default();
        control.add_device("hot_tub", &mut hot_tub);
        control.add_device("sound_blaster", &mut sound_blaster);
        control.turn_on("hot_tub");
        control.turn_on("sound_blaster");

        assert_eq!(hot_tub.bubbles_on, true);
        assert_eq!(hot_tub.heat_on, true);
        assert_eq!(sound_blaster.volume, 1);

        RemoteControl::off(&mut hot_tub);
        assert_eq!(hot_tub.bubbles_on, false);

        RemoteControl::on(&mut hot_tub);
        assert_eq!(hot_tub.bubbles_on, true);

    }
}
