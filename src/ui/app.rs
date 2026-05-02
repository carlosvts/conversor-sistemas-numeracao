use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Alignment, Position};
use ratatui::style::Modifier;
use ratatui::widgets::Paragraph;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::Block,
};

enum Tabs {
    Conversor,
    Trace,
    Quiz,
    Batch,
    Max,
}

struct App {
    tab: Tabs,
}

impl App {
    fn new() -> Self {
        Self {
            tab: Tabs::Conversor,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal: DefaultTerminal = ratatui::init();
    let mut app = App::new();
    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => app.tab = Tabs::Conversor,
                KeyCode::Char('2') => app.tab = Tabs::Trace,
                KeyCode::Char('3') => app.tab = Tabs::Quiz,
                KeyCode::Char('4') => app.tab = Tabs::Batch,
                KeyCode::Char('5') => app.tab = Tabs::Max,
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    ratatui::restore();
    Ok(()) // "return 0"
}

fn draw(frame: &mut Frame, app: &App) {
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

    // menu em cima
    draw_tabs(frame, up_inner, app.tab);

    // aba do meio
    match app.tab {
        Tabs::Conversor => draw_conversor(
            frame,
            middle_block_left,
            middle_block_right,
            left_inner,
            right_inner,
        ),
        Tabs::Trace => draw_trace(
            frame,
            middle_block_left,
            middle_block_right,
            left_inner,
            right_inner,
        ),
        Tabs::Quiz => draw_quiz(
            frame,
            middle_block_left,
            middle_block_right,
            left_inner,
            right_inner,
        ),
        Tabs::Batch => draw_batch(
            frame,
            middle_block_left,
            middle_block_right,
            left_inner,
            right_inner,
        ),
        Tabs::Max => draw_max(
            frame,
            middle_block_right,
            middle_block_left,
            left_inner,
            right_inner,
        ),
        _ => {}
    }

    // menu de baixo
    draw_statusbar(frame, bottom_inner);
}

fn draw_tabs(frame: &mut Frame, area: Rect, active_tab: Tabs) {
    let tabs = vec![
        (1, "conversor"),
        (2, "trace"),
        (3, "quiz"),
        (4, "batch"),
        (5, "max"),
    ];
    let mut spans: Vec<Span> = Vec::new();
    let active_tab_int: i32 = active_tab as i32;
    for (num, mode) in tabs {
        let label = format!("[{}] {}", num, mode);
        let style;
        if num == active_tab_int {
            style = Style::default().add_modifier(Modifier::REVERSED);
        } else {
            style = Style::default().fg(Color::DarkGray);
        }
        spans.push(Span::styled(format!(" {} ", label), style));
        spans.push(Span::raw("  "));
    }

    let line = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(line, area);
}

fn draw_statusbar(frame: &mut Frame, bottom_inner: Rect) {
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

fn draw_conversor(
    frame: &mut Frame,
    middle_block_left: Block,
    middle_block_right: Block,
    left_inner: Rect,
    right_inner: Rect,
) {
    // display
    frame.render_widget(middle_block_left, left_inner);
    frame.render_widget(middle_block_right, right_inner);

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
}
