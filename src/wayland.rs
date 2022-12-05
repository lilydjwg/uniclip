use super::common::{Clipboard, Selection, run_cmd};

pub struct WlPaste {
  sel: Selection,
}

impl Clipboard for WlPaste {
  fn new(which: Selection) -> Self {
    WlPaste { sel: which }
  }

  fn list(&self) {
    let cmd = match self.sel {
      Selection::Primary => &["wl-paste", "--list-types", "--primary"][..],
      Selection::Clipboard => &["wl-paste", "--list-types"][..],
    };

    run_cmd(cmd);
  }

  fn copy(&self, ty: Option<&str>) {
    let mut cmd = vec!["wl-copy"];
    if let Selection::Primary = self.sel {
      cmd.push("--primary");
    }
    if let Some(t) = ty {
      cmd.extend(["--type", t]);
    }

    run_cmd(&cmd);
  }

  fn paste(&self, ty: Option<&str>) {
    let mut cmd = vec!["wl-paste", "--no-newline"];
    if let Selection::Primary = self.sel {
      cmd.push("--primary");
    }
    if let Some(t) = ty {
      cmd.extend(["--type", t]);
    }

    run_cmd(&cmd);
  }
}
