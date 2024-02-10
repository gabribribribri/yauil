#[cfg(test)]
mod duidle {
    use {
        sfml::{graphics::RenderWindow, window::Style},
        yauil::*,
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
        ye.add_widget(Widget::Label {
            text: String::from("Bonsoir, ceci est un texte."),
        });
        ye.loop_draw();
    }
}
