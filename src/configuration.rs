extern crate ini;
extern crate clap;

use std::collections::HashMap;
use std::str::FromStr;
use self::ini::Ini;
use std::option::Option;
use clap::{App,ArgMatches};

pub const MAINNET_COORDINATOR_ADDRESS: &'static str =
    "KPWCHICGJZXKE9GSUDXZYUAPLHAKAHYHDXNPHENTERYMMBQOPSQIDENXKLKCEYCPVTZQLEEJVYJZV9BWU";
pub const TESTNET_COORDINATOR_ADDRESS: &'static str =
    "EQQFCZBIHRHWPXKMTOLMYUYPCN9XLMJPYZVFJSAY9FQHCCLWTOLLUGKKMXYFDBOOYFBLBI9WUEILGECYM";
const MAINNET_SNAPSHOT_FILE: &'static str = "/snapshotMainnet.txt";
const TESTNET_SNAPSHOT_FILE: &'static str = "/snapshotTestnet.txt";
const MAINNET_SNAPSHOT_SIG_FILE: &'static str = "/snapshotMainnet.sig";

const PREVIOUS_EPOCHS_SPENT_ADDRESSES_TXT: &'static str = "/previousEpochsSpentAddresses.txt";
const PREVIOUS_EPOCH_SPENT_ADDRESSES_SIG: &'static str = "/previousEpochsSpentAddresses.sig";
const MAINNET_MILESTONE_START_INDEX: &'static str = "426550";
const TESTNET_MILESTONE_START_INDEX: &'static str = "434525";
const MAINNET_NUM_KEYS_IN_MILESTONE: &'static str = "20";
const TESTNET_NUM_KEYS_IN_MILESTONE: &'static str = "22";
const GLOBAL_SNAPSHOT_TIME: &'static str = "1525042800";
const TESTNET_GLOBAL_SNAPSHOT_TIME: &'static str = "1522306500";


const MAINNET_MWM: &'static str = "14";
const TESTNET_MWM: &'static str = "9";
const PACKET_SIZE: &'static str = "1650";
const TESTNET_PACKET_SIZE: &'static str = "1653";
const REQ_HASH_SIZE: &'static str = "46";
const TESTNET_REQ_HASH_SIZE: &'static str = "49";

pub struct Configuration {
    conf: HashMap<DefaultConfSettings,String>,
    ini_file_param: Ini,
    matches: ArgMatches<'static>
}



impl Configuration {

