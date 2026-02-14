use std::error::Error;
use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use uom::si::electric_current::ampere;
use uom::si::{capacitance::microfarad, inductance::microhenry};

use crate::app::AppState;
use crate::fields::FieldId;

mod app;
mod buck_boost_designer;
mod fields;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(12),
                    Constraint::Min(8),
                    Constraint::Length(3),
                ])
                .split(size);

            // Top: Inputs
            let input_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);


            let left_fields = FieldId::all().iter().take(8).collect::<Vec<_>>();
            let right_fields = FieldId::all().iter().skip(8).collect::<Vec<_>>();

            let inputs_left = render_fields(&app, &left_fields, 0);
            let inputs_right = render_fields(&app, &right_fields, 8);

            f.render_widget(inputs_left, input_chunks[0]);
            f.render_widget(inputs_right, input_chunks[1]);

            // Middle: Results or Heatmap
            let cached_worst = app.cached_worst;

            let mid_text = if app.show_heatmap {
                app.designer.ascii_heatmap(app.steps, cached_worst.inductance)
            } else {
                format!(
                    "Worst-case (over VIN×VOUT sweep)\n\
                     Inductance = {:.2} µH\n\
                     Peak Inductor Current = {:.2} A\n\
                     Input Capacitance = {:.2} µF\n\
                     Input Capacitance (bulk) = {:.2} µF (min), {:.2} µF (max)\n\
                     Output Capacitance = {:.2} µF\n\n\
                     Best efficiency = {:.2}%\n\
                     Worst efficiency = {:.2}%\n\n\
                     FET peak current (buck) = {:.2} A\n\
                     FET peak current (boost) = {:.2} A\n\n\
                     Controls: ↑/↓ select, (Enter) edit/commit, (Esc) cancel, (h) heatmap, (e) export CSV, (m) export Markdown, (q) quit",
                    cached_worst.inductance.get::<microhenry>(),
                    cached_worst.peak_current.get::<ampere>(),
                    cached_worst.cin.get::<microfarad>(),
                    app.designer.cin_bulk_min().get::<microfarad>(),
                    app.designer.cin_bulk_max().get::<microfarad>(),
                    cached_worst.cout.get::<microfarad>(),
                    cached_worst.eff_high.value * 100.0,
                    cached_worst.eff_low.value * 100.0,
                    cached_worst.fet_peak_buck.get::<ampere>(),
                    cached_worst.fet_peak_boost.get::<ampere>(),
                )
            };

            let mid = Paragraph::new(mid_text)
                .block(Block::default().title("Results").borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            f.render_widget(mid, chunks[1]);

            // Bottom: Status
            let status = Paragraph::new(app.status.as_str())
                .block(Block::default().title("Status").borders(Borders::ALL));
            f.render_widget(status, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(50))?
            && let Event::Key(k) = event::read()?
            && handle_key(&mut app, k)?
        {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn render_fields(app: &AppState, fields: &[&FieldId], base_index: usize) -> Paragraph<'static> {
    let mut lines: Vec<Spans> = Vec::new();

    for (i, fid) in fields.iter().enumerate() {
        let idx = base_index + i;
        let selected = idx == app.selected;

        let mut label = fid.label().to_string();
        let value = app.selected_value_display(idx);

        if selected {
            label = format!("> {label}");
        } else {
            label = format!("  {label}");
        }

        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        lines.push(Spans::from(vec![
            Span::styled(format!("{:<18}", label), style),
            Span::raw(" : "),
            Span::styled(value, style),
        ]));
    }

    Paragraph::new(lines)
        .block(Block::default().title("Inputs").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
}

fn handle_key(app: &mut AppState, key: KeyEvent) -> Result<bool, Box<dyn Error>> {
    if app.editing {
        match key.code {
            KeyCode::Esc => app.cancel_edit(),
            KeyCode::Enter => app.commit_edit(),
            KeyCode::Backspace => {
                app.edit_buf.pop();
            }
            KeyCode::Char(c) => {
                // allow typical numeric input including exponent and sign
                if c.is_ascii_digit() || ".-+eE".contains(c) {
                    app.edit_buf.push(c);
                }
            }
            _ => {}
        }
        return Ok(false);
    }

    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('h') => {
            app.show_heatmap = !app.show_heatmap;
            app.status = if app.show_heatmap {
                "Heatmap ON. Press h to return.".into()
            } else {
                "Heatmap OFF. Showing results.".into()
            };
        }
        KeyCode::Char('e') => {
            // allow Ctrl+E or e
            let filename = format!(
                "buckboost_vin_vout_map_{}.csv",
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            );
            match app.designer.export_csv(filename.as_str(), app.steps) {
                Ok(()) => app.status = format!("Exported CSV: {filename}"),
                Err(e) => app.status = format!("CSV export failed: {e}"),
            }
        }
        KeyCode::Char('m') => {
            // allow Ctrl+M or m
            let now = chrono::Local::now();
            let filename = format!(
                "buckboost_vin_vout_map_{}.md",
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            );
            match app.designer.render_markdown(filename.as_str(), now) {
                Ok(()) => app.status = format!("Exported Markdown: {filename}"),
                Err(e) => app.status = format!("Markdown export failed: {e}"),
            }
        }
        KeyCode::Up => {
            if app.selected > 0 {
                app.selected -= 1;
            }
        }
        KeyCode::Down => {
            if app.selected + 1 < app.field_count() {
                app.selected += 1;
            }
        }
        KeyCode::Enter => {
            app.begin_edit();
        }
        _ => {
            // Ctrl+C also quits (nice in terminal tools)
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
