use std::{error::Error, io};
use std::env;

use app::{CurrentTab, CurrentlyEditing};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{
        App,
        AppState,
        Table,
        SelectedFlag,
        OrderdFlag,
    },
    ui::ui,
};

use std::fs::{self};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let table_name = &args[1];
    let filepath = format!("./tables/{}.toml", table_name);

    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    
    let str = fs::read_to_string(filepath)?;
    let table: Table = toml::from_str(&str)?;

    let mut app = App::new(table);

    let res = run_app(&mut terminal, &mut app);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Ok(()) = res {
        app.generate_query(table_name);
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.state {
                AppState::Running => match key.code {
                    KeyCode::Char('l') | KeyCode::Right => app.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => app.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                    KeyCode::Char('j') | KeyCode::Down => app.next_column(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous_column(),
                    KeyCode::Enter => match app.current_tab {
                        CurrentTab::Init => {}
                        CurrentTab::Select => {
                            let current_column = app.current_column;
                            match app.specified_columns.selected_columns[current_column] {
                                SelectedFlag::Selected => app.specified_columns.selected_columns[current_column] = SelectedFlag::NotSelected,
                                SelectedFlag::NotSelected => app.specified_columns.selected_columns[current_column] = SelectedFlag::Selected,
                            }
                        },
                        CurrentTab::OrderBy => {
                            let current_column = app.current_column;
                            match app.specified_columns.ordered_columns[current_column] {
                                OrderdFlag::Asc => app.specified_columns.ordered_columns[current_column] = OrderdFlag::Desc,
                                OrderdFlag::Desc => app.specified_columns.ordered_columns[current_column] = OrderdFlag::Off,
                                OrderdFlag::Off => app.specified_columns.ordered_columns[current_column] = OrderdFlag::Asc,
                            }
                        },
                        CurrentTab::Where => {}
                    }
                    KeyCode::Char('a') => {
                        if let CurrentTab::Select = app.current_tab {
                            for i in 0..app.base_columns.len() {
                                app.specified_columns.selected_columns[i] = SelectedFlag::Selected;
                            }
                        }
                    },
                    KeyCode::Char('e') => {
                        if let CurrentTab::Where = app.current_tab {
                            app.state = AppState::Editing;
                            app.currently_editing = Some(app::CurrentlyEditing::Constraint);
                            if let Some(existing_constraint) = &app.specified_columns.where_constraints[app.current_column] {
                                app.constraint_input = existing_constraint.clone();
                            }
                        }
                    }
                    _ => {}
                },
                AppState::Editing => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                app.save_constraint();
                                app.state = AppState::Running;
                            }
                        },
                        KeyCode::Backspace => {
                            app.constraint_input.pop();
                        },
                        KeyCode::Esc => {
                            app.state = AppState::Running;
                            app.clear_constraint();
                            app.currently_editing = None;
                        },
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                app.constraint_input.push(value);
                            }
                        }
                        _ => {}
                    }
                },
                AppState::Quitting => {
                    return Ok(());
                },
            }
        }
        
    }
}