use adw::prelude::*;
use adw::{AboutWindow, ActionRow, Application, ApplicationWindow, Window};
use backend::{Backend, Event};
use file_object::FileObject;
use gio::{resources_register_include, ListStore};
use glib::{clone, BindingFlags, MainContext, PRIORITY_DEFAULT};
use gtk::{Builder, Button, Label, ListBox};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

mod file_object;

fn main() {
    let backend = Rc::new(RefCell::<Option<Backend>>::new(None));
    let receiver = Arc::new(Mutex::<Option<Receiver<Event>>>::new(None));

    let app = Application::builder()
        .application_id("com.github.tbuen.audio-gtk")
        .build();

    app.connect_startup(clone!(@strong backend, @strong receiver => move |_| {
        println!("startup begin");
        let (b, r) = Backend::new();
        backend.borrow_mut().replace(b);
        receiver.lock().unwrap().replace(r);
        println!("startup end");
    }));

    app.connect_activate(clone!(@strong backend, @strong receiver => move |app| {
        println!("activate begin");
        build_ui(app, backend.clone(), receiver.clone());
        println!("activate end");
    }));

    app.connect_shutdown(clone!(@strong backend, @strong receiver => move |_| {
        println!("shutdown begin");
        backend.borrow_mut().take();
        receiver.lock().unwrap().take();
        println!("shutdown end");
    }));

    app.run();

    println!("Exiting gracefully...");
}

fn build_ui(
    app: &Application,
    backend: Rc<RefCell<Option<Backend>>>,
    receiver: Arc<Mutex<Option<Receiver<Event>>>>,
) {
    resources_register_include!("resources.gresource").unwrap();

    let builder = Builder::new();
    builder
        .add_from_resource("/com/github/tbuen/audio-gtk/ui/window_main.ui")
        .unwrap();
    builder
        .add_from_resource("/com/github/tbuen/audio-gtk/ui/window_stats.ui")
        .unwrap();

    let window_main: ApplicationWindow = builder.object("window_main").unwrap();
    window_main.set_application(Some(app));
    window_main.present();

    builder.object::<Button>("button_about").unwrap().connect_clicked(clone!(@strong builder => move |_| {
        builder.add_from_resource("/com/github/tbuen/audio-gtk/ui/window_about.ui").unwrap();
        let window_about: AboutWindow = builder.object("window_about").unwrap();
        window_about.set_application_name(env!("CARGO_PKG_NAME"));
        window_about.set_version(&format!("{} - backend {}", env!("VERSION"), backend::VERSION));
        window_about.present();
    }));

    let listbox: ListBox = builder.object("listbox_files").unwrap();
    let model = ListStore::new(FileObject::static_type());
    listbox.bind_model(Some(&model), clone!(@strong builder, @strong model, @strong backend => move |obj| {
        let row = ActionRow::builder().activatable(true).build();
        row.connect_activated(clone!(@strong obj, @strong builder, @strong model, @strong backend => move |_| {
            if obj.property::<bool>("dir") {
                println!("Activated directory: {}", obj.property::<String>("name"));
                if let Some(b) = &*backend.borrow_mut() {
                    b.dir_enter(&obj.property::<String>("name"));
                    refresh_list(&builder, &model, b);
                }
            } else {
                println!("Activated file: {}", obj.property::<String>("name"));
            }
        }));
        obj.bind_property("name", &row, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        row.into()
    }));

    builder
        .object::<Button>("button_stats")
        .unwrap()
        .connect_clicked(clone!(@strong builder => move |_| {
            builder.object::<Window>("window_stats").unwrap().present();
        }));

    builder
        .object::<Button>("button_up")
        .unwrap()
        .connect_clicked(
            clone!(@strong builder, @strong model, @strong backend => move |_| {
                if let Some(b) = &*backend.borrow_mut() {
                    b.dir_up();
                    refresh_list(&builder, &model, b);
                }
            }),
        );

    let (gtk_sender, gtk_receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let receiver = receiver.clone();
    thread::spawn(move || {
        if let Some(r) = &*receiver.lock().unwrap() {
            loop {
                if let Ok(evt) = r.recv() {
                    gtk_sender.send(evt).unwrap();
                } else {
                    break;
                }
            }
        }
        println!("spawni exit");
    });

    gtk_receiver.attach(
        None,
        clone!(@strong builder, @strong backend => @default-return Continue(false),
            move |evt| {
                match evt {
                    Event::Connected  => {
                        builder.object::<Button>("button_stats").unwrap().set_icon_name("network-idle-symbolic");
                    }
                    Event::Version(_ver) => {
                        println!("received version *******");
                    }
                    Event::Synchronized => {
                        builder.object::<Button>("button_stats").unwrap().set_icon_name("network-transmit-receive-symbolic");
                        if let Some(b) = &*backend.borrow_mut() {
                            refresh_list(&builder, &model, b);
                        }
                    }
                    Event::Disconnected => {
                        builder.object::<Button>("button_stats").unwrap().set_icon_name("network-error-symbolic");
                    }
                }
                Continue(true)
            }
        ),
    );
}

fn refresh_list(builder: &Builder, model: &ListStore, backend: &Backend) {
    let current_dir = &backend.current_dir();
    builder
        .object::<Label>("label_dir")
        .unwrap()
        .set_label(current_dir);
    builder
        .object::<Button>("button_up")
        .unwrap()
        .set_sensitive(!current_dir.is_empty());

    let dir_content = backend.dir_content();
    println!("Display now the following file list: {:?}", dir_content);
    model.remove_all();
    for entry in dir_content {
        model.append(&FileObject::new(entry));
    }
}
