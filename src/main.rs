use base64::{decode, encode};
use std::str::from_utf8;
use std::str::FromStr;
use structopt::StructOpt;

use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

#[derive(StructOpt)]
#[structopt(about = "JSON over DNS")]
enum Cli {
    Fetch { host: String },
    Encode { value: String },
}

use Cli::*;

fn main() {
    let cli = Cli::from_args();

    match cli {
        Encode { value } => {
            let b64 = encode(value);
            println!("Put this in your TXT record (including the quotes):");
            println!("\"{}\"", b64);
        }
        Fetch { host } => {
            let address = "8.8.8.8:53".parse().unwrap();
            let conn = UdpClientConnection::new(address).unwrap();
            let client = SyncClient::new(conn);
            let name = Name::from_str(&host).unwrap();
            let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::TXT).unwrap();
            let answers: &[Record] = response.answers();
            for answer in answers.iter() {
                if let &RData::TXT(ref t) = answer.rdata() {
                    for s in t.clone().txt_data().iter() {
                        let bytes = decode(s).unwrap();
                        let res = from_utf8(&bytes);
                        match res {
                            Ok(str) => {
                                println!("{}", str);
                            }
                            Err(err) => {
                                eprintln!("Decoding error: {}", err);
                            }
                        }
                    }
                }
            }
        }
    }
}
