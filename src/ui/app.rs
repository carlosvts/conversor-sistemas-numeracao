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
    Custom(u8),
}

impl Base {
    pub fn label(&self) -> &str {
        match self {
            Base::Auto => "AUTO",
            Base::Bin => "BIN",
            Base::Oct => "OCT",
            Base::Dec => "DEC",
            Base::Hex => "HEX",
            Base::Custom(_) => "CUSTOM",
        }
    }

    pub fn next(&self) -> Base {
        match self {
            Base::Auto => Base::Bin,
            Base::Bin => Base::Oct,
            Base::Oct => Base::Dec,
            Base::Dec => Base::Hex,
            Base::Hex => Base::Custom(10),
            Base::Custom(_) => Base::Auto,
        }
    }

    pub fn prev(&self) -> Base {
        match self {
            Base::Auto => Base::Custom(10),
            Base::Bin => Base::Auto,
            Base::Oct => Base::Bin,
            Base::Dec => Base::Oct,
            Base::Hex => Base::Dec,
            Base::Custom(_) => Base::Hex,
        }
    }

    pub fn to_hint(&self) -> Option<u8> {
        match self {
            Base::Auto => None,
            Base::Bin => Some(2),
            Base::Oct => Some(8),
            Base::Dec => Some(10),
            Base::Hex => Some(16),
            Base::Custom(n) => Some(*n),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Base::Auto | Base::Dec => 10,
            Base::Bin => 2,
            Base::Oct => 8,
            Base::Hex => 16,
            Base::Custom(n) => *n,
        }
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Base::Custom(_))
    }

    pub fn all_static() -> &'static [Base] {
        &[Base::Auto, Base::Bin, Base::Oct, Base::Dec, Base::Hex]
    }
}

// ─── Foco do conversor ───────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConversorFocus {
    Input,
    SourceBase,
    SourceCustom,
    TargetBase,
    TargetCustom,
}

impl ConversorFocus {
    pub fn next(&self, source_is_custom: bool, target_is_custom: bool) -> ConversorFocus {
        match self {
            ConversorFocus::Input => ConversorFocus::SourceBase,
            ConversorFocus::SourceBase => {
                if source_is_custom {
                    ConversorFocus::SourceCustom
                } else {
                    ConversorFocus::TargetBase
                }
            }
            ConversorFocus::SourceCustom => ConversorFocus::TargetBase,
            ConversorFocus::TargetBase => {
                if target_is_custom {
                    ConversorFocus::TargetCustom
                } else {
                    ConversorFocus::Input
                }
            }
            ConversorFocus::TargetCustom => ConversorFocus::Input,
        }
    }

    pub fn is_typing(&self) -> bool {
        matches!(
            self,
            ConversorFocus::Input | ConversorFocus::SourceCustom | ConversorFocus::TargetCustom
        )
    }
}

// ─── Estados ─────────────────────────────────────────────────────────────────

/// Estado da aba Trace.
/// Preenchido pelo main.rs após uma conversão com generate_trace = true.
pub struct TraceState {
    /// Valor original digitado pelo usuário
    pub valor_original: String,
    /// Base de origem (como número)
    pub base_origem: u8,
    /// Base de destino (como número)
    pub base_destino: u8,
    /// Resultado final da conversão
    pub resultado: String,
    /// Linhas de trace geradas pelo processor (ex: "3 x 10^1 = 30", "10 / 2 = 5  r 0")
    pub passos: Vec<String>,
    /// Índice do passo destacado atualmente na navegação
    pub passo_atual: usize,
}

impl Default for TraceState {
    fn default() -> Self {
        Self {
            valor_original: String::new(),
            base_origem: 10,
            base_destino: 2,
            resultado: String::new(),
            passos: Vec::new(),
            passo_atual: 0,
        }
    }
}

impl TraceState {
    pub fn tem_dados(&self) -> bool {
        !self.passos.is_empty()
    }

    pub fn avancar(&mut self) {
        if !self.passos.is_empty() && self.passo_atual + 1 < self.passos.len() {
            self.passo_atual += 1;
        }
    }

