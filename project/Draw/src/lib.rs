pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // contains anything that implements Draw trait
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub height: u32,
    pub width: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //code
    } 
}
