use ::std::env;

pub struct Options {
  pub help: bool,     // show help page
  pub start: bool,    // start service
  pub reload: bool,   // reload service
  pub show: bool,     // show the basic config files, basic env, etc
  pub stop: bool,     // stop service
  pub unknown: Vec<String>, // unknown options
  pub empty: bool     // no option
}

impl Options {
  pub fn new() -> Self {
    Options {
      help: false,
      start: false,
      reload: false,
      show: false,
      stop: false,
      unknown: Vec::new(),
      empty: false
    }
  }
}

pub fn get_options() -> Options {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0); // remove the program name

  let mut options = Options::new();
  if args.len() == 0 {
    options.empty = true;
  }

  for arg in args {
    match &arg[..] {
      "h" | "help" => options.help = true,
      "s" | "start" => options.start = true,
      "r" | "reload" => options.reload = true,
      "show" => options.reload = true,
      "stop" => options.reload = true,
      _ => options.unknown.push(arg)
    }
  }

  options
}
