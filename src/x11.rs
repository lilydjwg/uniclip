use super::common::{Clipboard, Selection, run_cmd};

pub struct Xclip {
  sel: Selection,
}

impl Clipboard for Xclip {
  fn new(which: Selection) -> Self {
    Xclip { sel: which }
  }

  fn list(&self) {
    let mut cmd = vec!["xclip", "-o", "-t", "TARGETS"];
    cmd.extend(match self.sel {
      Selection::Primary => &["-selection", "primary"],
      Selection::Clipboard => &["-selection", "clipboard"],
    });

    run_cmd(&cmd);
  }

  fn copy(&self, ty: Option<&str>) {
    let mut cmd = vec!["xclip", "-i"];
    cmd.extend(match self.sel {
      Selection::Primary => &["-selection", "primary"],
      Selection::Clipboard => &["-selection", "clipboard"],
    });
    if let Some(t) = ty {
      cmd.extend(&["-t", t]);
    }

    run_cmd(&cmd);
  }

  fn paste(&self, ty: Option<&str>) {
    let mut cmd = vec!["xclip", "-o"];
    cmd.extend(match self.sel {
      Selection::Primary => &["-selection", "primary"],
      Selection::Clipboard => &["-selection", "clipboard"],
    });
    if let Some(t) = ty {
      cmd.extend(&["-t", t]);
    }

    run_cmd(&cmd);
  }
}
