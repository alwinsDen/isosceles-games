use color_eyre::{
    Result,
    eyre::{WrapErr, bail},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::{self},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};
use ratatui_textarea::TextArea;
use std::io;
mod constants;
use constants::CLI_ART;

#[derive(Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

//display implementation of the App struct
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("counter", &self.counter)
            .finish()
    }
}

impl App {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment(&mut self) -> Result<()> {
        self.counter += 1;

        /*
        a test condition to trigger failure if
        +ve counter is above 2
        */
        if self.counter > 2 {
            bail!("Counter overflowed!");
        }

        Ok(())
    }

    fn decrement(&mut self) -> Result<()> {
        self.counter -= 1;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.decrement()?,
            KeyCode::Right => self.increment()?,
            _ => {}
        }
        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling error event for -> {key_event:#?}")),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()
                .wrap_err("handle events failed.")
                .unwrap();
        }
        return Ok(());
    }

    /*
    this is for the debug metrics during the debug mode.
    */
    pub fn debug_metrics(&self, debug_area: Rect, buf: &mut Buffer, main_area: &Rect) {
        let test_area = Text::styled(
            format!(
                r#"
    debug area width: {}
    main terminal width: {}
        "#,
                debug_area.width, main_area.width
            ),
            Style::from(Color::Gray),
        );
        test_area.render(debug_area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let _title = Line::from(" Isosceles Games CLI ").bold();
        let instructions = Line::from(vec![
            //here into() automatically converts the stuff into whatever type is defined in the variable.
            " Quit ".into(),
            "<Q>".into(),
        ]);
        let block = Block::bordered()
            .title_bottom(instructions.centered())
            .border_set(border::EMPTY)
            .bg(Color::from_u32(0x000000));
        let inner = block.inner(area);
        block.render(area, buf);

        let [_debug_metrics_area, banner_area, _, textarea_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(6),
            Constraint::Fill(1),
        ])
        .areas(inner);

        let [_, textarea_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(80),
            Constraint::Fill(1),
        ])
        .areas(textarea_area);
        let banner_text = Text::styled(
            CLI_ART.trim_start_matches('\n'),
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
        );
        Paragraph::new(banner_text)
            .alignment(Alignment::Center)
            .render(banner_area, buf);
        let mut textarea = TextArea::from([""]);
        textarea.set_styled_placeholder(Text::styled(
            format!("Run anthing...search for runners."),
            Style::default().fg(Color::White),
        ));
        buf.set_style(textarea_area, Style::new().bg(Color::from_u32(0x4f4f4f)));
        let padded_area = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .inner(textarea_area);
        textarea.render(padded_area, buf);
        #[cfg(debug_assertions)]
        self.debug_metrics(_debug_metrics_area, buf, &area);
    }
}

fn main() -> io::Result<()> {
    let _ = color_eyre::install();
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

// _______test section___________
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into()).unwrap();
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert!(app.exit);
    }

    // panic tests
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.handle_key_event(KeyCode::Left.into()).unwrap();
    }

    #[test]
    fn handle_key_overflow() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        assert_eq!(
            app.handle_key_event(KeyCode::Right.into())
                .unwrap_err()
                .to_string(),
            "Counter overflowed!"
        )
    }
}
