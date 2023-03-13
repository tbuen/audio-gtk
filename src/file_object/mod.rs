use backend::DirEntry;
use glib::Object;
use serde::{Deserialize, Serialize};

mod imp;

glib::wrapper! {
    pub struct FileObject(ObjectSubclass<imp::FileObject>);
}

impl FileObject {
    pub fn new(entry: DirEntry) -> Self {
        match entry {
            DirEntry::Dir(name) => Object::builder()
                .property("dir", true)
                .property("name", name)
                .build(),
            DirEntry::File(name) => Object::builder()
                .property("dir", false)
                .property("name", name)
                .build(),
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FileData {
    dir: bool,
    name: String,
}
