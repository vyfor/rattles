use rattles::presets;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

struct Entry {
    label: &'static str,
    tick: Box<dyn FnMut() -> &'static str>,
    toggle_reverse: Box<dyn FnMut()>,
}

enum DirectionalRattler<T: rattles::Rattle> {
    Forward(rattles::Rattler<T, false>),
    Reverse(rattles::Rattler<T, true>),
}

impl<T: rattles::Rattle> DirectionalRattler<T> {
    fn tick_now(&mut self) -> &'static str {
        match self {
            Self::Forward(rattler) => rattler.current_frames()[0],
            Self::Reverse(rattler) => rattler.current_frames()[0],
        }
    }

    fn toggle_reverse(&mut self) {
        *self = match std::mem::replace(self, Self::Forward(rattles::Rattler::new())) {
            Self::Forward(rattler) => Self::Reverse(rattler.reverse()),
            Self::Reverse(rattler) => Self::Forward(rattler.reverse()),
        };
    }
}

impl Entry {
    fn new<T>(label: &'static str, rattler: rattles::Rattler<T>) -> Self
    where
        T: rattles::Rattle + 'static,
    {
        let shared = Rc::new(RefCell::new(DirectionalRattler::Forward(rattler)));
        let tick_shared = Rc::clone(&shared);
        let reverse_shared = Rc::clone(&shared);

        Self {
            label,
            tick: Box::new(move || tick_shared.borrow_mut().tick_now()),
            toggle_reverse: Box::new(move || reverse_shared.borrow_mut().toggle_reverse()),
        }
    }

    fn toggle_reverse(&mut self) {
        (self.toggle_reverse)();
    }
}

struct Category {
    title: &'static str,
    entries: Vec<Entry>,
}

impl Category {
    fn toggle_reverse(&mut self) {
        for entry in &mut self.entries {
            entry.toggle_reverse();
        }
    }
}

struct TerminalGuard;

impl TerminalGuard {
    fn install() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

macro_rules! push_rattler {
    ($entries:expr, $label:literal, $ctor:path) => {{
        $entries.push(Entry::new($label, $ctor()));
    }};
}

fn build_categories() -> Vec<Category> {
    let mut arrows = Vec::new();
    push_rattler!(arrows, "arrow", presets::arrows::arrow);
    push_rattler!(arrows, "double_arrow", presets::arrows::double_arrow);

    let mut ascii = Vec::new();
    push_rattler!(ascii, "dqpb", presets::ascii::dqpb);
    push_rattler!(ascii, "rolling_line", presets::ascii::rolling_line);
    push_rattler!(ascii, "simple_dots", presets::ascii::simple_dots);
    push_rattler!(
        ascii,
        "simple_dots_scrolling",
        presets::ascii::simple_dots_scrolling
    );
    push_rattler!(ascii, "arc", presets::ascii::arc);
    push_rattler!(ascii, "balloon", presets::ascii::balloon);
    push_rattler!(ascii, "circle_halves", presets::ascii::circle_halves);
    push_rattler!(ascii, "circle_quarters", presets::ascii::circle_quarters);
    push_rattler!(ascii, "point", presets::ascii::point);
    push_rattler!(ascii, "square_corners", presets::ascii::square_corners);
    push_rattler!(ascii, "toggle", presets::ascii::toggle);
    push_rattler!(ascii, "triangle", presets::ascii::triangle);
    push_rattler!(ascii, "grow_horizontal", presets::ascii::grow_horizontal);
    push_rattler!(ascii, "grow_vertical", presets::ascii::grow_vertical);
    push_rattler!(ascii, "noise", presets::ascii::noise);

    let mut braille = Vec::new();
    push_rattler!(braille, "dots", presets::braille::dots);
    push_rattler!(braille, "dots2", presets::braille::dots2);
    push_rattler!(braille, "dots3", presets::braille::dots3);
    push_rattler!(braille, "dots4", presets::braille::dots4);
    push_rattler!(braille, "dots5", presets::braille::dots5);
    push_rattler!(braille, "dots6", presets::braille::dots6);
    push_rattler!(braille, "dots7", presets::braille::dots7);
    push_rattler!(braille, "dots8", presets::braille::dots8);
    push_rattler!(braille, "dots9", presets::braille::dots9);
    push_rattler!(braille, "dots10", presets::braille::dots10);
    push_rattler!(braille, "dots11", presets::braille::dots11);
    push_rattler!(braille, "dots12", presets::braille::dots12);
    push_rattler!(braille, "dots13", presets::braille::dots13);
    push_rattler!(braille, "dots14", presets::braille::dots14);
    push_rattler!(braille, "dots_circle", presets::braille::dots_circle);
    push_rattler!(braille, "sand", presets::braille::sand);
    push_rattler!(braille, "bounce", presets::braille::bounce);
    push_rattler!(braille, "braillewave", presets::braille::wave);
    push_rattler!(braille, "scan", presets::braille::scan);
    push_rattler!(braille, "rain", presets::braille::rain);
    push_rattler!(braille, "pulse", presets::braille::pulse);
    push_rattler!(braille, "snake", presets::braille::snake);
    push_rattler!(braille, "sparkle", presets::braille::sparkle);
    push_rattler!(braille, "cascade", presets::braille::cascade);
    push_rattler!(braille, "columns", presets::braille::columns);
    push_rattler!(braille, "orbit", presets::braille::orbit);
    push_rattler!(braille, "breathe", presets::braille::breathe);
    push_rattler!(braille, "waverows", presets::braille::waverows);
    push_rattler!(braille, "checkerboard", presets::braille::checkerboard);
    push_rattler!(braille, "helix", presets::braille::helix);
    push_rattler!(braille, "fillsweep", presets::braille::fillsweep);
    push_rattler!(braille, "diagswipe", presets::braille::diagswipe);
    push_rattler!(braille, "infinity", presets::braille::infinity);

    let mut emoji = Vec::new();
    push_rattler!(emoji, "hearts", presets::emoji::hearts);
    push_rattler!(emoji, "clock", presets::emoji::clock);
    push_rattler!(emoji, "earth", presets::emoji::earth);
    push_rattler!(emoji, "moon", presets::emoji::moon);
    push_rattler!(emoji, "speaker", presets::emoji::speaker);
    push_rattler!(emoji, "weather", presets::emoji::weather);

    vec![
        Category {
            title: "Arrows",
            entries: arrows,
        },
        Category {
            title: "ASCII",
            entries: ascii,
        },
        Category {
            title: "Braille",
            entries: braille,
        },
        Category {
            title: "Emoji",
            entries: emoji,
        },
    ]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut categories = build_categories();
    let _guard = TerminalGuard::install()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Fill(1)])
                .split(area);

