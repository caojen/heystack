mod diskio;
mod init;
mod config;

#[macro_use]
mod log;

fn main() {
    let options = init::options::get_options();
    init::start::deal_with_options(&options);
}
