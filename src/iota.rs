
use configuration;
use configuration::Configuration;
use configuration::DefaultConfSettings;
use tangle::Tangle;
use txhash::TxHash;
use zmq_wrapper::MessageQ;
use std::str::FromStr;

pub struct Iota {
    configuration: Configuration,
    tangle :Tangle,

}

impl Iota {

    pub fn new(configuration: Configuration) -> Iota {
        //Coordinator init
        let mut coo_address = configuration::MAINNET_COORDINATOR_ADDRESS.to_string();
        if Configuration::get_conf_flag(&configuration, DefaultConfSettings::TESTNET.to_string().as_ref()) {
            let tmp = Configuration::get_param(&configuration, DefaultConfSettings::COORDINATOR);
            coo_address = match tmp {
                Some(x) => x,
                _ => configuration::TESTNET_COORDINATOR_ADDRESS.to_string(),
            };
            if coo_address.eq(configuration::TESTNET_COORDINATOR_ADDRESS) {
                warn!("No coordinator address given for testnet. Defaulting to {}",configuration::TESTNET_COORDINATOR_ADDRESS);
            }
        }
        let coo_hash = TxHash::from_str(coo_address.as_ref()).unwrap();

        //Database init
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

        //ZMQ
        let message_q = MessageQ::new(
            Configuration::integer_param(&configuration, DefaultConfSettings::ZmqThreads),
            Configuration::stringify_param(&configuration, DefaultConfSettings::ZmqIpc),
            Configuration::integer_param(&configuration, DefaultConfSettings::ZmqPort),
            Configuration::booling_param(&configuration, DefaultConfSettings::ZmqEnabled)
        );


        Iota{configuration, tangle}
    }

    pub fn shutdown(db_path :String){
        Tangle::shutdown(db_path);
    }
}