    pub fn  new (app :clap::App<'static,'static>) -> Configuration {

        //store default configuration in conf
        let mut conf:HashMap<DefaultConfSettings,String> = HashMap::new();
        conf.insert(DefaultConfSettings::PORT, "14600".to_string());
        conf.insert(DefaultConfSettings::ApiHost, "localhost".to_string());
        conf.insert(DefaultConfSettings::UdpReceiverPort, "14600".to_string());
        conf.insert(DefaultConfSettings::TcpReceiverPort, "15600".to_string());
        conf.insert(DefaultConfSettings::TESTNET, "false".to_string());
        conf.insert(DefaultConfSettings::DEBUG, "false".to_string());
        conf.insert(DefaultConfSettings::RemoteLimitApi, "".to_string());
        conf.insert(DefaultConfSettings::RemoteAuth, "".to_string());
        conf.insert(DefaultConfSettings::NEIGHBORS, "".to_string());
        conf.insert(DefaultConfSettings::IxiDir, "ixi".to_string());
        conf.insert(DefaultConfSettings::DbPath, "mainnetdb".to_string());
        conf.insert(DefaultConfSettings::DbLogPath, "mainnet.log".to_string());
        conf.insert(DefaultConfSettings::DbCacheSize, "100000".to_string()); //KB
        conf.insert(DefaultConfSettings::CONFIG, "iota.ini".to_string());
        conf.insert(DefaultConfSettings::PRemoveRequest, "0.01".to_string());
        conf.insert(DefaultConfSettings::PDropTransaction, "0.0".to_string());
        conf.insert(DefaultConfSettings::PSelectMilestoneChild, "0.7".to_string());
        conf.insert(DefaultConfSettings::PSendMilestone, "0.02".to_string());
        conf.insert(DefaultConfSettings::PReplyRandomTip, "0.66".to_string());
        conf.insert(DefaultConfSettings::PPropagateRequest, "0.01".to_string());
        conf.insert(DefaultConfSettings::MainDb, "rocksdb".to_string());
        conf.insert(DefaultConfSettings::EXPORT, "false".to_string());
        conf.insert(DefaultConfSettings::SendLimit, "-1.0".to_string());
        conf.insert(DefaultConfSettings::MaxPeers, "0".to_string());
        conf.insert(DefaultConfSettings::DnsRefresherEnabled, "true".to_string());
        conf.insert(DefaultConfSettings::DnsResolutionEnabled, "true".to_string());
        conf.insert(DefaultConfSettings::REVALIDATE, "false".to_string());
        conf.insert(DefaultConfSettings::RescanDb, "false".to_string());
        conf.insert(DefaultConfSettings::MWM, MAINNET_MWM.to_string());

        // Pick a number based on best performance
        conf.insert(DefaultConfSettings::MinRandomWalks, "5".to_string());
        conf.insert(DefaultConfSettings::MaxRandomWalks, "27".to_string());
        // Pick a milestone depth number depending on risk model
        conf.insert(DefaultConfSettings::MaxDepth, "15".to_string());

        conf.insert(DefaultConfSettings::MaxFindTransactions, "100000".to_string());
        conf.insert(DefaultConfSettings::MaxRequestsList, "1000".to_string());
        conf.insert(DefaultConfSettings::MaxGetTrytes, "10000".to_string());
        conf.insert(DefaultConfSettings::MaxBodyLength, "1000000".to_string());
        conf.insert(DefaultConfSettings::ZmqEnabled, "false".to_string());
        conf.insert(DefaultConfSettings::ZmqPort, "5556".to_string());
        conf.insert(DefaultConfSettings::ZmqIpc, "ipc://iri".to_string());
        conf.insert(DefaultConfSettings::ZmqThreads, "2".to_string());

        conf.insert(DefaultConfSettings::QSizeNode, "1000".to_string());
        conf.insert(DefaultConfSettings::PDropCacheEntry, "0.02".to_string());
        conf.insert(DefaultConfSettings::CacheSizeBytes, "15000".to_string());

        conf.insert(DefaultConfSettings::COORDINATOR, MAINNET_COORDINATOR_ADDRESS.to_string());
        conf.insert(DefaultConfSettings::DontValidateTestnetMilestoneSig, "false".to_string());
        conf.insert(DefaultConfSettings::SnapshotFile, MAINNET_SNAPSHOT_FILE.to_string());
        conf.insert(DefaultConfSettings::SnapshotSignatureFile, MAINNET_SNAPSHOT_SIG_FILE.to_string());
        conf.insert(DefaultConfSettings::MilestoneStartIndex, MAINNET_MILESTONE_START_INDEX.to_string());
        conf.insert(DefaultConfSettings::NumberOfKeysInAMilestone, MAINNET_NUM_KEYS_IN_MILESTONE.to_string());
        conf.insert(DefaultConfSettings::TransactionPacketSize, PACKET_SIZE.to_string());
        conf.insert(DefaultConfSettings::RequestHashSize, REQ_HASH_SIZE.to_string());
        conf.insert(DefaultConfSettings::SnapshotTime, GLOBAL_SNAPSHOT_TIME.to_string());


        //parse command line args
        let matc :ArgMatches<'static> = App::get_matches(app);
        let matches_copy = matc.clone();
        let config_file = matc.value_of("config").unwrap_or("").to_string();
        let mut config_file_path = config_file;
        if config_file_path.is_empty() {
            config_file_path = conf.get(&DefaultConfSettings::CONFIG).unwrap().to_string();
        }
        let file_path = config_file_path.clone();
        let config_file_exists = self::Ini::load_from_file(config_file_path);
        let ini_file_param = match config_file_exists {
            Ok(x) => x,
            _ => {
                info!("[CONFIGURATION] Configuration file {} not found. Using default configuration.", file_path);
                Ini::new()
            },
        };

        let ini_copy = &ini_file_param.clone();
        let tmp = Ini::get_from(ini_copy,Some("IRI"),"testnet");
        let testnet_ini :bool = match tmp {
            Some(x) => x=="true",
            _ => false,
        };


        if matc.is_present("testnet") || testnet_ini {
            //In testnet mode, some parameters use other default values.
            conf.insert(DefaultConfSettings::TESTNET, "true".to_string());
            conf.insert(DefaultConfSettings::DbPath, "testnetdb".to_string());
            conf.insert(DefaultConfSettings::DbLogPath, "testnetdb.log".to_string());
            conf.insert(DefaultConfSettings::COORDINATOR, TESTNET_COORDINATOR_ADDRESS.to_string());
            conf.insert(DefaultConfSettings::SnapshotFile, TESTNET_SNAPSHOT_FILE.to_string());
            conf.insert(DefaultConfSettings::MilestoneStartIndex, TESTNET_MILESTONE_START_INDEX.to_string());
            conf.insert(DefaultConfSettings::SnapshotSignatureFile, "".to_string());
            conf.insert(DefaultConfSettings::MWM, TESTNET_MWM.to_string());
            conf.insert(DefaultConfSettings::NumberOfKeysInAMilestone,
                             TESTNET_NUM_KEYS_IN_MILESTONE.to_string());
            conf.insert(DefaultConfSettings::TransactionPacketSize, TESTNET_PACKET_SIZE.to_string());
            conf.insert(DefaultConfSettings::RequestHashSize, TESTNET_REQ_HASH_SIZE.to_string());
            conf.insert(DefaultConfSettings::SnapshotTime, TESTNET_GLOBAL_SNAPSHOT_TIME.to_string());
        }

        //check mandatory parameter port
        let port = matc.value_of("port").unwrap_or("").to_string();
        let port_ini = Ini::get_from(ini_copy,Some("IRI"),"port").unwrap_or("");
        if port.is_empty() && port_ini.is_empty() {
            error!("Invalid arguments list. Provide at least the PORT in iota.ini or with -p option");
            info!("{}",matc.usage());
            ::std::process::exit(0);
        }

        Configuration{conf, ini_file_param, matches : matches_copy}
    }


