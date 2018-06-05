extern crate zmq;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub struct MessageQ {
    join_handle :Option<JoinHandle<()>>,
    tx : Option<Sender<String>>,
}

impl MessageQ {

    pub fn new(threads :i32, ipc :Option<String>, port :i32, enabled :bool) -> MessageQ {

        if enabled {
            let (tx , rx) :(Sender<String>,Receiver<String>) = channel();
            let context = zmq::Context::new();
            let publisher = context.socket(zmq::PUB).unwrap();
            match publisher.bind(format!("tcp://*:{}",port).as_ref()) {
                Ok(_) => {
                    info!("ZMQ starting on port: {}",port);
                    if threads>1 {
                        warn!("ZMQ binding for rust only support 1 IO-Thread. Current configuration \
                        ask for {} IO-Threads. Sorry, but only one thread will be started. \
                        See: https://github.com/erickt/rust-zmq/issues/206",threads);
                    }
                    //TODO : some more code required here

                    // Spawn worker thread, giving it `send` and whatever else it needs
                    let join_handle = thread::spawn(move||  {
                        let mut running = true;
                        while(running) {
                            match rx.recv() {
                                Ok(message) => {
                                    match message.len() {
                                        0 => {running = false;drop(());},
                                        _ => {info!("publish:{}",message);publisher.send_str(message.as_ref(),0);()},
                                    };

                                },
                                _ => warn!("Fail to receive message"),
                            }
                        }
                    });


                    return MessageQ{
                        join_handle: Some(join_handle),
                        tx: Some(tx)
                    };
                }
                Err(info) => {
                    error!("Fail to bind ZMQ on port {}. Error: {}",port, info);
                    error!("ZMQ disabled");
                    return MessageQ{
                        join_handle: None,
                        tx: None
                    }
                }
            };
        }else{
            MessageQ{
                join_handle: None,
                tx: None
            }
        }
    }

    pub fn publish(&self, message :String){
        if message.len()>0 {
            match &self.tx {
                Some(tx) => {
                    tx.send(message);
                },
                _ => (),
            }
        }
    }

    pub fn shutdown(&self){
        match self.tx {
            Some(ref tx) => {
                tx.send("".to_string());

            },
            _ => (),
        };
    }


}