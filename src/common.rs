use std::process::Command;

pub enum Selection {
  Primary,
  Clipboard,
}

pub trait Clipboard {
  fn new(which: Selection) -> Self;
  fn list(&self);
  fn copy(&self, ty: Option<&str>);
  fn paste(&self, ty: Option<&str>);
}

pub fn run_cmd(cmd: &[&str]) {
  Command::new(cmd[0])
    .args(&cmd[1..])
    .status()
    .expect("failed to run command");
}
