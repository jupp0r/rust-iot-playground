struct Stereo {
    active: bool,
}

impl Stereo {
    fn volume_up(&mut self) {}
    fn volume_down(&mut self) {}
}

struct HotTub;

impl HotTub {
    fn enable_bubbles(&mut self) {}
    fn disable_bubbles(&mut self) {}
    fn set_temperature(&mut self) {}
    fn turn_off_heat(&mut self) {}
}

trait RemoteControllable {
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

fn main() {
    println!("Hello, world!");
}