    fn get_command_line_value(&self, k :&str) ->Option<&str> {
        self.matches.value_of(k)
    }

    fn get_ini_value(&self, k :&str) ->Option<&str> {
        let tmp = self.ini_file_param.get_from(Some("IRI"),k);
        if tmp.is_some() {
            return Some(self.ini_file_param.get_from(Some("IRI"), k).unwrap());
        }
        return None;
    }

    pub fn get_param(&self, param :DefaultConfSettings) ->Option<String> {
        self.get_conf_value(param.to_string().as_ref())
    }

    pub fn get_flag(&self, param :DefaultConfSettings) ->bool {
        self.get_conf_flag(param.to_string().as_ref())
    }

    fn get_conf_value(&self, k :&str) ->Option<String> {
        return match self.get_command_line_value(k) {
            Some(x) => Some(x.to_string()),
            _ => {
                match self.get_ini_value(k) {
                    Some(x) => Some(x.to_string()),
                    _ => {
                        let key = k.parse().unwrap_or(DefaultConfSettings::UnknownParam);
                        if self.conf.contains_key(&key) {
                            return Some(self.conf.get(&key).unwrap().to_string());
                        }
                        return None;
                    }
                }
            }
        };
    }

    fn get_command_line_flag(&self, k :&str) -> Option<bool> {
        if self.matches.is_present(k)  {
            return Some(true)
        }
        return None;
    }

    fn get_ini_flag(&self, k :&str) ->Option<bool> {
        let tmp = self.ini_file_param.get_from(Some("IRI"),k);
        match tmp {
            Some("true") => return Some(true),
            _ => return None,
        }
    }

    pub fn get_conf_flag(&self, k :&str) ->bool {
        return match self.get_command_line_flag(k) {
            Some(x) => Some(x).unwrap(),
            _ => {
                match self.get_ini_flag(k) {
                    Some(x) => Some(x).unwrap(),
                    _ => {
                        let key = k.parse().unwrap_or(DefaultConfSettings::UnknownParam);
                        if self.conf.contains_key(&key) {
                            return (self.conf.get(&key).unwrap()).starts_with("true");
                        }
                        return false;
                    }
                }
            }
        };
    }

    fn string_it(&self, k :&str) -> String {
        self.get_conf_value(k).unwrap_or("".to_string())
    }

    pub fn floating(&self, k :&str) -> f32 {
         match f32::from_str(self.get_conf_value(k).unwrap_or(("0.0".to_string())).as_ref()) {
             Ok(x) => x,
             _ => 0.0,
         }
    }

    pub fn integer(&self, k :&str) -> i32 {
        return i32::from_str(self.get_conf_value(k).unwrap_or(("0".to_string())).as_ref()).unwrap_or(0);
    }

