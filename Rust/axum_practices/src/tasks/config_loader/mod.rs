pub mod services;
use config::Config;
use lazy_static::lazy_static;
use services::config_loader::config_loader;
lazy_static! {
    #[derive(Debug)]
   pub static ref CONFIGURATIONS: Config = config_loader().unwrap();
}
pub fn config_loader_main() {
    println!(
        "{:#?}",
        CONFIGURATIONS
            .get::<String>("couchbase.connectionurl")
            .unwrap()
    );
    println!(
        "{:#?}",
        CONFIGURATIONS.get::<String>("couchbase.username").unwrap()
    );
}