    pub fn recuar(&mut self) {
        if self.passo_atual > 0 {
            self.passo_atual -= 1;
        }
    }
}

pub struct BatchState {
    pub selected_file: usize,
}

pub struct ConversorState {
    pub input: String,
    pub source_base: Base,
    pub source_custom_input: String,
    pub target_base: Base,
    pub target_custom_input: String,
    pub output: String,
    pub error: Option<String>,
    pub focus: ConversorFocus,
}

impl ConversorState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            source_base: Base::Auto,
            source_custom_input: String::new(),
            target_base: Base::Bin,
            target_custom_input: String::new(),
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
        let typing = match self.tab {
            Tabs::Conversor => self.conversor.focus.is_typing(),
            _ => false,
        };

        if !typing {
            match key {
                KeyCode::Char('1') => {
                    self.tab = Tabs::Conversor;
                    return;
                }
                KeyCode::Char('2') => {
                    self.tab = Tabs::Trace;
                    return;
                }
                KeyCode::Char('3') => {
                    self.tab = Tabs::Quiz;
                    return;
                }
                KeyCode::Char('4') => {
                    self.tab = Tabs::Batch;
                    return;
                }
                KeyCode::Char('5') => {
                    self.tab = Tabs::Max;
                    return;
                }
                _ => {}
            }
        }

        match self.tab {
            Tabs::Conversor => handle_conversor(&mut self.conversor, key),
            Tabs::Trace => handle_trace(&mut self.trace, key),
            Tabs::Batch => handle_batch(&mut self.batch, key),
            _ => {}
        }
    }
}

// ─── Helpers de base customizada ─────────────────────────────────────────────

fn parse_custom_base(buf: &str) -> Option<u8> {
    buf.parse::<u8>().ok().filter(|&n| (2..=36).contains(&n))
}

fn commit_custom_base(base: &mut Base, buf: &str) {
    if let Some(n) = parse_custom_base(buf) {
        *base = Base::Custom(n);
    }
}

// ─── Handlers de input ───────────────────────────────────────────────────────

fn handle_conversor(state: &mut ConversorState, key: KeyCode) {
    let source_custom = state.source_base.is_custom();
    let target_custom = state.target_base.is_custom();

    match key {
        KeyCode::Tab => {
            match state.focus {
                ConversorFocus::SourceCustom => {
                    let buf = state.source_custom_input.clone();
                    commit_custom_base(&mut state.source_base, &buf);
                }
                ConversorFocus::TargetCustom => {
                    let buf = state.target_custom_input.clone();
                    commit_custom_base(&mut state.target_base, &buf);
                }
                _ => {}
            }
            state.focus = state
                .focus
                .next(state.source_base.is_custom(), state.target_base.is_custom());
        }

        KeyCode::Esc => match state.focus {
            ConversorFocus::SourceCustom => state.focus = ConversorFocus::SourceBase,
            ConversorFocus::TargetCustom => state.focus = ConversorFocus::TargetBase,
            _ => {}
        },

        KeyCode::Char(c) if state.focus == ConversorFocus::Input => {
            state.input.push(c);
        }
        KeyCode::Backspace if state.focus == ConversorFocus::Input => {
            state.input.pop();
        }

        KeyCode::Right if state.focus == ConversorFocus::SourceBase => {
            state.source_base = state.source_base.next();
            if state.source_base.is_custom() {
                state.source_custom_input = String::new();
            }
        }
        KeyCode::Left if state.focus == ConversorFocus::SourceBase => {
            state.source_base = state.source_base.prev();
            if state.source_base.is_custom() {
                state.source_custom_input = String::new();
            }
        }
        KeyCode::Enter if state.focus == ConversorFocus::SourceBase && source_custom => {
            state.focus = ConversorFocus::SourceCustom;
        }

        KeyCode::Char(c) if state.focus == ConversorFocus::SourceCustom => {
            if c.is_ascii_digit() && state.source_custom_input.len() < 2 {
                state.source_custom_input.push(c);
                commit_custom_base(&mut state.source_base, &state.source_custom_input.clone());
            }
        }
        KeyCode::Backspace if state.focus == ConversorFocus::SourceCustom => {
            state.source_custom_input.pop();
        }

        KeyCode::Right if state.focus == ConversorFocus::TargetBase => {
            state.target_base = state.target_base.next();
            if state.target_base.is_custom() {
                state.target_custom_input = String::new();
            }
        }
        KeyCode::Left if state.focus == ConversorFocus::TargetBase => {
            state.target_base = state.target_base.prev();
            if state.target_base.is_custom() {
                state.target_custom_input = String::new();
            }
        }
        KeyCode::Enter if state.focus == ConversorFocus::TargetBase && target_custom => {
            state.focus = ConversorFocus::TargetCustom;
        }

        KeyCode::Char(c) if state.focus == ConversorFocus::TargetCustom => {
            if c.is_ascii_digit() && state.target_custom_input.len() < 2 {
                state.target_custom_input.push(c);
                commit_custom_base(&mut state.target_base, &state.target_custom_input.clone());
            }
        }
        KeyCode::Backspace if state.focus == ConversorFocus::TargetCustom => {
            state.target_custom_input.pop();
        }

        KeyCode::Enter => {
            state.error = None;
        }

        _ => {}
    }
}

