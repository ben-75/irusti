use std::io::Write;
use std::net::TcpListener;
use std::thread;
use scoped_pool::Pool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::str;
use std::str::FromStr;

use http_parser::{HttpParserCallback, CallbackResult, ParseAction, HttpParser, HttpParserType, HttpMethod};

use serde_json::{Value, Error};
use serde_json::from_slice;
use serde_json::from_value;
use serde_json::to_string;

use network::node_info::NodeInfo;

pub fn start_tcp_server(tcp_port :i32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}",tcp_port)).unwrap();
    debug!("TCP server starting on port {}", tcp_port);

    let pool = Pool::new(4);

    pool.scoped(|scope| {
        for stream in listener.incoming() {
            match stream{
                Ok(stream)=>{
                    scope.execute(move || handle_connection(stream));
                },
                Err(_)=>{
                    break;
                }
            }


        }
    });

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer =  [0; 1024];
    let get_sleep = b"GET /sleep";
    stream.read(&mut buffer);
    let mut parser = HttpParser::new(HttpParserType::Request);
    let mut http_response = Callback{
        response_code:200,
        headers_valid:false,
        json:Value::Null,
        response_body:"".to_string()};
    
    parser.execute(&mut http_response, buffer.as_ref());

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = format!("HTTP/1.1 {} OK\
    \r\nAccess-Control-Allow-Origin: *\
    \r\nKeep-Alive: timeout=500, max=100\
    \r\n
    \r\n{}",http_response.response_code, http_response.response_body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

struct Callback{
    response_code :u32,
    headers_valid :bool,
    json :Value,
    response_body: String,
}

impl HttpParserCallback for Callback {

    fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
        println!("Message begin {:?}",parser.method.unwrap().to_string());
        match parser.method {
            Some(HttpMethod::Post) => return Ok(ParseAction::None),
            _ => return {self.response_code=405;return Ok(ParseAction::SkipBody);}
        }
        Ok(ParseAction::None)
    }

    fn on_url(&mut self, parser: &mut HttpParser, data: &[u8],) -> CallbackResult {
        println!("on_url:'{:?}'",str::from_utf8(data));
        Ok(ParseAction::None)
    }

    fn on_header_field(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        let mut header_name = str::from_utf8(data);
        match header_name {
            Err(_) => {self.response_code=400;return Ok(ParseAction::SkipBody);}
            Ok(x) => {
                if x.to_lowercase().eq("x-iota-api-version") {
                    self.headers_valid = true;
                }
            }
        }
        Ok(ParseAction::None)
    }

    fn on_headers_complete(&mut self, parser: &mut HttpParser) -> CallbackResult {
        if self.response_code != 200 {
            return Ok(ParseAction::SkipBody);
        }
        if !self.headers_valid {
            self.response_code=400;
            return Ok(ParseAction::SkipBody);
        }
        Ok(ParseAction::None)
    }

    fn on_body(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        let parsed = from_slice(data);
        match parsed {
            Err(_) => self.response_code = 400,
            Ok(x) => {
                self.json = x;
                match self.json["command"].as_str() {
                    None => {
                        self.response_code=400;
                    }
                    Some("storeMessage") => {self.store_message();},
                    Some("addNeighbors") => {self.add_neighbors();},
                    Some("attachToTangle") => {self.attach_to_tangle();},
                    Some("broadcastTransactions") => {self.broadcast_transactions();},
                    Some("findTransactions") => {self.find_transactions();},
                    Some("getBalances") => {self.get_balances();},
                    Some("getInclusionStates") => {self.get_inclusion_states();},
                    Some("getNeighbors") => {self.get_neighbors();},
                    Some("getNodeInfo") => {self.get_node_info();},
                    Some("getTips") => {self.get_tips();},
                    Some("getTransactionsToApprove") => {self.get_transactions_to_approve();},
                    Some("getTrytes") => {self.get_trytes();},
                    Some("interruptAttachingToTangle") => {self.interrupt_attaching_to_tangle();},
                    Some("removeNeighbors") => {self.remove_neighbors();},
                    Some("storeTransactions") => {self.store_transactions();},
                    Some("getMissingTransactions") => {self.get_missing_transactions();},
                    Some("checkConsistency") => {self.check_consistency();},
                    Some("wereAddressesSpentFrom") => {self.were_addresses_spent_from();},
                    _ => {self.run_ixi_command()},
                }
            },
        }
        Ok(ParseAction::None)
    }
}

impl Callback {

    pub fn store_message(&self) {
        debug!("store_message");
    }

    pub fn add_neighbors(&self) {
        debug!("add_neighbors");

    }

    pub fn attach_to_tangle(&self) {
        debug!("attach_to_tangle");

    }

    pub fn broadcast_transactions(&self) {
        debug!("broadcast_transactions");

    }

    pub fn find_transactions(&self) {
        debug!("find_transactions");

    }

    pub fn get_balances(&self) {
        debug!("get_balances");

    }

    pub fn get_inclusion_states(&self) {
        debug!("get_inclusion_states");

    }

    pub fn get_neighbors(&self) {
        debug!("get_neighbors");

    }

    pub fn get_node_info(&mut self) {
        debug!("get_node_info");
        let node_info = NodeInfo::new();
        let serialized = to_string(&node_info).unwrap();
        self.response_body = serialized;
    }

    pub fn get_tips(&self) {
        debug!("get_tips");

    }

    pub fn get_transactions_to_approve(&self) {
        debug!("get_transactions_to_approve");

    }

    pub fn get_trytes(&self) {
        debug!("get_trytes");

    }

    pub fn interrupt_attaching_to_tangle(&self) {
        debug!("interrupt_attaching_to_tangle");

    }

    pub fn remove_neighbors(&self) {
        debug!("remove_neighbors");

    }

    pub fn store_transactions(&self) {
        debug!("store_transactions");

    }

    pub fn get_missing_transactions(&self) {
        debug!("get_missing_transactions");

    }

    pub fn check_consistency(&self) {
        debug!("check_consistency");

    }

    pub fn were_addresses_spent_from(&self) {
        debug!("were_addresses_spent_from");

    }

   pub fn run_ixi_command(&self) {
       debug!("run_ixi_command");

   }
}