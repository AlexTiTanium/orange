extern crate gl_generator;

#[allow(dead_code)]
use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
  let dest = env::var("OUT_DIR").unwrap();
  let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

  // TODO: use debug || prod detection here
  Registry::new(Api::Gl, (4, 1), Profile::Core, Fallbacks::All, [])
    .write_bindings(StructGenerator, &mut file)
    //.write_bindings(DebugStructGenerator, &mut file)
    .unwrap();
}
