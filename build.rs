use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::PathBuf;

fn main() {
  let profile = env::var("PROFILE").unwrap(); // "release" for release builds, "debug" for other builds

  if profile == "release" {
    copy_resources();
  }
}

fn copy_resources() {
  let dest = env::var("CARGO_MANIFEST_DIR").unwrap();
  let from = env::var("CARGO_MANIFEST_DIR").unwrap();
  let profile = env::var("PROFILE").unwrap();

  //Initialize default values for CopyOptions
  let mut options = CopyOptions::new();
  options.overwrite = true;
  // options.mirror_copy = true; // To mirror copy the whole structure of the source directory

  let mut dest_path = PathBuf::from(dest);
  dest_path.push("target");
  dest_path.push(profile);

  let mut from_path = PathBuf::from(from);
  from_path.push("resources");

  // copy source/dir1 to target/dir1
  copy(from_path.as_path(), dest_path.as_path(), &options).unwrap();
}
