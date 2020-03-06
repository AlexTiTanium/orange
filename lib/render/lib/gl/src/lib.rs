pub mod gl {
    //include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    //include!("./gl-41-debug.rs");
    include!("./gl-41-global.rs");
}

pub fn debug() {
    // println!("{:?}", gl::Gl::);
}
