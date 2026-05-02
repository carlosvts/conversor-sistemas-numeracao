use std::time::Duration;

use color_eyre::Result;
use crossterm::cursor::position;
use ratatui::crossterm::event::{self, Event};
use ratatui::layout::{Alignment, Position};
use ratatui::widgets::Paragraph;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
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
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Fill(1),
            Constraint::Percentage(5),
        ])
        .spacing(Spacing::Overlap(1))
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    // blocos
    let up_block = Block::bordered()
        .title("CONVERSOR UNIVERSAL DE BASES")
        .border_style(Style::default().fg(Color::LightRed))
        .merge_borders(MergeStrategy::Exact);

    let middle_block_left = Block::bordered()
        .title("ENTRADA")
        .border_style(Style::default().fg(Color::LightYellow))
        .merge_borders(MergeStrategy::Exact);

    let middle_block_right = Block::bordered()
        .title("SAIDA")
        .border_style(Style::default().fg(Color::LightGreen))
        .merge_borders(MergeStrategy::Exact);

    let bottom_right_block = Block::bordered()
        .title("Info")
        .border_style(Style::default().fg(Color::LightBlue))
        .merge_borders(MergeStrategy::Exact);

    let up_inner = up_block.inner(outer_layout[0]);
    let left_inner = middle_block_left.inner(inner_layout[0]);
    let right_inner = middle_block_right.inner(inner_layout[1]);
    let bottom_inner = bottom_right_block.inner(outer_layout[2]);

    // display
    frame.render_widget(up_block, outer_layout[0]);
    frame.render_widget(middle_block_left, inner_layout[0]);
    frame.render_widget(middle_block_right, inner_layout[1]);
    frame.render_widget(bottom_right_block, outer_layout[2]);

    let title = Paragraph::new(Line::from(vec![Span::raw(
        "[1] conversor    [2] trace    [3] quiz    [4] batch    [5] máximos",
    )]))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(title, up_inner);

    // input
    let input = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Valor : ", Style::default().fg(Color::DarkGray)),
            Span::styled("42", Style::default().fg(Color::White)),
            Span::styled("█", Style::default().fg(Color::LightYellow)), // cursor
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("De    : ", Style::default().fg(Color::DarkGray)),
            Span::styled("[DEC]", Style::default().fg(Color::LightYellow)),
        ]),
        Line::from(vec![
            Span::styled("Para  : ", Style::default().fg(Color::DarkGray)),
            Span::styled("[BIN]", Style::default().fg(Color::LightYellow)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "[enter] converter",
            Style::default().fg(Color::LightGreen),
        )),
        Line::from(Span::styled(
            "[s] passo a passo",
            Style::default().fg(Color::DarkGray),
        )),
    ]);
    frame.render_widget(input, left_inner);
    // output
    // TODO

    // rodapé
    let info = Paragraph::new(Line::from(vec![
        Span::styled("status: ", Style::default().fg(Color::DarkGray)),
        Span::styled("OK", Style::default().fg(Color::LightGreen)),
        Span::raw("  │  modo: conversor  │  histórico: 3"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(info, bottom_inner);
}
