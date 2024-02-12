use sfml::{graphics::Font, SfBox};

const FONT_INTER: SfBox<Font> = Font::from_file("fonts/Inter-Medium.ttf").unwrap();

#[cfg(test)]
mod duidle {

    use {
        crate::FONT_INTER,
        sfml::{
            graphics::{RenderWindow, Text},
            window::Style,
        },
        yauil::{engine::Engine, widget::label::Label, widget::Widget},
    };

    #[test]
    fn creating_window() {
        let mut window = RenderWindow::new(
            (600, 800),
            "The First Test",
            Style::CLOSE,
            &Default::default(),
        );
        let mut ye: Engine = Engine::new(&mut window);
        ye.loop_draw();
    }

    #[test]
    fn label_prototype() {
        let mut window = RenderWindow::new(
            (600, 800),
            "The First Test",
            Style::CLOSE,
            &Default::default(),
        );

        let mut label = Label {
            text: Text::new("Bonsoir, Ceci est un texte.", FONT_INTER, 15),
        };
    }
}
