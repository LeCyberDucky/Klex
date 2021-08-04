use std::thread;
use std::time::Instant;

use iced::{executor, Application, Command};

use crate::backend;
use crate::util::{self, Message};

pub enum Data {}

pub enum Event {}

// pub mod data {
//     use super::*;

//     #[derive(Clone)]
//     pub enum Backend {
//         // FilePathList(StyledPathList),
//     }

//     // #[derive(Clone)]
//     // pub enum Server {
//     //     ConnectionInfo {
//     //         public_ip: Option<IpAddr>,
//     //         external_port: Option<u16>,
//     //         status: String,
//     //         secret_key: String,
//     //     },
//     // }

//     impl Data for Backend {}
//     // impl Data for Server {}
// }

// pub mod event {
//     use super::*;

//     #[derive(Clone)]
//     pub enum Backend {
//         // StateChange(AppState),
//     }

//     // #[derive(Clone)]
//     // pub enum Server {}

//     impl Event for Backend {}
//     // impl Event for Server {}
// }

#[derive(Debug, Clone)]
pub enum InternalMessage {
    Tick(Instant),
}

pub struct UI {
    backend: util::ThreadChannel<Message<backend::Data, backend::Event>, Message<Data, Event>>,
    settings: Settings,
}

pub struct Settings {
    target_refresh_rate: u64,
}

impl Settings {
    fn new(target_refresh_rate: u64) -> Self {
        Self {
            target_refresh_rate,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let target_refresh_rate = 60;
        Self::new(target_refresh_rate)
    }
}

impl Application for UI {
    type Executor = executor::Default;
    type Message = InternalMessage;
    type Flags = Settings;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (ui, backend) = util::ThreadChannel::new_pair();

        thread::Builder::new()
            .name("Klex - Backend".into())
            .spawn(move || {
                let backend = backend::Backend::new(ui);
                backend.run();
            });

        let ui = UI {
            backend,
            settings: flags,
        };

        (ui, Command::none())
    }

    fn title(&self) -> String {
        String::from("Klex")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Self::Message::Tick(_) => {
                let backend_updates = self.backend.receive();
                for update in backend_updates {
                    match update {
                        Message::Data(data) => match data {},
                        Message::Event(event) => match event {},
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
