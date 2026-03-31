use crate::event::Event;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Dialog {
    title: String,
    dialogs: Vec<DialogBox>,
}

impl Dialog {
    pub fn new(title: String, dialogs: Vec<DialogBox>) -> Dialog {
        Dialog { title, dialogs }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_dialogs(&self) -> &Vec<DialogBox> {
        &self.dialogs
    }
}

#[derive(Deserialize, Clone)]
pub struct DialogBox {
    description: String,
    options: Vec<DialogOption>,
}

impl DialogBox {
    pub fn new(description: String, options: Vec<DialogOption>) -> DialogBox {
        DialogBox {
            description,
            options,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_options(&self) -> &Vec<DialogOption> {
        &self.options
    }
}

#[derive(Deserialize, Clone)]
pub struct DialogOption {
    description: String,
    event: Event,
    next: usize,
}

impl DialogOption {
    pub fn new(description: String, event: Event, next: usize) -> DialogOption {
        DialogOption {
            description,
            event,
            next,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_event(&self) -> &Event {
        &self.event
    }

    pub fn get_next(&self) -> usize {
        self.next
    }
}
