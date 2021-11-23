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

  /// Copy (Input) (default if stdout is not a tty)
  #[structopt(short, long)]
  input: bool,
}

fn run<T: Clipboard>(opt: &Opt) {
  let clip = T::new(
    if opt.clipboard { Selection::Clipboard } else { Selection::Primary }
  );

  let input = opt.input || atty::is(atty::Stream::Stdout);

  if input {
    clip.copy(opt.ty.as_deref());
  } else {
    clip.paste(opt.ty.as_deref());
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
