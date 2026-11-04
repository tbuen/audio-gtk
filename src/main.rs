mod gui;
mod model;
mod operation;
mod update;

use std::sync::mpsc;

use flexi_logger::Logger;
use log::info;

use gui::Gui;
use model::Model;

fn main() {
    let logger = Logger::try_with_str("info").unwrap();
    logger.start().unwrap();

    let (operation_sender, operation_receiver) = mpsc::channel();
    let (update_sender, update_receiver) = mpsc::channel();

    let model = Model::new(operation_receiver, update_sender);
    let gui = Gui::new(update_receiver, operation_sender);

    gui.run();

    drop(model);
    info!("Exiting gracefully...");
}
