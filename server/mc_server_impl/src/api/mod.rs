pub mod config;

use rjni::{JavaVM, Version, Classpath, Options, Class, Type, Object};
use std::path::Path;

pub struct JavaAPI {
    pub jvm: JavaVM
}

impl JavaAPI {
    pub fn init() -> JavaAPI {
        let classpath = Classpath::new()
            .add(&Path::new("server.jar"));

        let opts = Options::new()
            .version(Version::V18)
            .classpath(classpath);

        let jvm: JavaVM = JavaVM::new(opts).unwrap();

        JavaAPI { jvm }
    }
}