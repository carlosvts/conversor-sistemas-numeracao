use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::Block,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if key_pressed()? {
            return Ok(());
        }
    }
}

// sair da ui
fn key_pressed() -> Result<bool> {
    Ok(event::poll(Duration::from_millis(16))? && matches!(event::read()?, Event::Key(_)))
}

fn draw(frame: &mut Frame) {
    // espaçamentos
    // dois retangulos da direita
    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());

    // retangulo da esquerda
    let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(right);

    // blocos
    let left_block = Block::bordered()
        .title("Traceback")
        .border_style(Style::default().fg(Color::LightRed))
        .merge_borders(MergeStrategy::Exact);

    let top_right_block = Block::bordered()
        .title("Resultado")
        .border_style(Style::default().fg(Color::LightGreen))
        .merge_borders(MergeStrategy::Exact);

    let bottom_right_block = Block::bordered()
        .title("Input")
        .border_style(Style::default().fg(Color::LightBlue))
        .merge_borders(MergeStrategy::Exact);

    // display
    frame.render_widget(left_block, left);
    frame.render_widget(top_right_block, top_right);
    frame.render_widget(bottom_right_block, bottom_right);
}
