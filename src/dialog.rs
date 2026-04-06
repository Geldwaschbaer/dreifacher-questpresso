use crate::event::{Event, EventBuilder};
use async_from::{AsyncFrom, async_trait};
use macroquad::texture::{Texture2D, load_texture};
use serde::Deserialize;

#[derive(Clone)]
pub struct Dialog {
    title: String,
    dialogs: Vec<DialogBox>,
    texture: Texture2D,
}

impl Dialog {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_dialogs(&self) -> &Vec<DialogBox> {
        &self.dialogs
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.texture
    }
}

#[derive(Deserialize)]
pub struct DialogBuilder {
    title: String,
    dialogs: Vec<DialogBoxBuilder>,
    texture: String,
}

#[async_trait]
impl AsyncFrom<DialogBuilder> for Dialog {
    async fn async_from(value: DialogBuilder) -> Self {
        let mut dialogs = Vec::new();
        for dialog_builder in value.dialogs {
            dialogs.push(DialogBox::async_from(dialog_builder).await);
        }
        let texture = load_texture(&value.texture)
            .await
            .expect("expect dialog texture exists");
        Dialog {
            title: value.title,
            dialogs,
            texture,
        }
    }
}

#[derive(Clone)]
pub struct DialogBox {
    description: String,
    options: Vec<DialogOption>,
}

impl DialogBox {
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_options(&self) -> &Vec<DialogOption> {
        &self.options
    }
}

#[derive(Deserialize)]
pub struct DialogBoxBuilder {
    description: String,
    options: Vec<DialogOptionBuilder>,
}

#[async_trait]
impl AsyncFrom<DialogBoxBuilder> for DialogBox {
    async fn async_from(value: DialogBoxBuilder) -> Self {
        let mut options = Vec::new();
        for option_builder in value.options {
            options.push(DialogOption::async_from(option_builder).await);
        }
        DialogBox {
            description: value.description,
            options,
        }
    }
}

#[derive(Clone)]
pub struct DialogOption {
    description: String,
    events: Vec<Event>,
    next: usize,
}

impl DialogOption {
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get_next(&self) -> usize {
        self.next
    }
}

#[derive(Deserialize)]
pub struct DialogOptionBuilder {
    description: String,
    #[serde(default = "Vec::new")]
    events: Vec<EventBuilder>,
    next: usize,
}

#[async_trait]
impl AsyncFrom<DialogOptionBuilder> for DialogOption {
    async fn async_from(value: DialogOptionBuilder) -> Self {
        let mut events = Vec::new();
        for event_builder in value.events {
            events.push(Event::async_from(event_builder).await);
        }
        DialogOption {
            description: value.description,
            events,
            next: value.next,
        }
    }
}
