extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate linked_hash_set;
extern crate rand;

use clap::{App, Arg};
use configuration::Configuration;
use iota::Iota;
use tangle::Tangle;
use transaction_requester::TransactionRequester;
use configuration::DefaultConfSettings;
use tips_view_model::TipsViewModel;

pub mod configuration;
pub mod iota;
pub mod tangle;
pub mod txhash;
pub mod curl;
pub mod zmq_wrapper;
pub mod tips_view_model;
pub mod transaction_requester;

const APP_NAME : &'static str = "IRustI";
const VERSION : &'static str = "1.4.2.4";
const AUTHOR : &'static str = "ben75";
const ABOUT : &'static str = "Rust implementation of IOTA protocol";

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Starting {}...",APP_NAME);

    //configure command line parser
    let app :App = App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Config file")
            .required_unless("port")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("tcp port")
            .required_unless("config")
            .takes_value(true))
        .arg(Arg::with_name("udp-receiver-port")
            .short("u")
            .long("udp-receiver-port")
            .value_name("UDP_PORT")
            .help("upd receiver port")
            .takes_value(true))
        .arg(Arg::with_name("tcp-receiver-port")
            .short("t")
            .long("tcp-receiver-port")
            .value_name("TCP_PORT")
            .help("tcp receiver port")
            .takes_value(true))
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .value_name("DEBUG")
            .help("enable debug logs")
            .takes_value(false))
        .arg(Arg::with_name("remote")
            .long("remote")
            .value_name("REMOTE")
            .help("enable remote API")
            .takes_value(false))
        .arg(Arg::with_name("remote-limit-api")
            .long("remote-limit-api")
            .value_name("REMOTE_LIMIT_API")
            .help("remote limit api")
            .takes_value(true))
        .arg(Arg::with_name("remote-auth")
            .long("remote-auth")
            .value_name("REMOTE_AUTH")
            .help("remote authentication")
            .takes_value(true))
        .arg(Arg::with_name("neighbors")
            .short("n")
            .long("neighbors")
            .value_name("NEIGHBORS")
            .help("neighbors list")
            .takes_value(true))
        .arg(Arg::with_name("export")
            .long("export")
            .value_name("EXPORT")
            .help("export")
            .takes_value(false))
        .arg(Arg::with_name("zmqenabled")
            .long("zmq-enabled")
            .value_name("ZMQ_ENABLED")
            .help("enable ZMQ")
            .takes_value(false))
        .arg(Arg::with_name("testnet")
            .long("testnet")
            .value_name("TEST_NET")
            .help("Run in testnet mode")
            .takes_value(false))
        .arg(Arg::with_name("revalidate")
            .long("revalidate")
            .value_name("REVALIDATE")
            .help("Revalidate")
            .takes_value(false))
        .arg(Arg::with_name("rescan")
            .long("rescan")
            .value_name("RESCAN")
            .help("Rescan")
            .takes_value(false))
        .arg(Arg::with_name("send-limit")
            .long("send-limit")
            .value_name("SEND_LIMIT")
            .help("send limit")
            .takes_value(true))
        .arg(Arg::with_name("sync")
            .long("sync")
            .value_name("SYNC")
            .help("Sync")
            .takes_value(false))
        .arg(Arg::with_name("dns-resolution-false")
            .long("dns-resolution-false")
            .value_name("DNS_RESOLUTION_FALSE")
            .help("DNS resolution disabled")
            .takes_value(false))
        .arg(Arg::with_name("max-peers")
            .long("max-peers")
            .value_name("MAX_PEERS")
            .help("max peers")
            .takes_value(true))
        .arg(Arg::with_name("testnet-coordinator")
            .long("testnet-coordinator")
            .value_name("TESTNET_COORDINATOR")
            .help("testnet coordinator")
            .takes_value(true))
        .arg(Arg::with_name("testnet-no-coo-validation")
            .long("testnet-no-coo-validation")
            .value_name("TESTNET_NO_COO_VALIDATION")
            .help("Ignore coordinator validation on testnet")
            .takes_value(false))
        .arg(Arg::with_name("snapshot")
            .long("snapshot")
            .value_name("SNAPSHOT")
            .help("Snapshot index")
            .takes_value(true))
        .arg(Arg::with_name("snapshot-sig")
            .long("snapshot-sig")
            .value_name("SNAPSHOT_SIG")
            .help("Snapshot signature")
            .takes_value(true))
        .arg(Arg::with_name("mwm")
            .long("mwm")
            .value_name("MWM")
            .help("Minimum magnitude weight (testnet only)")
            .takes_value(true))
        .arg(Arg::with_name("milestone-start")
            .long("milestone-start")
            .value_name("MILESTONE_START")
            .help("Milestone start index")
            .takes_value(true))
        .arg(Arg::with_name("milestone-keys")
            .long("milestone-keys")
            .value_name("MILESTONE_KEYS")
            .help("Milestone keys")
            .takes_value(true))
        .arg(Arg::with_name("snapshot-timestamp")
            .long("snapshot-timestamp")
            .value_name("SNAPSHOT_TIMESTAMP")
            .help("Snapshot timestamp")
            .takes_value(true));


    //create a configuration based on default values and config file
    let configuration = Configuration::new(app);

    if configuration.get_param(configuration::DefaultConfSettings::NEIGHBORS).is_some(){
        warn!("No neighbor has been specified. Server starting nodeless.");
    }

    if configuration.get_param(configuration::DefaultConfSettings::RemoteLimitApi).is_some(){
        debug!("The following api calls are not allowed : {:?} ", configuration.get_param(configuration::DefaultConfSettings::RemoteLimitApi).unwrap());
    }

    if configuration.booling_param(configuration::DefaultConfSettings::RemoteAuth){
        debug!("Remote access requires basic authentication");
    }

    if configuration.booling_param(configuration::DefaultConfSettings::ApiHost){
        info!("Remote access enabled. Binding API socket to listen any interface.");
    }

    if configuration.booling_param(configuration::DefaultConfSettings::EXPORT){
        info!("Export transaction trytes turned on.");
    }

    if configuration.integer_param(configuration::DefaultConfSettings::PORT) < 1024 {
        warn!("Warning: api port value seems too low.");
    }


    if configuration.booling_param(configuration::DefaultConfSettings::TESTNET){
        info!("Use Testnet !");
    }

    if configuration.booling_param(configuration::DefaultConfSettings::DEBUG){
        info!("Debug mode turned on.");
        configuration.print();
    }

    if configuration.get_param(configuration::DefaultConfSettings::COORDINATOR).is_some() {
        if !configuration.booling_param(configuration::DefaultConfSettings::TESTNET){
            warn!("coordinator-address is ignored. (it requires the --testnet flag)");
        }
    }

    if configuration.booling_param(configuration::DefaultConfSettings::DontValidateTestnetMilestoneSig){
        if !configuration.booling_param(configuration::DefaultConfSettings::TESTNET){
            warn!("testnet-no-coo-validation is ignored. (it requires the --testnet flag)");
        }
    }

    let tangle = createTangle(configuration.get_param(DefaultConfSettings::DbPath).unwrap(),
                              configuration.get_flag(DefaultConfSettings::TESTNET));
    let iota = Iota::new(configuration);
    let mut tips_view_model = TipsViewModel::new();
    let transaction_requester = TransactionRequester::new(tips_view_model);

    iota.shutdown();
    tangle.shutdown();
}

fn createTangle(db_path :String, is_testnet :bool) -> Tangle {
    //Database init
    let mut effective_db_path = db_path;
    if is_testnet {
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
    Tangle::new(effective_db_path)
}
