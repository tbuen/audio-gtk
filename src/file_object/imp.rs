use super::FileData;
use adw::prelude::*;
use glib::{ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Default)]
pub struct FileObject {
    pub data: RefCell<FileData>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileObject {
    const NAME: &'static str = "FileObject";
    type Type = super::FileObject;
}

impl ObjectImpl for FileObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoolean::builder("dir").build(),
                ParamSpecString::builder("name").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "dir" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `Boolean`.");
                self.data.borrow_mut().dir = input_value;
            }
            "name" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().name = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "dir" => self.data.borrow().dir.to_value(),
            "name" => self.data.borrow().name.to_value(),
            _ => unimplemented!(),
        }
    }
}
