use anyhow::Result;
use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            key_input: String::default(),
            value_input: String::default(),
            pairs: HashMap::default(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::default();
        self.value_input = String::default();
        self.currently_editing = Option::default();
    }

    pub fn toggle_editing(&mut self) {
        let Some(edit_mode) = &self.currently_editing else {
            self.currently_editing = Some(CurrentlyEditing::Key);
            return;
        };

        match edit_mode {
            CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
            CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
}
