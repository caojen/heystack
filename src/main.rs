mod diskio;
mod init;
mod config;
mod master;
mod storage;

#[macro_use] mod log;

use ::std::io;

fn main() -> io::Result<()> {
    let options = init::options::get_options();
    init::start::deal_with_options(&options)?;

    Ok(())
}