fn handle_trace(state: &mut TraceState, key: KeyCode) {
    match key {
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => state.avancar(),
        KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => state.recuar(),
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
        .border_style(Style::default().fg(Color::LightBlue))
        .merge_borders(MergeStrategy::Exact);

    let left_block = Block::bordered()
        .title("  ENTRADA  ")
        .border_style(Style::default().fg(Color::LightBlue))
        .merge_borders(MergeStrategy::Exact);

    let right_block = Block::bordered()
        .title("  SAIDA  ")
        .border_style(Style::default().fg(Color::LightBlue))
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

    frame.render_widget(&bottom_block, outer[2]);
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
        Paragraph::new(
            " Carlos Vinícius Teixeira de Souza │  Introdução à Computação  │  João Vitor Pereira Gomes ",
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::LightBlue)),
        area,
    );
}

fn base_selector_spans<'a>(current: Base, focused: bool, _custom_buf: &str) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();

    for &base in Base::all_static() {
        let active = base == current;
        let style = if active && focused {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)
        } else if active {
            Style::default().fg(Color::LightBlue)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(format!(" {} ", base.label()), style));
    }

    let custom_active = current.is_custom();
    let custom_btn_style = if custom_active && focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightMagenta)
            .add_modifier(Modifier::BOLD)
    } else if custom_active {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let custom_label = if custom_active {
        let n = if let Base::Custom(n) = current { n } else { 0 };
        if n >= 2 {
            format!(" CUSTOM({}) ", n)
        } else {
            " CUSTOM(?) ".to_string()
        }
    } else {
        " CUSTOM ".to_string()
    };
    spans.push(Span::styled(custom_label, custom_btn_style));

    Line::from(spans)
}

fn custom_base_input_line<'a>(buf: &str, focused: bool) -> Line<'a> {
    let valid = parse_custom_base(buf);
    let label_style = Style::default().fg(Color::DarkGray);
    let cursor = if focused { "█" } else { "" };

    let value_style = match (buf.is_empty(), valid.is_some()) {
        (true, _) => Style::default().fg(Color::DarkGray),
        (false, true) => Style::default().fg(Color::LightCyan),
        (false, false) => Style::default().fg(Color::LightRed),
    };

    let hint = if valid.is_some() || buf.is_empty() {
        ""
    } else {
        "  ← deve ser 2–36"
    };

    Line::from(vec![
        Span::styled("  Base (2–36): ", label_style),
        Span::styled(buf.to_string(), value_style),
        Span::styled(cursor, Style::default().fg(Color::LightCyan)),
        Span::styled(hint, Style::default().fg(Color::Red)),
    ])
}

