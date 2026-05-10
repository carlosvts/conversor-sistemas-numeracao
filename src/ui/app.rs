use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::Alignment;
use ratatui::style::Modifier;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::fs;

#[repr(i32)] // representavel como i32
#[derive(PartialEq, Eq, Clone, Copy)] // necessario para comparação com inteiros
enum Tabs {
    Conversor = 1,
    Trace = 2,
    Quiz = 3,
    Batch = 4,
    Max = 5,
}

// para incializar com zero
#[derive(Default)]
#[warn(dead_code)]
struct TraceState {
    valor: u64,
    base_origem: u8,
    base_destino: u8,
    etapa_atual: usize,
    passos: Vec<(u64, u64, u8)>,
    bits: Vec<u8>,
}

struct BatchState {
    selected_file: usize,
}

struct App {
    tab: Tabs,
    pub batch: BatchState,
}

impl App {
    fn new() -> Self {
        Self {
            tab: Tabs::Conversor,
            batch: BatchState { selected_file: 0 },
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
                // troca abas
                KeyCode::Char('1') => app.tab = Tabs::Conversor,
                KeyCode::Char('2') => app.tab = Tabs::Trace,
                KeyCode::Char('3') => app.tab = Tabs::Quiz,
                KeyCode::Char('4') => app.tab = Tabs::Batch,
                KeyCode::Char('5') => app.tab = Tabs::Max,
                KeyCode::Char('q') => break,

                // navegação batch
                KeyCode::Down => {
                    if app.tab == Tabs::Batch {
                        app.batch.selected_file += 1;
                    }
                }

                KeyCode::Up => {
                    if app.tab == Tabs::Batch {
                        if app.batch.selected_file > 0 {
                            app.batch.selected_file -= 1;
                        }
                    }
                }
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
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .spacing(Spacing::Overlap(1))
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    // blocos
    let up_block = Block::bordered()
        .title("   CONVERSOR UNIVERSAL DE BASES   ")
        .border_style(Style::default().fg(Color::LightRed))
        .merge_borders(MergeStrategy::Exact);

    let middle_block_left = Block::bordered()
        .title("  ENTRADA  ")
        .border_style(Style::default().fg(Color::LightYellow))
        .merge_borders(MergeStrategy::Exact);

    let middle_block_right = Block::bordered()
        .title("  SAIDA  ")
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

    frame.render_widget(&up_block, up_inner);
    frame.render_widget(&middle_block_left, inner_layout[0]);
    frame.render_widget(&middle_block_right, inner_layout[1]);

    // menu em cima
    draw_tabs(frame, up_inner, app.tab);

    // aba do meio
    match app.tab {
        Tabs::Conversor => draw_conversor(frame, left_inner, right_inner),
        Tabs::Trace => {
            let tracestate = TraceState::default();
            draw_trace(frame, left_inner, right_inner, &tracestate)
        }
        //Tabs::Quiz => draw_quiz(
        //    frame,
        //    middle_block_left,
        //    middle_block_right,
        //    left_inner,
        //    right_inner,
        //),
        Tabs::Batch => draw_batch(frame, left_inner, right_inner, app),
        //Tabs::Max => draw_max(
        //    frame,
        //    middle_block_right,
        //    middle_block_left,
        //    left_inner,
        //    right_inner,
        //),
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
    for (num, mode) in tabs {
        let label = format!("[{}] {}", num, mode);
        let style;
        // copia do enum
        if num == active_tab as i32 {
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
        Span::raw(" Carlos Vinícius Teixeira de Souza │  Introdução à Computação  │  João Vitor Pereira Gomes "),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightRed));
    frame.render_widget(info, bottom_inner);
}

fn draw_conversor(frame: &mut Frame, left_inner: Rect, right_inner: Rect) {
    // display
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
    let output = Paragraph::new("Output: 42");
    frame.render_widget(output, right_inner);
}

// TODO alterar para uma lógica de verdade e não hardcoded
fn draw_trace(frame: &mut Frame, left_inner: Rect, right_inner: Rect, tracestate: &TraceState) {
    let left = Paragraph::new(vec![
        Line::from(Span::styled(
            "42 ÷ 2 = 21  r 0",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "21 ÷ 2 = 10  r 1",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(vec![
            Span::styled("10 ÷ 2 =  5  r ", Style::default().fg(Color::DarkGray)),
            Span::styled("0", Style::default().fg(Color::LightMagenta)),
            Span::styled("  ←", Style::default().fg(Color::LightCyan)),
        ]),
        Line::from(Span::styled(
            " 5 ÷ 2 =  ?  r ?",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        )),
        Line::from(Span::styled(
            " 2 ÷ 2 =  ?  r ?",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        )),
        Line::from(Span::styled(
            " 1 ÷ 2 =  ?  r ?",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[←] anterior  [→] próxima  [r] reiniciar",
            Style::default().fg(Color::DarkGray),
        )),
    ]);

    let right = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("pos:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                " 5   4   3   2   1   0",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("      ", Style::default().fg(Color::DarkGray)),
            Span::styled("[1] ", Style::default().fg(Color::LightGreen)),
            Span::styled("[0] ", Style::default().fg(Color::DarkGray)),
            Span::styled("[1] ", Style::default().fg(Color::LightGreen)),
            Span::styled(
                "[?] ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled(
                "[?] ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled(
                "[?] ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "somatório posicional:",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(vec![
            Span::styled("1×2⁵", Style::default().fg(Color::LightGreen)),
            Span::styled(" + ", Style::default().fg(Color::DarkGray)),
            Span::styled("0×2⁴", Style::default().fg(Color::DarkGray)),
            Span::styled(" + ", Style::default().fg(Color::DarkGray)),
            Span::styled("1×2³", Style::default().fg(Color::LightGreen)),
            Span::styled(" + ...", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![Span::styled(
            "= 32 + 0 + 8 + ...",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
        Line::from(Span::styled(
            "etapa: 3 / 6",
            Style::default().fg(Color::DarkGray),
        )),
    ]);

    frame.render_widget(left, left_inner);
    frame.render_widget(right, right_inner);
}

fn draw_batch(frame: &mut Frame, left_inner: Rect, right_inner: Rect, app: &App) {
    // LEITURA DOS CSV
    let mut entries: Vec<String> = Vec::new();

    if let Ok(read_dir) = fs::read_dir("./data") {
        for entry in read_dir {
            if let Ok(entry) = entry {
                let path = entry.path();

                // aceita apenas arquivos .csv
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if ext == "csv" {
                            if let Some(name) = path.file_name() {
                                entries.push(name.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // seleção atual
    let mut selected_index: usize = app.batch.selected_file;

    // impede overflow
    if selected_index >= entries.len() && entries.len() > 0 {
        selected_index = entries.len() - 1;
    }

    // =========================
    // LISTA DE ARQUIVOS
    // =========================

    let mut items: Vec<ListItem> = Vec::new();

    for i in 0..entries.len() {
        let style;

        if i == selected_index {
            style = Style::default()
                .fg(Color::Black)
                .bg(Color::LightRed)
                .add_modifier(Modifier::BOLD);
        } else {
            style = Style::default().fg(Color::Gray);
        }

        let line = Line::from(vec![Span::styled(format!(" {}", entries[i]), style)]);

        items.push(ListItem::new(line));
    }

    let file_list = List::new(items).block(
        Block::default()
            .title(" Arquivos CSV ")
            .borders(Borders::NONE),
    );

    frame.render_widget(file_list, left_inner);

    // PREVIEW
    let mut preview_lines: Vec<Line> = Vec::new();

    preview_lines.push(Line::from("Arquivo selecionado:"));
    preview_lines.push(Line::from(""));
    preview_lines.push(Line::from(format!(
        "Quantidade de arquivos {}",
        entries.len()
    )));
    if entries.len() > 0 {
        preview_lines.push(Line::from(vec![Span::styled(
            &entries[selected_index],
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )]));

        preview_lines.push(Line::from(""));
        preview_lines.push(Line::from("[enter] converter lote"));
    } else {
        preview_lines.push(Line::from(vec![Span::styled(
            "    Nenhum arquivo CSV encontrado",
            Style::default().fg(Color::DarkGray),
        )]));
    }

    let preview = Paragraph::new(preview_lines);

    frame.render_widget(preview, right_inner);
}
