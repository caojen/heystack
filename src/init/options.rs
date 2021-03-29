use ::std::env;

pub struct Options {
  pub help: bool,     // show help page
  pub start: bool,    // start service
  pub reload: bool,   // reload service
  pub show: bool,     // show the basic config files, basic env, etc
  pub stop: bool,     // stop service
  pub unknown: Vec<String> // unknown options
}

impl Options {
  pub fn new() -> Self {
    Options {
      help: false,
      start: false,
      reload: false,
      show: false,
      stop: false,
      unknown: Vec::new()
    }
  }
}

pub fn get_options() -> Options {
  let args: Vec<String> = env::args().collect();

  let mut options = Options::new();

  for arg in args {
    match &arg[..] {
      "h" | "help" => options.help = true,
      "start" => options.start = true,
      "reload" => options.reload = true,
      "show" => options.reload = true,
      "stop" => options.reload = true,
      _ => options.unknown.push(arg)
    }
  }

  options
}
