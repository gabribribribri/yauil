use {
    crate::widget::Widget,
    sfml::{
        graphics::{Color, RenderTarget, RenderWindow},
        window::Event,
    },
};

pub struct Engine<'a> {
    window: &'a mut RenderWindow,
    widgets: Vec<&'a mut dyn Widget>,
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
            let mut vertical_offset = 0f32;
            for widget in &mut self.widgets {
                widget.draw_widget(&mut self.window, vertical_offset);
            }
            self.window.display();
        }
    }
    pub fn add_widget(&mut self, widget: &'a mut dyn Widget) {
        self.widgets.push(widget)
    }
}
