use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Tabs, Widget},
    Frame,
};

use crate::app::{App, CurrentTab, OrderdFlag, SelectedFlag};

use strum::IntoEnumIterator;

/// UIを描画する
pub fn ui(frame: &mut Frame, app: &App) {
    // チャンク分割
    // |---------------|
    // |   Length(0)   | ヘッダーチャンク
    // |---------------|
    // |     Min(0)    | メインチャンク
    // |---------------|
    // |   Length(1)   | フッターチャンク
    // |---------------|
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // ヘッダーの描画

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(20)])
        .split(chunks[0]);

    let labels = CurrentTab::iter().map(CurrentTab::label);
    let highlight_style = (Color::default(), app.current_tab.palette().c700);
    let current_tab_index = app.current_tab as usize;

    let tabs = Tabs::new(labels)
        .highlight_style(highlight_style)
        .select(current_tab_index)
        .padding("", "")
        .divider(" ");

    frame.render_widget(tabs, header_chunks[0]);

    let title = Paragraph::new(Text::styled("SQL Generator", Style::default().bold()));

    frame.render_widget(title, header_chunks[1]);

    let footer = Line::raw("◄ ► to change tab | Press q to quit").centered();

    // カレントタブに基づいたメインパネルの描画
    match app.current_tab {
        CurrentTab::Init => app
            .current_tab
            .render_init(app, chunks[1], frame.buffer_mut()),
        CurrentTab::Select => app
            .current_tab
            .render_select(app, chunks[1], frame.buffer_mut()),
        CurrentTab::OrderBy => app
            .current_tab
            .render_order(app, chunks[1], frame.buffer_mut()),
        CurrentTab::Where => app
            .current_tab
            .render_where(app, chunks[1], frame.buffer_mut()),
    }

    // フッターの描画

    frame.render_widget(footer, chunks[2]);

    if app.currently_editing.is_some() {
        let popup_block = Block::default()
            .title("Enter a constraint for the selected column")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let column_block = Block::default().title("Column").borders(Borders::ALL);
        let constraint_block = Block::default().title("Constraint").borders(Borders::ALL);

        let column_text =
            Paragraph::new(app.base_columns[app.current_column].clone()).block(column_block);
        frame.render_widget(column_text, popup_chunks[0]);

        let constraint_text = Paragraph::new(app.constraint_input.clone()).block(constraint_block);
        frame.render_widget(constraint_text, popup_chunks[1]);
    }
}

impl CurrentTab {
    /// ラベルを描画する
    fn label(self) -> Line<'static> {
        format!(" {self} ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    /// 初期処理設定画面を描画する
    fn render_init(self, app: &App, area: Rect, buf: &mut Buffer) {
        let mut list_items = Vec::<ListItem>::new();
        for i in 0..app.init_config.len() {
            let mut text_color = Color::DarkGray;
            if i == app.current_column {
                text_color = Color::White;
            }

            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{} {}", app.init_config[i].0, app.init_config[i].1),
                Style::default().fg(text_color),
            ))));
        }

        List::new(list_items).block(self.block()).render(area, buf);
    }

    /// カラム指定画面を描画する
    fn render_select(self, app: &App, area: Rect, buf: &mut Buffer) {
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
                }
                SelectedFlag::NotSelected => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        &app.base_columns[i],
                        Style::default().fg(text_color),
                    ))));
                }
            }
        }

        List::new(list_items).block(self.block()).render(area, buf);
    }

    /// オーダー指定画面を描画する
    fn render_order(self, app: &App, area: Rect, buf: &mut Buffer) {
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
                }
                OrderdFlag::Desc => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("↓ {}", app.base_columns[i]),
                        Style::default().fg(text_color),
                    ))));
                }
                OrderdFlag::Off => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        &app.base_columns[i],
                        Style::default().fg(text_color),
                    ))));
                }
            }
        }

        List::new(list_items).block(self.block()).render(area, buf);
    }

    /// 制約指定画面を描画する
    fn render_where(self, app: &App, area: Rect, buf: &mut Buffer) {
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

            match &app.specified_columns.where_constraints[i] {
                Some(constraint) => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("{} {}", app.base_columns[i], constraint),
                        Style::default().fg(text_color),
                    ))));
                }
                None => {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        &app.base_columns[i],
                        Style::default().fg(text_color),
                    ))));
                }
            }
        }

        List::new(list_items).block(self.block()).render(area, buf);
    }

    /// ブロックを描画する
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    /// カレントタブをPaletteにマップする
    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Init => tailwind::ORANGE,
            Self::Select => tailwind::BLUE,
            Self::OrderBy => tailwind::EMERALD,
            Self::Where => tailwind::PURPLE,
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
