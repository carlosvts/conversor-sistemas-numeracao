use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Alignment;
use ratatui::style::Modifier;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::fs;

// ─── Tabs ────────────────────────────────────────────────────────────────────

#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tabs {
    Conversor = 1,
    Trace = 2,
    Quiz = 3,
    Batch = 4,
    Max = 5,
}

// ─── Base ────────────────────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Base {
    Auto,
    Bin,
    Oct,
    Dec,
    Hex,
}

impl Base {
    pub fn label(&self) -> &str {
        match self {
            Base::Auto => "AUTO",
            Base::Bin => "BIN",
            Base::Oct => "OCT",
            Base::Dec => "DEC",
            Base::Hex => "HEX",
        }
    }

    pub fn next(&self) -> Base {
        match self {
            Base::Auto => Base::Bin,
            Base::Bin => Base::Oct,
            Base::Oct => Base::Dec,
            Base::Dec => Base::Hex,
            Base::Hex => Base::Auto,
        }
    }

    pub fn prev(&self) -> Base {
        match self {
            Base::Auto => Base::Hex,
            Base::Bin => Base::Auto,
            Base::Oct => Base::Bin,
            Base::Dec => Base::Oct,
            Base::Hex => Base::Dec,
        }
    }

    // None = autodetect pelo prefixo no parser
    pub fn to_hint(&self) -> Option<u8> {
        match self {
            Base::Auto => None,
            Base::Bin => Some(2),
            Base::Oct => Some(8),
            Base::Dec => Some(10),
            Base::Hex => Some(16),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Base::Auto | Base::Dec => 10,
            Base::Bin => 2,
            Base::Oct => 8,
            Base::Hex => 16,
        }
    }

    // essa funcao vive enquanto Base existir
    pub fn all() -> &'static [Base] {
        &[Base::Auto, Base::Bin, Base::Oct, Base::Dec, Base::Hex]
    }
}

// ─── Foco do conversor ───────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConversorFocus {
    Input,
    SourceBase,
    TargetBase,
}

impl ConversorFocus {
    pub fn next(&self) -> ConversorFocus {
        match self {
            ConversorFocus::Input => ConversorFocus::SourceBase,
            ConversorFocus::SourceBase => ConversorFocus::TargetBase,
            ConversorFocus::TargetBase => ConversorFocus::Input,
        }
    }
}

// ─── Estados ─────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct TraceState {
    pub valor: u64,
    pub base_origem: u8,
    pub base_destino: u8,
    pub etapa_atual: usize,
    pub passos: Vec<(u64, u64, u8)>,
    pub bits: Vec<u8>,
}

pub struct BatchState {
    pub selected_file: usize,
}

pub struct ConversorState {
    pub input: String,
    pub source_base: Base,
    pub target_base: Base,
    pub output: String,
    pub error: Option<String>,
    pub focus: ConversorFocus,
}

impl ConversorState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            source_base: Base::Auto,
            target_base: Base::Bin,
            output: String::new(),
            error: None,
            focus: ConversorFocus::Input,
        }
    }
}

// ─── App ─────────────────────────────────────────────────────────────────────

pub struct App {
    pub tab: Tabs,
    pub batch: BatchState,
    pub conversor: ConversorState,
    pub trace: TraceState,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tabs::Conversor,
            batch: BatchState { selected_file: 0 },
            conversor: ConversorState::new(),
            trace: TraceState::default(),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            // troca abas — tem prioridade sobre tudo
            KeyCode::Char('1') => self.tab = Tabs::Conversor,
            KeyCode::Char('2') => self.tab = Tabs::Trace,
            KeyCode::Char('3') => self.tab = Tabs::Quiz,
            KeyCode::Char('4') => self.tab = Tabs::Batch,
            KeyCode::Char('5') => self.tab = Tabs::Max,

            // delega para o handler da aba ativa
            key => match self.tab {
                Tabs::Conversor => handle_conversor(&mut self.conversor, key),
                Tabs::Batch => handle_batch(&mut self.batch, key),
                _ => {}
            },
        }
    }
}

