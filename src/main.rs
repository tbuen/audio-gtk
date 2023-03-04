use adw::gio::{resources_register, Resource, SimpleAction};
use adw::glib::{clone, Bytes, MainContext, PRIORITY_DEFAULT};
use adw::gtk::{Box, Button, Label, Orientation};
use adw::prelude::*;
use adw::{AboutWindow, Application, ApplicationWindow, HeaderBar, Window, WindowTitle};
use backend::{Backend, Event};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let backend = Rc::new(RefCell::<Option<Backend>>::new(None));
    let receiver = Arc::new(Mutex::<Option<Receiver<Event>>>::new(None));

    let app = Application::builder()
        .application_id("com.github.tbuen.audio-gtk")
        .build();

    app.connect_startup(clone!(@weak backend, @weak receiver => move |_| {
        println!("startup begin");
        let (b, r) = Backend::new();
        backend.borrow_mut().replace(b);
        receiver.lock().unwrap().replace(r);
        println!("startup end");
    }));

    app.connect_activate(clone!(@weak backend, @weak receiver => move |app| {
        println!("activate begin");
        build_ui(app, &*backend.borrow_mut(), receiver.clone());
        println!("activate end");
    }));

    app.connect_shutdown(clone!(@weak backend, @weak receiver => move |_| {
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
    _backend: &Option<Backend>,
    receiver: Arc<Mutex<Option<Receiver<Event>>>>,
) {
    let resources_bytes = include_bytes!("../resources/resources.gresource");
    let resource_data = Bytes::from(&resources_bytes[..]);
    let res = Resource::from_data(&resource_data).unwrap();
    resources_register(&res);

    let about_button = Button::builder()
        .icon_name("help-about-symbolic")
        .action_name("win.about")
        .build();

    let settings_button = Button::builder()
        .icon_name("emblem-system-symbolic")
        .sensitive(false)
        .build();

    let connection_button = Button::builder()
        .icon_name("network-offline-symbolic")
        .action_name("win.stats")
        .build();

    let volume_button = Button::builder()
        .icon_name("audio-volume-high-symbolic")
        .tooltip_text("50%")
        .build();

    let header_bar = HeaderBar::builder().build();

    header_bar.pack_end(&about_button);
    header_bar.pack_end(&settings_button);
    header_bar.pack_start(&connection_button);
    header_bar.pack_start(&volume_button);

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello World!");
    });

    let vbox = Box::builder().orientation(Orientation::Vertical).build();

    vbox.append(&header_bar);
    vbox.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(env!("CARGO_PKG_NAME"))
        .content(&vbox)
        .default_width(800)
        .default_height(600)
        .build();

    let action_about = SimpleAction::new("about", None);

    action_about.connect_activate(clone!(@weak app, @weak window => move |_, _| {
        AboutWindow::builder()
            .application(&app)
            .transient_for(&window)
            .resizable(false)
            .application_icon("audio")
            .application_name(env!("CARGO_PKG_NAME"))
            .version(format!("{} - backend {}", env!("VERSION"), backend::VERSION))
            .developer_name("Thomas Büning")
            .website("https://github.com/tbuen/audio-gtk")
            .build()
            .present();
    }));

    window.add_action(&action_about);

    let action_stats = SimpleAction::new("stats", None);

    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::builder().title("disconnected").build())
        .build();

    let label_client_cnt = Label::builder()
        .label("connected clients: 0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let vbox = Box::builder().orientation(Orientation::Vertical).build();

    vbox.append(&header_bar);
    vbox.append(&label_client_cnt);

    let window_stats = Window::builder()
        .hide_on_close(true)
        .title("Info")
        .content(&vbox)
        .build();

    action_stats.connect_activate(clone!(@weak window_stats => move |_,_| {
        window_stats.present();
    }));

    window.add_action(&action_stats);

    window.present();

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
        clone!(@weak connection_button, @weak header_bar => @default-return Continue(false),
            move |evt| {
                match evt {
                    Event::Connected  => {
                        connection_button.set_icon_name("network-idle-symbolic");
                    }
                    Event::Version(ver) => {
                        println!("received version *******");
                        if let Some(w) = header_bar.title_widget() {
                            if let Ok(t) = w.downcast::<WindowTitle>() {
                                t.set_title(&ver.project);
                                t.set_subtitle(&ver.version);
                            }
                            else {
                                println!("no windowtitle :(");
                            }
                        } else {
                            println!("no title widget");
                        }
                    }
                    Event::Synchronized => {
                        connection_button.set_icon_name("network-transmit-receive-symbolic");
                    }
                    Event::Disconnected => {
                        connection_button.set_icon_name("network-offline-symbolic");
                        if let Some(w) = header_bar.title_widget() {
                            if let Ok(t) = w.downcast::<WindowTitle>() {
                                t.set_title("disconnected");
                                t.set_subtitle("");
                            }
                            else {
                                println!("no windowtitle :(");
                            }
                        } else {
                            println!("no title widget");
                        }
                    }
                }
                Continue(true)
            }
        ),
    );
}
