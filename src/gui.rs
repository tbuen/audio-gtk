use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};

use adw::gio::resources_register_include;
use adw::glib::ControlFlow;
use adw::gtk::{Builder, Button};
use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use log::debug;

use crate::operation::Operation;
use crate::update::Update;

pub struct Gui {
    app: Application,
}

impl Gui {
    pub fn new(update: Receiver<Update>, operation: Sender<Operation>) -> Self {
        let update = Rc::new(update);

        let app = Application::builder()
            .application_id("com.github.tbuen.audio-gtk")
            .build();

        app.connect_startup(glib::clone!(
            #[strong]
            operation,
            move |_| {
                debug!("startup begin");
                operation.send(Operation::BackendStart).unwrap();
                debug!("startup end");
            }
        ));

        app.connect_activate(glib::clone!(
            #[strong]
            operation,
            move |app| {
                debug!("activate begin");
                let builder = Builder::new();
                Self::build_ui(app, &builder, &operation);
                glib::idle_add_local(glib::clone!(
                    #[strong]
                    update,
                    move || Self::thread(&builder, &update)
                ));
                debug!("activate end");
            }
        ));

        app.connect_shutdown(glib::clone!(move |_| {
            debug!("shutdown begin");
            operation.send(Operation::BackendStop).unwrap();
            debug!("shutdown end");
        }));

        Self { app }
    }

    pub fn run(&self) {
        self.app.run();
    }

