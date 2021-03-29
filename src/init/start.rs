use super::options;

pub fn deal_with_options(option: &options::Options) {
  if option.unknown.len() != 0 {
    crate::log!("Unknown option(s): ");
    for op in &option.unknown {
      crate::log!(op);
    }
    crate::logln!("\nTry h or help for more information");
  } else if option.empty {
    crate::logln!("No argument detected.");
    crate::logln!("Try h or help for more information");
  } else if option.help {
    show_usage();
  } else if option.start {

  } else if option.stop {

  } else if option.reload {

  } else if option.show {
    
  }

  unimplemented!("");
}

fn show_usage() {
  crate::logln!("HeyStack\n");
  crate::logln!("Arguments:");
  crate::logln!("  h, help     Show the usage page");
  crate::logln!("  s, start    Start the HeyStack service");
  crate::logln!("  show        Show the config file");
  crate::logln!("  r, reload   Reload program from config file");
  crate::logln!("  stop        Stop the program");
}