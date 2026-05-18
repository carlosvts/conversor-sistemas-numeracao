use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};

// backend e frontend
mod contracts;
mod ui;

use contracts::dto::{ConversionOptions, RawConversionInput};
use contracts::facade::ConversionFacade;
use ui::app::{App, ConversorFocus, Tabs, draw};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let facade = ConversionFacade::new_default();

    loop {
        // similar com raylib
        // desenha
        terminal.draw(|frame| draw(frame, &app))?;

        // handle input
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }

            // se for Enter no conversor, chama o facade antes de delegar
            if app.tab == Tabs::Conversor
                && app.conversor.focus == ConversorFocus::Input
                && key.code == KeyCode::Enter
            {
                let result = facade.request(RawConversionInput {
                    raw_value: app.conversor.input.clone(),
                    source_base_hint: app.conversor.source_base.to_hint(),
                    target_base: app.conversor.target_base.to_u8(),
                    options: ConversionOptions {
                        generate_trace: true,
                        ..Default::default()
                    },
                });

                match result {
                    Ok(r) => {
                        // clones para evitar borrowing, ja que usamos mais de uma vez
                        app.conversor.output = r.output_value.clone();
                        app.trace.valor_original = app.conversor.input.clone();
                        app.trace.base_origem =
                            app.conversor.source_base.to_hint().unwrap_or(r.source_base);
                        app.trace.base_destino = app.conversor.target_base.to_u8();
                        app.trace.resultado = r.output_value.clone();

                        app.trace.passos = r.trace.clone();
                        app.trace.passo_atual = 0;
                    }
                    Err(_) => {
                        // fallback: tenta converter para decimal
                        let fallback = facade.request(RawConversionInput {
                            raw_value: app.conversor.input.clone(),
                            source_base_hint: app.conversor.source_base.to_hint(),
                            target_base: 10,
                            options: ConversionOptions::default(),
                        });
                        match fallback {
                            Ok(r) => app.conversor.output = format!("(decimal) {}", r.output_value),
                            Err(e) => app.conversor.error = Some(format!("{:?}", e)),
                        }
                    }
                }
            }

            app.handle_key(key.code);
        }
    }
    // limpa tudo para o final do loop
    ratatui::restore();
    // return 0
    Ok(())
}
