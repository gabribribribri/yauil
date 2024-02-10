use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::Event,
};

pub struct Engine<'a> {
    window: &'a mut RenderWindow,
    widgets: Vec<Widget>,
}

impl<'a> Engine<'a> {
    pub fn new(window: &'a mut RenderWindow) -> Engine {
        return Self {
            window,
            widgets: Vec::new(),
        };
    }

    pub fn loop_draw(&mut self) {
        loop {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => return,
                    _ => (),
                }
            }
            self.window.clear(Color::BLACK);
            self.window.display();
        }
    }
    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.push(widget)
    }
}

pub enum Widget {
    Label { text: String },
}
