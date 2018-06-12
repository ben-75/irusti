extern crate rocksdb;
extern crate num_cpus;
use self::rocksdb::{DB, Options, ColumnFamily};
use txhash::TxHash;

pub struct Tangle {
    db :DB,
    cf_default :ColumnFamily,
    cf_transaction :ColumnFamily,
    cf_transaction_metadata :ColumnFamily,
    cf_milestone :ColumnFamily,
    cf_state_diff :ColumnFamily,
    cf_address :ColumnFamily,
    cf_approvee :ColumnFamily,
    cf_bundle :ColumnFamily,
    cf_obsolete_tag :ColumnFamily,
    cf_tag :ColumnFamily,
}

impl Tangle {

    pub fn get_effective_path(db_path: String, is_testnet :bool) -> String {
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
        effective_db_path
    }

    pub fn new(db_path: String) -> Tangle {
        let db_path_2 = db_path.clone();

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_manifest_file_size(1048576);
        opts.set_max_open_files(10000);
        opts.set_max_background_compactions(1);
        opts.set_allow_concurrent_memtable_write(true);

        //TODO : terminate setup
        let num = num_cpus::get();
        info!("Number of cpus: {}", num);

        let cfs = ["default","transaction",
            "transaction-metadata",
            "milestone",
            "stateDiff",
            "address",
            "approvee",
            "bundle",
            "obsoleteTag",
            "tag"];
        let db :DB = match DB::open_cf(&opts, db_path_2.clone(), &cfs) {
            Ok(database) => {info!("Starting Tangle at {}",db_path_2);database},
            Err(error) => {
                error!("Cannot open database: {}", error);
                panic!("Aborting. Reason: cannot open database");
            }
        };
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }

        db.delete(b"my key").unwrap();

        Tangle{
            cf_default: DB::cf_handle(&db,"default").unwrap(),
            cf_transaction: DB::cf_handle(&db,"transaction").unwrap(),
            cf_transaction_metadata: DB::cf_handle(&db,"transaction-metadata").unwrap(),
            cf_milestone : DB::cf_handle(&db,"milestone").unwrap(),
            cf_state_diff : DB::cf_handle(&db,"stateDiff").unwrap(),
            cf_address : DB::cf_handle(&db,"address").unwrap(),
            cf_approvee : DB::cf_handle(&db,"approvee").unwrap(),
            cf_bundle : DB::cf_handle(&db,"bundle").unwrap(),
            cf_obsolete_tag : DB::cf_handle(&db,"obsoleteTag").unwrap(),
            cf_tag: DB::cf_handle(&db,"tag").unwrap(),
            db,
        }
    }

    pub fn transaction_exists(&self, txhash :&TxHash) -> bool {
        match self.db.get_cf(self.cf_transaction,txhash.as_u8_array()) {
            Err(_) => {error!("Database read failure");false},
            Ok(x) => {
                match x {
                    None => false,
                    Some(_) => true,
                }
            }
        }
    }

    pub fn transaction_save(&self, txhash :&TxHash, value :&[u8]) -> Result<(),String> {
        match self.db.put_cf(self.cf_transaction,txhash.as_u8_array(), value){
            Ok(()) => Ok(()),
            Err(x) => Err(x.to_string()),
        }
    }

    pub fn shutdown(db_path :String){
        let opts= Options::default();
        info!("Shutting down database at {}", db_path);
        match DB::destroy(&opts, db_path){
            Ok(_info) => info!("Shutdown database."),
            Err(error) => error!("Fail to shutdown db. {:?}", error),
        }
    }

}