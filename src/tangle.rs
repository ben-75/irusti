extern crate rocksdb;
extern crate num_cpus;
use self::rocksdb::{DB, Options};
use txhash::TxHash;

pub struct Tangle {
    db :DB,
    pub db_path :String,
}

impl Tangle {

    pub fn safe_new(db_path: String, is_testnet :bool) -> Tangle {

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
        let db :DB = match DB::open(&opts, db_path_2.clone()) {
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

        Tangle{db, db_path}
    }

    pub fn new_read_only(db_path :String) -> Tangle {
        let mut opts = Options::default();
        opts.create_if_missing(false);
        opts.create_missing_column_families(false);
        opts.set_max_manifest_file_size(1048576);
        opts.set_max_open_files(10000);
        opts.set_max_background_compactions(1);
        opts.set_allow_concurrent_memtable_write(false);
        //TODO : make it really read only
        let db :DB = match DB::open(&opts, db_path.clone()) {
            Ok(database) => {info!("Open Tangle Read Only at {}",db_path);database},
            Err(error) => {
                error!("Cannot open read only database: {}", error);
                panic!("Aborting. Reason: cannot open read only database");
            }
        };
        Tangle{db,db_path}
    }

    pub fn exists(&self, hash :&TxHash) -> bool {
        //TODO
        false
    }

    pub fn shutdown(&self){
        let opt= Options::default();
        info!("Shutting down database at {}",self.db_path);
        match DB::destroy(&opt, &self.db_path){
            Ok(_info) => info!("Shutdown database."),
            Err(error) => error!("Fail to shutdown db. {:?}", error),
        }
    }

}