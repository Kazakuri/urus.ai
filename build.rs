use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        visit_dirs(&path, cb)?;
      } else {
        cb(&entry);
      }
    }
  }
  Ok(())
}

fn run_npm_command(cmd: &str, args: &Vec<&str>, error: &str) {
  let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

  let command = root.join("node_modules").join(".bin").join(cmd);

  let output = if cfg!(target_os = "windows") {
    let mut arguments = vec!["/C", command.to_str().unwrap()];
    arguments.extend(args);

    Command::new("cmd").args(arguments).output().expect(error)
  } else {
    let mut arguments = vec!["-c", command.to_str().unwrap()];
    arguments.extend(args);

    Command::new("sh").args(arguments).output().expect(error)
  };

  println!("{}", String::from_utf8(output.stdout).unwrap());
  println!("{}", String::from_utf8(output.stderr).unwrap());
}

fn main() {
  let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

  run_npm_command(
    "postcss",
    &vec!["style/style.css", ">", "public/style.min.css"],
    "Failed to build minified CSS",
  );

  println!(
    "cargo:rerun-if-changed={}",
    root.join("tailwind.config.js").to_str().unwrap()
  );
  println!(
    "cargo:rerun-if-changed={}",
    root.join("postcss.config.js").to_str().unwrap()
  );

  visit_dirs(&root.join("style"), &|e: &DirEntry| {
    println!("cargo:rerun-if-changed={}", e.path().to_str().unwrap());
  })
  .unwrap();
}