    pub fn booling(&self, k :&str) -> bool {
        self.get_conf_flag(k)
    }

    pub fn long_num(&self, k :&str) -> i64 {
        return i64::from_str(self.get_conf_value(k).unwrap_or(("0".to_string())).as_ref()).unwrap_or(0);
    }

    fn stringify_param(&self, d :DefaultConfSettings) -> String {
        return self.string_it(d.to_string().as_ref());
    }

    pub fn integer_param(&self, d : DefaultConfSettings) -> i32 {
        return self.integer(d.to_string().as_ref());
    }

    pub fn long_num_param(&self, d : DefaultConfSettings) -> i64 {
        return self.long_num(d.to_string().as_ref());
    }

    pub fn booling_param(&self, d : DefaultConfSettings) -> bool {
        return self.booling(d.to_string().as_ref());
    }

    const NOT_DEFINED:&'static str = "<UNDEFINED>";
    pub fn print(&self){
        println!();
        println!("====================");
        println!("|  CONFIGURATION   |");
        println!("====================");
        println!();
        println!("Configuration file     : {}",self.get_param(DefaultConfSettings::CONFIG).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Port                   : {}",self.get_param(DefaultConfSettings::PORT).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("UDP receiver port      : {}",self.get_param(DefaultConfSettings::UdpReceiverPort).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("TCP receiver port      : {}",self.get_param(DefaultConfSettings::TcpReceiverPort).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Neighbors              : {}",self.get_param(DefaultConfSettings::NEIGHBORS).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("API Host               : {}",self.get_param(DefaultConfSettings::ApiHost).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Remote Limit API       : {}",self.get_param(DefaultConfSettings::RemoteLimitApi).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Remote Auth            : {}",self.get_flag(DefaultConfSettings::RemoteAuth));
        println!("Debug                  : {}",self.get_flag(DefaultConfSettings::DEBUG));
        println!("Testnet                : {}",self.get_flag(DefaultConfSettings::TESTNET));
        println!("DB path                : {}",self.get_param(DefaultConfSettings::DbPath).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("DB log path            : {}",self.get_param(DefaultConfSettings::DbLogPath).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("DB cache size          : {}",self.get_param(DefaultConfSettings::DbCacheSize).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("DB                     : {}",self.get_param(DefaultConfSettings::MainDb).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("IXI Directory          : {}",self.get_param(DefaultConfSettings::IxiDir).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. drop request     : {}",self.get_param(DefaultConfSettings::PRemoveRequest).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. drop tx          : {}",self.get_param(DefaultConfSettings::PDropTransaction).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. milest. chld     : {}",self.get_param(DefaultConfSettings::PSelectMilestoneChild).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. send milest.     : {}",self.get_param(DefaultConfSettings::PSendMilestone).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. random tip       : {}",self.get_param(DefaultConfSettings::PReplyRandomTip).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. propag. req.     : {}",self.get_param(DefaultConfSettings::PPropagateRequest).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Export                 : {}",self.get_flag(DefaultConfSettings::EXPORT));
        println!("Send limit             : {}",self.get_param(DefaultConfSettings::SendLimit).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Max peers              : {}",self.get_param(DefaultConfSettings::MaxPeers).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Dns resolution ON      : {}",self.get_flag(DefaultConfSettings::DnsResolutionEnabled));
        println!("Dns refresh ON         : {}",self.get_flag(DefaultConfSettings::DnsRefresherEnabled));
        println!("[TESTNET ONLY] Coo address            : {}",self.get_param(DefaultConfSettings::COORDINATOR).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("[TESTNET ONLY] Skp milestone val.     : {}",self.get_flag(DefaultConfSettings::DontValidateTestnetMilestoneSig));
        println!("Revalidate             : {}",self.get_flag(DefaultConfSettings::REVALIDATE));
        println!("Rescan                 : {}",self.get_flag(DefaultConfSettings::RescanDb));
        println!("Min random walks       : {}",self.get_param(DefaultConfSettings::MinRandomWalks).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Max random walks       : {}",self.get_param(DefaultConfSettings::MaxRandomWalks).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Max requests list      : {}",self.get_param(DefaultConfSettings::MaxRequestsList).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Max get trytes         : {}",self.get_param(DefaultConfSettings::MaxGetTrytes).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Max body length        : {}",self.get_param(DefaultConfSettings::MaxBodyLength).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Max depth              : {}",self.get_param(DefaultConfSettings::MaxDepth).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Min weight magn.       : {}",self.get_param(DefaultConfSettings::MWM).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("ZMQ enabled            : {}",self.get_flag(DefaultConfSettings::ZmqEnabled));
        println!("ZMQ ipc                : {}",self.get_param(DefaultConfSettings::ZmqIpc).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("ZMQ threads            : {}",self.get_param(DefaultConfSettings::ZmqThreads).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("ZMQ port               : {}",self.get_param(DefaultConfSettings::ZmqPort).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("QSize node             : {}",self.get_param(DefaultConfSettings::QSizeNode).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Prob. drop cache       : {}",self.get_param(DefaultConfSettings::PDropCacheEntry).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("PCache size (bytes)    : {}",self.get_param(DefaultConfSettings::CacheSizeBytes).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Snapshot file          : {}",self.get_param(DefaultConfSettings::SnapshotFile).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Snapshot sig. file     : {}",self.get_param(DefaultConfSettings::SnapshotSignatureFile).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Snapshot time          : {}",self.get_param(DefaultConfSettings::SnapshotTime).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Milestone start idx    : {}",self.get_param(DefaultConfSettings::MilestoneStartIndex).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Milestone keys         : {}",self.get_param(DefaultConfSettings::NumberOfKeysInAMilestone).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Tx packet size         : {}",self.get_param(DefaultConfSettings::TransactionPacketSize).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("Request hash size      : {}",self.get_param(DefaultConfSettings::RequestHashSize).unwrap_or(Configuration::NOT_DEFINED.to_string()));
        println!("====================");
        println!();
    }
}

#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum DefaultConfSettings {
    CONFIG,
    PORT,
    ApiHost,
    UdpReceiverPort,
    TcpReceiverPort,
    TESTNET,
    DEBUG,
    RemoteLimitApi,
    RemoteAuth,
    NEIGHBORS,
    IxiDir,
    DbPath,
    DbLogPath,
    DbCacheSize,
    PRemoveRequest,
    PDropTransaction,
    PSelectMilestoneChild,
    PSendMilestone,
    PReplyRandomTip,
    PPropagateRequest,
    MainDb,
    EXPORT,
    // exports transaction trytes to filesystem
    SendLimit,
    MaxPeers,
    DnsResolutionEnabled,
    DnsRefresherEnabled,
    COORDINATOR,
    DontValidateTestnetMilestoneSig,
    REVALIDATE,
    RescanDb,
    MinRandomWalks,
    MaxRandomWalks,
    MaxFindTransactions,
    MaxRequestsList,
    MaxGetTrytes,
    MaxBodyLength,
    MaxDepth,
    MWM,
    ZmqEnabled,
    ZmqPort,
    ZmqIpc,
    ZmqThreads,
    QSizeNode,
    PDropCacheEntry,
    CacheSizeBytes,
    SnapshotFile,
    SnapshotSignatureFile,
    MilestoneStartIndex,
    NumberOfKeysInAMilestone,
    TransactionPacketSize,
    RequestHashSize,
    SnapshotTime,
    UnknownParam,
}

impl FromStr for DefaultConfSettings {
    type Err = ();

    fn from_str(s: &str) -> Result<DefaultConfSettings, ()> {
        let tmp = s.to_lowercase();
        let lower_case = tmp.as_ref();
        match lower_case {
            "config" => Ok(DefaultConfSettings::CONFIG),
            "port" => Ok(DefaultConfSettings::PORT),
            "apihost" => Ok(DefaultConfSettings::ApiHost),
            "udpreceiverport" => Ok(DefaultConfSettings::UdpReceiverPort),
            "tcpreceioverport" => Ok(DefaultConfSettings::TcpReceiverPort),
            "testnet" => Ok(DefaultConfSettings::TESTNET),
            "debug" => Ok(DefaultConfSettings::DEBUG),
            "remotelimitapi" => Ok(DefaultConfSettings::RemoteLimitApi),
            "remoteauth" => Ok(DefaultConfSettings::RemoteAuth),
            "neighbors" => Ok(DefaultConfSettings::NEIGHBORS),
            "ixidir" => Ok(DefaultConfSettings::IxiDir),
            "dbpath" => Ok(DefaultConfSettings::DbPath),
            "dblogpath" => Ok(DefaultConfSettings::DbLogPath),
            "dbcachesize" => Ok(DefaultConfSettings::DbCacheSize),
            "premoverequest" => Ok(DefaultConfSettings::PRemoveRequest),
            "pdroptransaction" => Ok(DefaultConfSettings::PDropTransaction),
            "pselectmilestonechild" => Ok(DefaultConfSettings::PSelectMilestoneChild),
            "psendmilestone" => Ok(DefaultConfSettings::PSendMilestone),
            "preplyrandomtip" => Ok(DefaultConfSettings::PReplyRandomTip),
            "ppropagaterequest" => Ok(DefaultConfSettings::PPropagateRequest),
            "maindb" => Ok(DefaultConfSettings::MainDb),
            "export" => Ok(DefaultConfSettings::EXPORT),
            // exports transaction trytes to filesystem
            "sendlimit" => Ok(DefaultConfSettings::SendLimit),
            "maxpeers" => Ok(DefaultConfSettings::MaxPeers),
            "dnsresolutionenabled" => Ok(DefaultConfSettings::DnsResolutionEnabled),
            "dnsrefreshenabled" => Ok(DefaultConfSettings::DnsRefresherEnabled),
            "coordinator" => Ok(DefaultConfSettings::COORDINATOR),
            "dontvalidatetestnetmilestone" => Ok(DefaultConfSettings::DontValidateTestnetMilestoneSig),
            "revalidate" => Ok(DefaultConfSettings::REVALIDATE),
            "rescandb" => Ok(DefaultConfSettings::RescanDb),
            "minrandomwalks" => Ok(DefaultConfSettings::MinRandomWalks),
            "maxrandomwalks" => Ok(DefaultConfSettings::MaxRandomWalks),
            "maxfindtransactions" => Ok(DefaultConfSettings::MaxFindTransactions),
            "maxrequestslist" => Ok(DefaultConfSettings::MaxRequestsList),
            "maxgettrytes" => Ok(DefaultConfSettings::MaxGetTrytes),
            "maxbodylength" => Ok(DefaultConfSettings::MaxBodyLength),
            "maxdepth" => Ok(DefaultConfSettings::MaxDepth),
            "mwm" => Ok(DefaultConfSettings::MWM),
            "zmqenabled" => Ok(DefaultConfSettings::ZmqEnabled),
            "zmqport" => Ok(DefaultConfSettings::ZmqPort),
            "zmqipc" => Ok(DefaultConfSettings::ZmqIpc),
            "zmqthreads" => Ok(DefaultConfSettings::ZmqThreads),
            "qsizenode" => Ok(DefaultConfSettings::QSizeNode),
            "pdropcacheentry" => Ok(DefaultConfSettings::PDropCacheEntry),
            "cachesizebytes" => Ok(DefaultConfSettings::CacheSizeBytes),
            "snapshotfile" => Ok(DefaultConfSettings::SnapshotFile),
            "snapshotsignaturefile" => Ok(DefaultConfSettings::SnapshotSignatureFile),
            "milestonestartindex" => Ok(DefaultConfSettings::MilestoneStartIndex),
            "numberofkeysinamilestone" => Ok(DefaultConfSettings::NumberOfKeysInAMilestone),
            "transactionpacketsize" => Ok(DefaultConfSettings::TransactionPacketSize),
            "requesthashsize" => Ok(DefaultConfSettings::RequestHashSize),
            "snapshottime" => Ok(DefaultConfSettings::SnapshotTime),
            "udp-receiver-port" => Ok(DefaultConfSettings::UdpReceiverPort),
            "tcp-receiver-port" => Ok(DefaultConfSettings::TcpReceiverPort),
            "remote-limit-api" => Ok(DefaultConfSettings::RemoteLimitApi),
            "remote-auth" => Ok(DefaultConfSettings::RemoteAuth),
            "zmq-enabled" => Ok(DefaultConfSettings::ZmqEnabled),
            "TEST_NET" => Ok(DefaultConfSettings::TESTNET),
            "test-net" => Ok(DefaultConfSettings::TESTNET),
            "send-limit" => Ok(DefaultConfSettings::SendLimit),
            "dns-resolution-false" => Ok(DefaultConfSettings::DnsResolutionEnabled),
            "max-peers" => Ok(DefaultConfSettings::MaxPeers),
            "testnet-coordinator" => Ok(DefaultConfSettings::COORDINATOR),
            "testnet-no-coo-validation" => Ok(DefaultConfSettings::DontValidateTestnetMilestoneSig),
            "snapshot-sig" => Ok(DefaultConfSettings::SnapshotSignatureFile),
            "milestone-start" => Ok(DefaultConfSettings::MilestoneStartIndex),
            "milestone-keys" => Ok(DefaultConfSettings::NumberOfKeysInAMilestone),
            "snapshot-timestamp" => Ok(DefaultConfSettings::SnapshotTime),
            _ => Err(()),
        }
    }


}

impl ToString for DefaultConfSettings {
    //type Err = ();

    fn to_string(&self) -> String {
        match self {
            &DefaultConfSettings::CONFIG => "config",
            &DefaultConfSettings::PORT => "port",
            &DefaultConfSettings::ApiHost => "apihost",
            &DefaultConfSettings::UdpReceiverPort => "udpreceiverport",
            &DefaultConfSettings::TcpReceiverPort => "tcpreceioverport",
            &DefaultConfSettings::TESTNET => "testnet",
            &DefaultConfSettings::DEBUG => "debug",
            &DefaultConfSettings::RemoteLimitApi => "remotelimitapi",
            &DefaultConfSettings::RemoteAuth => "remoteauth",
            &DefaultConfSettings::NEIGHBORS => "neighbors",
            &DefaultConfSettings::IxiDir => "ixidir",
            &DefaultConfSettings::DbPath => "dbpath",
            &DefaultConfSettings::DbLogPath => "dblogpath",
            &DefaultConfSettings::DbCacheSize => "dbcachesize",
            &DefaultConfSettings::PRemoveRequest => "premoverequest",
            &DefaultConfSettings::PDropTransaction => "pdroptransaction",
            &DefaultConfSettings::PSelectMilestoneChild => "pselectmilestonechild",
            &DefaultConfSettings::PSendMilestone => "psendmilestone",
            &DefaultConfSettings::PReplyRandomTip => "preplyrandomtip",
            &DefaultConfSettings::PPropagateRequest => "ppropagaterequest",
            &DefaultConfSettings::MainDb => "maindb",
            &DefaultConfSettings::EXPORT => "export",
            // exports transaction trytes to filesystem
            &DefaultConfSettings::SendLimit => "sendlimit",
            &DefaultConfSettings::MaxPeers => "maxpeers",
            &DefaultConfSettings::DnsResolutionEnabled => "dnsresolutionenabled",
            &DefaultConfSettings::DnsRefresherEnabled => "dnsrefreshenabled",
            &DefaultConfSettings::COORDINATOR => "coordinator",
            &DefaultConfSettings::DontValidateTestnetMilestoneSig => "dontvalidatetestnetmilestone",
            &DefaultConfSettings::REVALIDATE => "revalidate",
            &DefaultConfSettings::RescanDb => "rescandb",
            &DefaultConfSettings::MinRandomWalks => "minrandomwalks",
            &DefaultConfSettings::MaxRandomWalks => "maxrandomwalks",
            &DefaultConfSettings::MaxFindTransactions => "maxfindtransactions",
            &DefaultConfSettings::MaxRequestsList => "maxrequestslist",
            &DefaultConfSettings::MaxGetTrytes => "maxgettrytes",
            &DefaultConfSettings::MaxBodyLength => "maxbodylength",
            &DefaultConfSettings::MaxDepth => "maxdepth",
            &DefaultConfSettings::MWM => "mwm",
            &DefaultConfSettings::ZmqEnabled => "zmqenabled",
            &DefaultConfSettings::ZmqPort => "zmqport",
            &DefaultConfSettings::ZmqIpc => "zmqipc",
            &DefaultConfSettings::ZmqThreads => "zmqthreads",
            &DefaultConfSettings::QSizeNode => "qsizenode",
            &DefaultConfSettings::PDropCacheEntry => "pdropcacheentry",
            &DefaultConfSettings::CacheSizeBytes => "cachesizebytes",
            &DefaultConfSettings::SnapshotFile => "snapshotfile",
            &DefaultConfSettings::SnapshotSignatureFile => "snapshotsignaturefile",
            &DefaultConfSettings::MilestoneStartIndex => "milestonestartindex",
            &DefaultConfSettings::NumberOfKeysInAMilestone => "numberofkeysinamilestone",
            &DefaultConfSettings::TransactionPacketSize => "transactionpacketsize",
            &DefaultConfSettings::RequestHashSize => "requesthashsize",
            &DefaultConfSettings::SnapshotTime => "snapshottime",
            _ => "",
        }.to_string()
    }
}