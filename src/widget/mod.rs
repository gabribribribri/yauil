use sfml::graphics::RenderWindow;

pub trait Widget {
    fn draw_widget(&mut self, window: &mut RenderWindow, y: f32);

    fn size(&self) -> f32 {
        100f32
    }
}

pub mod label;
