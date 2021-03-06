use radius::Config;
use radius::Error;

pub fn config() -> Result<Config, Error> {
    let path = std::env::current_dir()?;
    let path = path.join("../tests/config.toml");
    Config::read_file(path)
}