// ─── Handlers de input ───────────────────────────────────────────────────────

fn handle_conversor(state: &mut ConversorState, key: KeyCode) {
    match key {
        KeyCode::Tab => state.focus = state.focus.next(),

        KeyCode::Char(c) if state.focus == ConversorFocus::Input => {
            state.input.push(c);
        }

        KeyCode::Right if state.focus == ConversorFocus::SourceBase => {
            state.source_base = state.source_base.next();
        }
        KeyCode::Left if state.focus == ConversorFocus::SourceBase => {
            state.source_base = state.source_base.prev();
        }

        KeyCode::Right if state.focus == ConversorFocus::TargetBase => {
            state.target_base = state.target_base.next();
        }
        KeyCode::Left if state.focus == ConversorFocus::TargetBase => {
            state.target_base = state.target_base.prev();
        }

        KeyCode::Backspace if state.focus == ConversorFocus::Input => {
            state.input.pop();
        }

        // conversão — o main.rs chama o facade e escreve em state.output / state.error
        // por isso Enter só limpa o erro aqui; o resultado vem de fora
        KeyCode::Enter => {
            state.error = None;
        }

        _ => {}
    }
}

fn handle_batch(state: &mut BatchState, key: KeyCode) {
    match key {
        KeyCode::Down => state.selected_file += 1,
        KeyCode::Up => {
            if state.selected_file > 0 {
                state.selected_file -= 1;
            }
        }
        _ => {}
    }
}

// ─── Draw principal ──────────────────────────────────────────────────────────

pub fn draw(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .spacing(Spacing::Overlap(1))
        .split(frame.area());

    let inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[1]);

    let up_block = Block::bordered()
        .title("   CONVERSOR UNIVERSAL DE BASES   ")
        .border_style(Style::default().fg(Color::LightRed))
        .merge_borders(MergeStrategy::Exact);

    let left_block = Block::bordered()
        .title("  ENTRADA  ")
        .border_style(Style::default().fg(Color::LightYellow))
        .merge_borders(MergeStrategy::Exact);

    let right_block = Block::bordered()
        .title("  SAIDA  ")
        .border_style(Style::default().fg(Color::LightGreen))
        .merge_borders(MergeStrategy::Exact);

    let bottom_block = Block::bordered()
        .title("Info")
        .border_style(Style::default().fg(Color::LightBlue))
        .merge_borders(MergeStrategy::Exact);

    let up_inner = up_block.inner(outer[0]);
    let left_inner = left_block.inner(inner[0]);
    let right_inner = right_block.inner(inner[1]);
    let bottom_inner = bottom_block.inner(outer[2]);

    frame.render_widget(&up_block, outer[0]);
    frame.render_widget(&left_block, inner[0]);
    frame.render_widget(&right_block, inner[1]);

    draw_tabs(frame, up_inner, app.tab);

    match app.tab {
        Tabs::Conversor => draw_conversor(frame, left_inner, right_inner, &app.conversor),
        Tabs::Trace => draw_trace(frame, left_inner, right_inner, &app.trace),
        Tabs::Batch => draw_batch(frame, left_inner, right_inner, &app.batch),
        _ => {}
    }

    draw_statusbar(frame, bottom_inner);
}

// ─── Widgets ─────────────────────────────────────────────────────────────────

fn draw_tabs(frame: &mut Frame, area: Rect, active_tab: Tabs) {
    let tabs = [
        (1, "conversor"),
        (2, "trace"),
        (3, "quiz"),
        (4, "batch"),
        (5, "max"),
    ];

    let mut spans: Vec<Span> = Vec::new();
    for (num, label) in tabs {
        let style = if num == active_tab as i32 {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(format!(" [{}] {} ", num, label), style));
        spans.push(Span::raw("  "));
    }

    frame.render_widget(
        Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
        area,
    );
}

fn draw_statusbar(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new(" Carlos Vinícius Teixeira de Souza │  Introdução à Computação  │  João Vitor Pereira Gomes ")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightRed)),
        area,
    );
}

