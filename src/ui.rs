use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect
    },
    style::{
        palette::tailwind,
        Color,
        Style,
        Stylize
    },
    symbols, 
    text::{
        Line,
        Span,
        Text
    },
    widgets::{
        Block,
        List,
        ListItem,
        Paragraph,
        Padding,
        Tabs,
        Widget
    },
    Frame
};

use crate::app::{
    App, CurrentTab, OrderdFlag, SelectedFlag
};

use strum::IntoEnumIterator;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());
    

    // Render Header

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(20),
        ])
        .split(chunks[0]);

    let labels = CurrentTab::iter().map(CurrentTab::label);
    let highlight_style =  (Color::default(), app.current_tab.palette().c700);
    let current_tab_index = app.current_tab as usize;
    
    let tabs = Tabs::new(labels)
        .highlight_style(highlight_style)
        .select(current_tab_index)
        .padding("", "")
        .divider(" ");
    
    frame.render_widget(tabs, header_chunks[0]);
    
    let title = Paragraph::new(Text::styled(
        "SQL Generator",
        Style::default().bold(),
    ));
    
    frame.render_widget(title, header_chunks[1]);
    
    let footer = Line::raw("◄ ► to change tab | Press q to quit")
        .centered();
    
    // Render main panel

    // app.current_tab.render(chunks[1], frame.buffer_mut());
    
    match app.current_tab {
        CurrentTab::Tab0 => app.current_tab.render_tab0(app, chunks[1], frame.buffer_mut()),
        CurrentTab::Tab1 => app.current_tab.render_tab1(app, chunks[1], frame.buffer_mut()),
    }
    

    // Render footer
    
    frame.render_widget(footer, chunks[2]);
}

impl CurrentTab {
    fn label(self) -> Line<'static> {
        format!(" {self} ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, app: &App, area: Rect, buf: &mut Buffer) {
        let mut list_items = Vec::<ListItem>::new();
        let mut starting_point = 0;
        if app.current_column as isize - 4 >= 0 {
            starting_point = app.current_column - 4;
        }
        for i in starting_point..app.base_columns.len() {
            let mut text_color = Color::DarkGray;
            if i == app.current_column {
                text_color = Color::White;
            }

            match app.specified_columns.selected_columns[i] {
                SelectedFlag::Selected => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("✓ {}", app.base_columns[i]),
                        Style::default().fg(text_color),
                    ))));
                },
                SelectedFlag::NotSelected => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("{}", app.base_columns[i]),
                        Style::default().fg(text_color),
                    ))));
                }
            }

        }
        
        List::new(list_items)
            .block(self.block())
            .render(area, buf);
    }
    
    fn render_tab1(self, app: &App, area: Rect, buf: &mut Buffer) {
        let mut list_items = Vec::<ListItem>::new();
        let mut starting_point = 0;
        if app.current_column as isize - 4 >= 0 {
            starting_point = app.current_column - 4;
        }
        for i in starting_point..app.base_columns.len() {
            let mut text_color = Color::DarkGray;
            if i == app.current_column {
                text_color = Color::White;
            }

            match app.specified_columns.ordered_columns[i] {
                OrderdFlag::Asc => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("↑ {}", app.base_columns[i]),
                        Style::default().fg(text_color),
                    ))));
                },
                OrderdFlag::Desc => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("↓ {}", app.base_columns[i]),
                        Style::default().fg(text_color),
                    ))));
                },
                OrderdFlag::Off => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("{}", app.base_columns[i]),
                        Style::default().fg(text_color),
                    ))));
                }
            }

        }
        
        List::new(list_items)
            .block(self.block())
            .render(area, buf);
    }
    
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }
    
    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab0 => tailwind::BLUE,
            Self::Tab1 => tailwind::EMERALD,
        }
    }
}