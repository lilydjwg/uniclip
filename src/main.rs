use structopt::StructOpt;

mod common;
mod wayland;
mod x11;

use common::{Clipboard, Selection};

#[derive(StructOpt)]
#[structopt(name = "uniclip", about = "Use the clipboard on X11 and Wayland the same way.")]
struct Opt {
  /// Operate on the CLIPBOARD selection instead of PRIMARY
  #[structopt(short, long)]
  clipboard: bool,

  /// Specify the content MIME type
  #[structopt(short, long = "type")]
  ty: Option<String>,

  /// Copy (Input) (default if stdin is a tty but stdout is not a tty)
  #[structopt(short, long)]
  input: bool,

  /// Paste (Output) (default otherwise)
  #[structopt(short, long)]
  output: bool,

  /// List MIME types
  #[structopt(short, long)]
  list: bool,
}

enum Mode {
  Input,
  Output,
  Unspecified,
}

fn run<T: Clipboard>(opt: &Opt) {
  let clip = T::new(
    if opt.clipboard { Selection::Clipboard } else { Selection::Primary }
  );

  if opt.list {
    clip.list();
    return;
  }

  if opt.input && opt.output {
    panic!("both input and output options are specified");
  }
  let mut mode = if opt.input {
    Mode::Input
  } else if opt.output {
    Mode::Output
  } else {
    Mode::Unspecified
  };

  if let Mode::Unspecified = mode {
    let stdin_atty = atty::is(atty::Stream::Stdin);
    let stdout_atty = atty::is(atty::Stream::Stdout);
    if !stdin_atty && stdout_atty {
      mode = Mode::Input;
    } else {
      mode = Mode::Output;
    }
  }

  match mode {
    Mode::Input => clip.copy(opt.ty.as_deref()),
    Mode::Output => clip.paste(opt.ty.as_deref()),
    _ => unreachable!(),
  }
}

fn main() {
  let opt = Opt::from_args();

  let sesstype = std::env::var("XDG_SESSION_TYPE").expect("unknown session type");
  if sesstype == "wayland" {
    run::<wayland::WlPaste>(&opt);
  } else if sesstype == "x11" {
    run::<x11::Xclip>(&opt);
  } else {
    panic!("unknown session type");
  }
}
