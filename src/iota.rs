
use configuration;
use configuration::Configuration;
use configuration::DefaultConfSettings;
use tangle::Tangle;

pub struct Iota {
    configuration: Configuration,
    tangle :Tangle,

}

impl Iota {

    pub fn new(configuration: Configuration) -> Iota {
        if Configuration::get_conf_flag(&configuration, DefaultConfSettings::TESTNET.to_string().as_ref()) {
            let mut coo_address = configuration::MAINNET_COORDINATOR_ADDRESS.to_string();
            let tmp = Configuration::get_param(&configuration, DefaultConfSettings::COORDINATOR);
            coo_address = match tmp {
                Some(x) => x,
                _ => configuration::TESTNET_COORDINATOR_ADDRESS.to_string(),
            };
            if coo_address.eq(configuration::TESTNET_COORDINATOR_ADDRESS) {
                warn!("No coordinator address given for testnet. Defaulting to {}",configuration::TESTNET_COORDINATOR_ADDRESS);
            }
        }
        let mut effective_db_path = configuration.get_param(DefaultConfSettings::DbPath).unwrap();
        if configuration.get_flag(DefaultConfSettings::TESTNET) {
            if effective_db_path.eq(&"mainnetdb".to_string()) {
                warn!("Enforce use of testnetdb on test net");
                effective_db_path = "testnetdb".to_string();
            }
        }else{
            if effective_db_path.eq(&"testnetdb".to_string()) {
                warn!("Enforce use of mainnetdb on main net");
                effective_db_path = "mainnetdb".to_string();
            }
        }
        let tangle = Tangle::new(effective_db_path);

        Iota{configuration, tangle}
    }

    pub fn shutdown(db_path :String){
        Tangle::shutdown(db_path);
    }
}