fn draw_conversor(frame: &mut Frame, left: Rect, right: Rect, state: &ConversorState) {
    let cursor = if state.focus == ConversorFocus::Input {
        "█"
    } else {
        ""
    };

    let source_focused = state.focus == ConversorFocus::SourceBase;
    let target_focused = state.focus == ConversorFocus::TargetBase;
    let source_custom_focused = state.focus == ConversorFocus::SourceCustom;
    let target_custom_focused = state.focus == ConversorFocus::TargetCustom;

    let label_style = Style::default().fg(Color::DarkGray);
    let hint_style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::DIM);

    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("Valor : ", label_style),
            Span::styled(
                state.input.as_str().to_string(),
                Style::default().fg(Color::White),
            ),
            Span::styled(cursor, Style::default().fg(Color::LightBlue)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("De    : ", label_style)]),
        base_selector_spans(
            state.source_base,
            source_focused,
            &state.source_custom_input,
        ),
    ];

    if state.source_base.is_custom() {
        lines.push(custom_base_input_line(
            &state.source_custom_input,
            source_custom_focused,
        ));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("Para  : ", label_style)]));
    lines.push(base_selector_spans(
        state.target_base,
        target_focused,
        &state.target_custom_input,
    ));

    if state.target_base.is_custom() {
        lines.push(custom_base_input_line(
            &state.target_custom_input,
            target_custom_focused,
        ));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[enter] converter  |  [2] ver trace",
        Style::default().fg(Color::LightGreen),
    )));

    let hint_text = match state.focus {
        ConversorFocus::Input => "[tab] navegar  [backspace] apagar",
        ConversorFocus::SourceBase | ConversorFocus::TargetBase => {
            "[tab] navegar  [← →] trocar base  [enter] editar custom"
        }
        ConversorFocus::SourceCustom | ConversorFocus::TargetCustom => {
            "[tab] navegar  [esc] voltar ao seletor  [0-9] digitar base"
        }
    };
    lines.push(Line::from(Span::styled(hint_text, hint_style)));

    frame.render_widget(Paragraph::new(lines), left);

    let output_lines = match &state.error {
        Some(err) => vec![
            Line::from(Span::styled("Erro:", Style::default().fg(Color::LightRed))),
            Line::from(""),
            Line::from(Span::styled(err.clone(), Style::default().fg(Color::Red))),
        ],
        None => vec![
            Line::from(Span::styled(
                "Output:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                state.output.clone(),
                Style::default().fg(Color::LightGreen),
            )),
        ],
    };

    frame.render_widget(Paragraph::new(output_lines), right);
}

// ─── draw_trace ──────────────────────────────────────────────────────────────

