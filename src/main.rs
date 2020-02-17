use std::cmp;
use std::rc::Rc;
use std::fmt;

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
    devices: Vec<Rc<dyn RemoteControllable>>,
}

impl RemoteControl {
    fn add_device<T: RemoteControllable + 'static>(&mut self, device: Rc<T>) {
        self.devices.push(device);
    }
}

fn main() {
    println!("Hello, world!");
    let hot_tub = Rc::new(HotTub::default());
    let sound_blaster = Rc::new(Stereo::default());

    let mut control = RemoteControl::default();
    control.add_device(hot_tub.clone());
    control.add_device(sound_blaster.clone());

    dbg!(control);
}
