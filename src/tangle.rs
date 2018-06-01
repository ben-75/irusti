extern crate rocksdb;
extern crate num_cpus;
use self::rocksdb::{DB, Options};


pub struct Tangle {
    db :DB
}

impl Tangle {

    pub fn new(db_path: String) -> Tangle {
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
        let db :DB = match DB::open(&opts, db_path) {
            Ok(database) => database,
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

        Tangle{db}
    }

    pub fn shutdown(path :String){
        let opt= Options::default();
        match DB::destroy(&opt, path){
            Ok(info) => info!("Shutdown database."),
            Err(error) => error!("Fail to shutdown db. {:?}", error),
        }
    }
}