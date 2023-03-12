use glib::Object;
use serde::{Deserialize, Serialize};

mod imp;

glib::wrapper! {
    pub struct FileObject(ObjectSubclass<imp::FileObject>);
}

impl FileObject {
    pub fn new(name: String) -> Self {
        Object::builder().property("name", name).build()
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FileData {
    name: String,
}
