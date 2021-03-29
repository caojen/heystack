mod diskio;
mod init;

#[macro_use]
mod log;

fn main() {
    let options = init::options::get_options();
}
