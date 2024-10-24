use crossterm::event::{Event, KeyCode, read};
use std::io;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

pub struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl <'a> App<'a> {
    pub fn new() -> App<'a> {
        let mut app = App { state: TableState::default(), 
            items: vec![
                vec!["Banco de Dados 1", "Usuario 1", "MySQL"],
                vec!["Banco de Dados 2","Usuario 2","PostgreSQL"],
                vec!["Banco de Dados 3","Usuario 3","Sqlite"],
            ] };
        app.next();
        app
    }
    
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    
    fn action(&mut self) {
        println!("{:?}", self.state.selected());
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => app.action(),
                _ => {}
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let main_area = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let table_area = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)].as_ref()) // Margens superior e inferior
        .split(main_area[0]);

    let centered_table_area = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)].as_ref()) // Margens esquerda e direita
        .split(table_area[1]);

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::DarkGray);
    let header_cells = ["Name", "User", "SGBD"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Databases ").title_alignment(Alignment::Center))
        .highlight_style(selected_style)
        .highlight_symbol("=> ")
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ]);

    f.render_stateful_widget(t, centered_table_area[1], &mut app.state);
}