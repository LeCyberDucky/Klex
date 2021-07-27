use crate::ui;
use crate::util::{self, Message};


pub enum Data {

}

pub enum Event {

}

pub struct Backend {
    ui: util::ThreadChannel<Message<ui::Data, ui::Event>, Message<Data, Event>>
}

impl Backend {
    pub fn new(ui: util::ThreadChannel<Message<ui::Data, ui::Event>, Message<Data, Event>>) -> Self { Self { ui } }

    pub fn run(&self) {}
}