            let hint = Paragraph::new(Line::raw("q - quit | r - reverse"));
            frame.render_widget(hint, outer[0]);

            let row_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(outer[1]);

            let top = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(row_chunks[0]);
            let bottom = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(row_chunks[1]);

            let chunks = [top[0], top[1], bottom[0], bottom[1]];

            for (chunk, category) in chunks.into_iter().zip(categories.iter_mut()) {
                let title = format!(" {} ", category.title);
                let block = Block::default().borders(Borders::ALL).title(Line::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ));

                let content_width = chunk.width.saturating_sub(2) as usize;
                let content_height = chunk.height.saturating_sub(2) as usize;
                let mut lines = vec![String::new(); content_height];

                let frames: Vec<(&'static str, &'static str)> = category
                    .entries
                    .iter_mut()
                    .map(|entry| (entry.label, (entry.tick)()))
                    .collect();

                if !frames.is_empty() && content_width > 0 && content_height > 0 {
                    let max_label = frames
                        .iter()
                        .map(|(label, _)| label.len())
                        .max()
                        .unwrap_or(8);
                    let cell_width = (max_label + 5).max(16);
                    let label_width = max_label.min(cell_width.saturating_sub(2));
                    let cols = (content_width / cell_width).max(1);
                    let item_stride = if content_height >= 2 { 2 } else { 1 };
                    let rows_per_col = (content_height / item_stride).max(1);
                    let visible = rows_per_col.saturating_mul(cols).min(frames.len());

                    for (idx, (label, frame_text)) in frames.into_iter().take(visible).enumerate() {
                        let row = idx % rows_per_col;
                        let col = idx / rows_per_col;
                        let line_idx = row * item_stride;

                        if line_idx >= lines.len() {
                            continue;
                        }

                        let item_text = format!("{label:<label_width$} {frame_text}");
                        let padded = format!("{item_text:<cell_width$}");

                        if col > 0 {
                            lines[line_idx].push(' ');
                        }

                        lines[line_idx].push_str(&padded);
                    }
                }

                let paragraph =
                    Paragraph::new(lines.into_iter().map(Line::raw).collect::<Vec<_>>())
                        .block(block);
                frame.render_widget(paragraph, chunk);
            }
        })?;

        if event::poll(Duration::from_millis(10))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            let is_ctrl_c = matches!(key.code, KeyCode::Char('c') | KeyCode::Char('C'))
                && key.modifiers.contains(KeyModifiers::CONTROL);

            if is_ctrl_c
                || matches!(
                    key.code,
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q')
                )
            {
                break;
            }

            if matches!(key.code, KeyCode::Char('r') | KeyCode::Char('R')) {
                for category in &mut categories {
                    category.toggle_reverse();
                }
            }
        }

        std::thread::sleep(Duration::from_millis(33));
    }

    Ok(())
}