fn base_selector_spans<'a>(current: Base, focus: bool) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();
    for &base in Base::all() {
        let style = if base == current && focus {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .add_modifier(Modifier::BOLD)
        } else if base == current {
            Style::default().fg(Color::LightYellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(format!(" {} ", base.label()), style));
    }
    Line::from(spans)
}

fn draw_conversor(frame: &mut Frame, left: Rect, right: Rect, state: &ConversorState) {
    let cursor = if state.focus == ConversorFocus::Input {
        "█"
    } else {
        ""
    };

    let source_focused = state.focus == ConversorFocus::SourceBase;
    let target_focused = state.focus == ConversorFocus::TargetBase;

    let label_style = Style::default().fg(Color::DarkGray);
    let hint_style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::DIM);

    let input = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Valor : ", label_style),
            Span::styled(state.input.as_str(), Style::default().fg(Color::White)),
            Span::styled(cursor, Style::default().fg(Color::LightYellow)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("De    : ", label_style)]),
        base_selector_spans(state.source_base, source_focused),
        Line::from(""),
        Line::from(vec![Span::styled("Para  : ", label_style)]),
        base_selector_spans(state.target_base, target_focused),
        Line::from(""),
        Line::from(Span::styled(
            "[enter] converter",
            Style::default().fg(Color::LightGreen),
        )),
        Line::from(Span::styled("[tab] navegar  [← →] trocar base", hint_style)),
    ]);

    frame.render_widget(input, left);

    let output_lines = match &state.error {
        Some(err) => vec![
            Line::from(Span::styled("Erro:", Style::default().fg(Color::LightRed))),
            Line::from(""),
            Line::from(Span::styled(err.as_str(), Style::default().fg(Color::Red))),
        ],
        None => vec![
            Line::from(Span::styled(
                "Output:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                state.output.as_str(),
                Style::default().fg(Color::LightGreen),
            )),
        ],
    };

    frame.render_widget(Paragraph::new(output_lines), right);
}

fn draw_trace(frame: &mut Frame, left: Rect, right: Rect, state: &TraceState) {
    // ainda sem dados reais — renderiza vazio até o TraceState ser preenchido
    let left_content = if state.passos.is_empty() {
        Paragraph::new(Span::styled(
            "Nenhuma conversão em trace ainda.",
            Style::default().fg(Color::DarkGray),
        ))
    } else {
        // TODO: renderizar state.passos quando o trace for implementado
        Paragraph::new(Span::styled("...", Style::default().fg(Color::DarkGray)))
    };

    let right_content = if state.bits.is_empty() {
        Paragraph::new(Span::styled(
            "Execute uma conversão no modo trace.",
            Style::default().fg(Color::DarkGray),
        ))
    } else {
        Paragraph::new(Span::styled("...", Style::default().fg(Color::DarkGray)))
    };

    frame.render_widget(left_content, left);
    frame.render_widget(right_content, right);
}

fn draw_batch(frame: &mut Frame, left: Rect, right: Rect, state: &BatchState) {
    let mut entries: Vec<String> = Vec::new();

    if let Ok(dir) = fs::read_dir("./data") {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "csv" {
                        if let Some(name) = path.file_name() {
                            entries.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    let selected = if entries.is_empty() {
        0
    } else {
        state.selected_file.min(entries.len() - 1)
    };

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightRed)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(format!(" {}", name), style)))
        })
        .collect();

    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" Arquivos CSV ")
                .borders(Borders::NONE),
        ),
        left,
    );

    let mut preview: Vec<Line> = vec![
        Line::from("Arquivo selecionado:"),
        Line::from(""),
        Line::from(format!("Quantidade de arquivos: {}", entries.len())),
        Line::from(""),
    ];

    if entries.is_empty() {
        preview.push(Line::from(Span::styled(
            "Nenhum arquivo CSV encontrado",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        preview.push(Line::from(Span::styled(
            entries[selected].as_str(),
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )));
        preview.push(Line::from(""));
        preview.push(Line::from("[enter] converter lote"));
    }

    frame.render_widget(Paragraph::new(preview), right);
}

fn main() {
    println!("Use cargo run conversor-sistemas-numeracao");
}
