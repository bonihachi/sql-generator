use strum::{Display, EnumIter, FromRepr};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Table {
    columns: Vec<String>
}


#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, FromRepr, EnumIter, Display)]
pub enum CurrentTab {
    #[default]
    #[strum(to_string = "SELECT")]
    Tab0,
    #[strum(to_string = "ORDER BY")]
    Tab1,
}

impl CurrentTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum SelectedFlag {
    #[default]
    Selected,
    NotSelected,
}

#[derive(Default, Clone, Copy)]
pub enum OrderdFlag {
    #[default]
    Asc,
    Desc,
    Off,
}

#[derive(Default)]
pub struct SpecifiedColumns {
    pub selected_columns: Vec<SelectedFlag>, // for SELECT
    pub ordered_columns: Vec<OrderdFlag>, // for ORDERD BY
}

impl SpecifiedColumns {
    pub fn new(len: usize) -> SpecifiedColumns {
        SpecifiedColumns {
            selected_columns: vec![SelectedFlag::NotSelected; len],
            ordered_columns: vec![OrderdFlag::Off; len],
        }
    }
}

#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub current_tab: CurrentTab,
    pub base_columns: Vec<String>,
    pub current_column: usize,
    pub specified_columns: SpecifiedColumns,
}

impl App {
    pub fn new(table: Table) -> App {
        let len = table.columns.len();
        App {
            state: AppState::Running,
            current_tab: CurrentTab::Tab0,
            base_columns: table.columns,
            current_column: 0,
            specified_columns: SpecifiedColumns::new(len),
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
        self.current_column = 0;
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.previous();
        self.current_column = 0;
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }

    pub fn previous_column(&mut self) {
        if self.current_column > 0 {
            self.current_column = self.current_column - 1;
        } else {
            self.current_column = self.base_columns.len() - 1;
        }
    }
    
    pub fn next_column(&mut self) {
        if self.current_column < self.base_columns.len() - 1 {
            self.current_column = self.current_column + 1;
        } else {
            self.current_column = 0;
        }
    }
    
    pub fn generate_query(self, table_name: &String) {
        let len = self.base_columns.len();
        
        print!("SELECT");
        
        let mut num_of_selected_columns = 0;
        
        for i in 0..len {
            if self.specified_columns.selected_columns[i] == SelectedFlag::Selected {
                num_of_selected_columns += 1;
            }
        }
        
        if num_of_selected_columns == len {
            print!(" *")
        } else {
            let mut first_element = true;
            for i in 0..len {
                match self.specified_columns.selected_columns[i] {
                    SelectedFlag::Selected =>  {
                        if first_element {
                            print!(" {}", self.base_columns[i]);
                            first_element = false;
                        } else {
                            print!(", {}", self.base_columns[i]);
                        }
                    }
                    SelectedFlag::NotSelected => {},
                }
            }
        }
        println!();
        
        println!("FROM {}", table_name);
        
        let mut first_element = true;
        for i in 0..len {
            match self.specified_columns.ordered_columns[i] {
                OrderdFlag::Asc =>  {
                    if first_element {
                        print!("ORDER BY {} ASC", self.base_columns[i]);
                        first_element = false;
                    } else {
                        print!(", {} ASC", self.base_columns[i]);
                    }
                }
                OrderdFlag::Desc =>  {
                    if first_element {
                        print!("ORDER BY {} DESC", self.base_columns[i]);
                        first_element = false;
                    } else {
                        print!(", {} DESC", self.base_columns[i]);
                    }
                }
                OrderdFlag::Off => {},
            }
        }
        println!(";");
    }
}
