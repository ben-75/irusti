extern crate zmq;

pub struct MessageQ {
    context :Option<zmq::Context>,
    publisher : Option<zmq::Socket>,
}

impl MessageQ {

    pub fn new(threads :i32, ipc :Option<String>, port :i32, enabled :bool) -> MessageQ {

        if enabled {
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
                    return MessageQ{
                        context: Some(context),
                        publisher: Some(publisher)
                    };
                }
                Err(info) => {
                    error!("Fail to bind ZMQ on port {}. Error: {}",port, info);
                    error!("ZMQ disabled");
                    return MessageQ{
                        context: None,
                        publisher: None
                    }
                }
            };
        }else{

            MessageQ{
                context: None,
                publisher: None
            }
        }
    }


}