    fn build_ui(app: &Application, builder: &Builder, operation: &Sender<Operation>) {
        resources_register_include!("resources.gresource").unwrap();

        //let builder = Builder::new();
        builder
            .add_from_resource("/com/github/tbuen/audio-gtk/ui/window_main.ui")
            .unwrap();
        builder
            .add_from_resource("/com/github/tbuen/audio-gtk/ui/window_stats.ui")
            .unwrap();

        let window_main: ApplicationWindow = builder.object("window_main").unwrap();
        window_main.set_application(Some(app));
        window_main.present();

        /*builder
        .object::<Button>("button_about")
        .unwrap()
        .connect_clicked(glib::clone!(
            #[strong]
            builder,
            move |_| {
                builder
                    .add_from_resource("/com/github/tbuen/audio-gtk/ui/window_about.ui")
                    .unwrap();
                let window_about: AboutWindow = builder.object("window_about").unwrap();
                window_about.set_application_name(env!("CARGO_PKG_NAME"));
                window_about.set_version(&format!(
                    "{} - backend {}",
                    env!("VERSION"),
                    backend::VERSION
                ));
                window_about.present();
            }
        ));*/

        //let listbox: ListBox = builder.object("listbox_files").unwrap();
        //let model = ListStore::new(FileObject::static_type());
        /*let model = ListStore::new();
        listbox.bind_model(
            Some(&model),
            clone!(
                #[strong]
                builder,
                #[strong]
                model,
                #[strong]
                backend,
                move |obj| {
                    let row = ActionRow::builder().activatable(true).build();
                    row.connect_activated(clone!(
                        #[strong]
                        obj,
                        #[strong]
                        builder,
                        #[strong]
                        model,
                        #[strong]
                        backend,
                        move |_| {
                            if obj.property::<bool>("dir") {
                                println!("Activated directory: {}", obj.property::<String>("name"));
                                backend.fileview_change(ChangeDirection::ToChild(
                                    &obj.property::<String>("name"),
                                ));
                                refresh_list(&builder, &model, &backend);
                            } else {
                                println!("Activated file: {}", obj.property::<String>("name"));
                            }
                        }
                    ));
                    obj.bind_property("name", &row, "title")
                        .flags(BindingFlags::SYNC_CREATE)
                        .build();
                    row.into()
                }
            ),
        );*/

        /*builder
        .object::<Button>("button_stats")
        .unwrap()
        .connect_clicked(clone!(
            #[strong]
            builder,
            move |_| {
                builder.object::<Window>("window_stats").unwrap().present();
            }
        ));*/

        builder
            .object::<Button>("button_reload")
            .unwrap()
            .connect_clicked(glib::clone!(
                #[strong]
                operation,
                move |_| {
                    operation.send(Operation::SyncFiles).unwrap();
                }
            ));

        /*builder
        .object::<Button>("button_up")
        .unwrap()
        .connect_clicked(clone!(
            #[strong]
            builder,
            //#[strong]
            //model,
            #[strong]
            backend,
            move |_| {
                backend.fileview_change(ChangeDirection::ToParent);
                //refresh_list(&builder, &model, &backend);
                refresh_list(&builder, &backend);
            }
        ));*/

        /*idle_add(clone!(
            #[strong]
            builder,
            move || {
                /*let rec = receiver.lock().unwrap().unwrap();
                }*/

                ControlFlow::Continue
            }
        ));*/
    }

    fn thread(builder: &Builder, update: &Receiver<Update>) -> ControlFlow {
        let mut ret = ControlFlow::Continue;

        if let Ok(update) = update.try_recv() {
            match update {
                Update::Connected => {
                    builder
                        .object::<Button>("button_stats")
                        .unwrap()
                        .set_icon_name("network-transmit-receive-symbolic");
                    builder
                        .object::<Button>("button_reload")
                        .unwrap()
                        .set_sensitive(true);
                }
                /*Event::InfoAbout(_ver) => {
                    println!("received version *******");
                }
                Event::FileSync(s) => match s {
                    FileSync::Started => {
                        builder
                            .object::<Button>("button_reload")
                            .unwrap()
                            .set_sensitive(false);
                        builder
                            .object::<ProgressBar>("progressbar")
                            .unwrap()
                            .pulse();
                        refresh_list(&builder, &model, &backend);
                    }
                    /*FileSync::Step(None) => {
                        builder
                            .object::<ProgressBar>("progressbar")
                            .unwrap()
                            .pulse();
                    }
                    FileSync::Step(Some((synced, total))) => {
                        builder
                            .object::<ProgressBar>("progressbar")
                            .unwrap()
                            .set_fraction(synced as f64 / total as f64);
                    }*/
                    FileSync::Completed | FileSync::Aborted => {
                        builder
                            .object::<Button>("button_reload")
                            .unwrap()
                            .set_sensitive(true);
                        builder
                            .object::<ProgressBar>("progressbar")
                            .unwrap()
                            .set_fraction(0.0);
                        refresh_list(&builder, &model, &backend);
                    }
                },*/
                Update::Disconnected => {
                    builder
                        .object::<Button>("button_stats")
                        .unwrap()
                        .set_icon_name("network-error-symbolic");
                    builder
                        .object::<Button>("button_reload")
                        .unwrap()
                        .set_sensitive(false);
                    //builder
                    //    .object::<ProgressBar>("progressbar")
                    //    .unwrap()
                    //    .set_fraction(0.0);
                }
                Update::Quit => ret = ControlFlow::Break,
                /*Event::Error(e) => {
                    builder
                        .object::<ToastOverlay>("toast_overlay")
                        .unwrap()
                        .add_toast(Toast::builder().title(e.to_string()).build());
                }*/
                //_ => {}
            }
        }
        ret
    }
}

/*
//mod file_object;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

use adw::prelude::*;
use adw::{AboutWindow, ActionRow, Application, ApplicationWindow, Toast, ToastOverlay, Window};
use backend::{Backend, ChangeDirection, Event, FileSync};
//use file_object::FileObject;
use gio::{ListStore, resources_register_include};
use glib::{BindingFlags, ControlFlow, MainContext, clone, idle_add};
use gtk::{Builder, Button, Label, ListBox, ProgressBar};

// TODO log

// TODO direkt den clone vom Receiver als Parameter, oder noch besser, den Receiver erst in der Funktion vom Backend holen

fn refresh_list(builder: &Builder, /*model: &ListStore,*/ backend: &Backend) {
    let current_dir = backend
        .fileview_current()
        .last()
        .cloned()
        .unwrap_or_default();
    builder
        .object::<Label>("label_dir")
        .unwrap()
        .set_label(&current_dir);
    builder
        .object::<Button>("button_up")
        .unwrap()
        .set_sensitive(!&current_dir.is_empty());

    let dir_content = backend.fileview_content();
    println!("Display now the following file list: {:?}", dir_content);
    //model.remove_all();
    //model.append(&FileObject::new(entry));
    //for entry in dir_content {
    //    model.append(&FileObject::new(entry));
    //}
}
*/
