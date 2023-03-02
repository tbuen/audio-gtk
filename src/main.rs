use adw::gio::{resources_register, Menu, Resource, SimpleAction};
use adw::glib::{clone, Bytes};
use adw::gtk::{Box, Button, MenuButton, Orientation};
use adw::prelude::*;
use adw::{AboutWindow, Application, ApplicationWindow, HeaderBar};

fn main() {
    let app = Application::builder()
        .application_id("com.github.tbuen.audio-gtk")
        .build();

    app.connect_activate(activate);

    app.run();
}

fn activate(app: &Application) {
    let resources_bytes = include_bytes!("../resources/resources.gresource");
    let resource_data = Bytes::from(&resources_bytes[..]);
    let res = Resource::from_data(&resource_data).unwrap();
    resources_register(&res);

    let menu = Menu::new();
    menu.append(Some("Device"), None);
    menu.append(Some("Info"), Some("win.info"));
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();

    let header_bar = HeaderBar::builder().build();
    header_bar.pack_end(&menu_button);

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

    let action_info = SimpleAction::new("info", None);
    action_info.connect_activate(clone!(@weak app, @weak window => move |_, _| {
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

    window.add_action(&action_info);

    window.present();
}