fn draw_trace(frame: &mut Frame, left: Rect, right: Rect, state: &TraceState) {
    // ── Estado vazio: nenhuma conversão ainda ────────────────────────────────
    if !state.tem_dados() {
        let msg = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Nenhum trace disponível.",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Faça uma conversão na aba [1] e volte aqui.",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        frame.render_widget(Paragraph::new(msg.clone()), left);
        frame.render_widget(Paragraph::new(msg), right);
        return;
    }

    let total = state.passos.len();
    let atual = state.passo_atual;

    // ── Painel esquerdo: lista de passos com o atual destacado ───────────────
    let items: Vec<ListItem> = state
        .passos
        .iter()
        .enumerate()
        .map(|(i, linha)| {
            // Classifica a linha para colorir de acordo com o tipo de operação
            let (cor_base, prefixo) = if linha.starts_with("Result") {
                (Color::LightGreen, "  ✓ ")
            } else if linha.contains("x ") && linha.contains('^') {
                (Color::LightYellow, "  × ") // multiplicação (base→dec)
            } else if linha.contains(" / ") {
                (Color::LightCyan, "  ÷ ") // divisão (dec→base)
            } else {
                (Color::Gray, "    ")
            };

            let style = if i == atual {
                Style::default()
                    .fg(Color::Black)
                    .bg(cor_base)
                    .add_modifier(Modifier::BOLD)
            } else if i < atual {
                // passos já vistos: cor normal, sem destaque
                Style::default().fg(cor_base)
            } else {
                // passos futuros: esmaecidos
                Style::default().fg(Color::DarkGray)
            };

            let numero = format!("{:>2}. ", i + 1);
            ListItem::new(Line::from(vec![
                Span::styled(prefixo, style),
                Span::styled(numero, style),
                Span::styled(linha.clone(), style),
            ]))
        })
        .collect();

    // Título do painel com contador de passo
    let titulo_esq = format!("  Passos ({}/{})  ", atual + 1, total);
    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(titulo_esq)
                .title_style(Style::default().fg(Color::LightBlue))
                .borders(Borders::NONE),
        ),
        left,
    );

    // ── Painel direito: detalhes da conversão + passo atual ampliado ─────────
    let label = Style::default().fg(Color::DarkGray);
    let valor_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let base_style = Style::default().fg(Color::LightBlue);
    let resultado_style = Style::default()
        .fg(Color::LightGreen)
        .add_modifier(Modifier::BOLD);
    let hint_style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::DIM);

    // Destaca o passo atual em tamanho grande no painel direito
    let passo_linha = &state.passos[atual];
    let (passo_cor, passo_tipo) = if passo_linha.starts_with("Result") {
        (Color::LightGreen, "Resultado final")
    } else if passo_linha.contains("x ") && passo_linha.contains('^') {
        (Color::LightYellow, "Soma posicional (base → decimal)")
    } else if passo_linha.contains(" / ") {
        (Color::LightCyan, "Divisão sucessiva (decimal → base)")
    } else {
        (Color::Gray, "Operação")
    };

    let right_lines = vec![
        Line::from(""),
        // ── Cabeçalho da conversão ────────────────────────────────────────
        Line::from(vec![
            Span::styled("Conversão : ", label),
            Span::styled(state.valor_original.clone(), valor_style),
            Span::styled("  (base ", label),
            Span::styled(state.base_origem.to_string(), base_style),
            Span::styled(")  →  base ", label),
            Span::styled(state.base_destino.to_string(), base_style),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Resultado : ", label),
            Span::styled(state.resultado.clone(), resultado_style),
        ]),
        Line::from(""),
        // ── Separador ────────────────────────────────────────────────────
        Line::from(Span::styled(
            "─────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        // ── Passo atual em destaque ───────────────────────────────────────
        Line::from(vec![
            Span::styled("Passo ", label),
            Span::styled(
                format!("{}/{}", atual + 1, total),
                Style::default().fg(Color::LightBlue),
            ),
            Span::styled("  —  ", label),
            Span::styled(passo_tipo, Style::default().fg(passo_cor)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            passo_linha.clone(),
            Style::default().fg(passo_cor).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        // ── Barra de progresso textual ────────────────────────────────────
        Line::from(progress_bar(atual, total)),
        Line::from(""),
        Line::from(Span::styled(
            "[← →] navegar passos  [1] voltar ao conversor",
            hint_style,
        )),
    ];

    frame.render_widget(Paragraph::new(right_lines), right);
}

/// Gera uma barra de progresso simples em texto: [████░░░░]  3/8
fn progress_bar(atual: usize, total: usize) -> Line<'static> {
    let largura: usize = 5;
    let cheios = if total > 1 {
        (atual * largura) / (total - 1)
    } else {
        largura
    };
    let vazios = largura.saturating_sub(cheios);

    // caracteres que simulam uma barra
    let barra_cheia = "█".repeat(cheios);
    let barra_vazia = "░".repeat(vazios);

    Line::from(vec![
        Span::styled("[", Style::default().fg(Color::DarkGray)),
        Span::styled(barra_cheia, Style::default().fg(Color::LightBlue)),
        Span::styled(barra_vazia, Style::default().fg(Color::DarkGray)),
        Span::styled("]", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("  {}/{}", atual + 1, total),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}

// ─── draw_batch ──────────────────────────────────────────────────────────────

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
            entries[selected].as_str().to_string(),
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
