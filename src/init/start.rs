use ::std::io;

use super::options;
use crate::config::Config;

pub fn deal_with_options(option: &options::Options) -> io::Result<()> {
  if option.unknown.len() != 0 {
    crate::log!("Unknown option(s): ");
    for op in &option.unknown {
      crate::log!(op);
    }
    crate::logln!("\nTry h or help for more information");
    panic!("");
  } else if option.empty {
    crate::logln!("No argument detected.");
    crate::logln!("Try h or help for more information");
    panic!("");
  } else if option.help {
    show_usage();
    Ok(())
  } else if option.start {
    let config = Config::new()?;
    if config.is_started() {
      crate::logln!("The service is started at pid ", config.tpid);
      panic!("");
    }

    Ok(())
  } else if option.stop {
    unimplemented!("stop is ub");
  } else if option.reload {
    unimplemented!("reload is ub");
  } else if option.show {
    let config = Config::new()?;
    crate::logln!("Started: ", config.is_started());
    crate::logln!("Pid: ", config.tpid);
    crate::logln!("Pid File: ", config.pid_file);
    crate::logln!("Physical Volume: ", config.volume_name);
    crate::logln!("Index File: ", config.index_name);
    crate::logln!("Config Port: ", config.config_port);
    crate::logln!("Service Port: ", config.service_port);
    crate::logln!("Max Index Mem: ", config.max_index_in_mem);

    Ok(())
  } else {
    unreachable!("");
  }

  // unimplemented!("");
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