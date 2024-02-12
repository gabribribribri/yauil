use {
    crate::widget::Widget,
    sfml::graphics::{RenderTarget, RenderWindow, Text, Transformable},
};

pub struct Label<'a> {
    pub text: Text<'a>,
}

impl<'a> Widget for Label<'a> {
    fn draw_widget(&mut self, window: &mut RenderWindow, y: f32) {
        self.text.set_position((10f32, y));
        window.draw(&self.text);
    }
}
