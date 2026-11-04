use std::sync::mpsc::{Receiver, Sender};
use std::thread::{Builder, JoinHandle};
use std::time::Duration;

use backend::{Backend, Event};
use log::{debug, info, warn};

use crate::operation::Operation;
use crate::update::Update;

pub struct Model {
    handle: Option<JoinHandle<()>>,
}

impl Model {
    pub fn new(operation: Receiver<Operation>, update: Sender<Update>) -> Self {
        Self {
            handle: Some(
                Builder::new()
                    .name("model".into())
                    .spawn(move || Self::thread(operation, update))
                    .unwrap(),
            ),
        }
    }

    fn thread(operation: Receiver<Operation>, update: Sender<Update>) {
        if let Ok(Operation::BackendStart) = operation.recv() {
            Self::run(operation, update);
        }
        debug!("exit model thread");
    }

    fn run(operation: Receiver<Operation>, update: Sender<Update>) {
        let backend = Backend::new(None);
        let receiver = backend.receiver().unwrap();

        loop {
            if let Ok(op) = operation.recv_timeout(Duration::from_millis(10)) {
                match op {
                    Operation::BackendStart => {
                        warn!("received BackendStart while already running");
                    }
                    Operation::BackendStop => break,
                    Operation::SyncFiles => backend.sync_files(),
                }
            }

            if let Ok(evt) = receiver.recv_timeout(Duration::from_millis(10)) {
                match evt {
                    Event::Connected => update.send(Update::Connected).unwrap(),
                    Event::Disconnected => update.send(Update::Disconnected).unwrap(),
                    _ => info!("received {evt:?}"),
                }
            }
        }
        update.send(Update::Quit).unwrap();
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}
