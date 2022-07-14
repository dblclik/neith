use tokio::net::UdpSocket;
use std::io;
use std::collections::BTreeMap;

#[derive(Debug, Default, PartialEq, PartialOrd, Hash)]
struct RetentionPolicy {
    interval: u128 // ms to retain series data
}

#[derive(Debug)]
enum Retention {
    RetentionPolicy,
    None
}

impl Default for Retention {
    fn default() -> Self {Retention::None}
}

#[derive(Debug, Default)]
struct Index<T> {
    series: BTreeMap<u128, T>, // ms-resolution timeseries
    iniated: u128, // time since epoch (ms)
    retention: Retention // retention configuration enum
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    println!("listening to address {}", "0.0.0.0:8080");
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        // println!("message: {:?}", &buf[..len]);
        println!("message string: {}", String::from_utf8_lossy(&buf[..len].to_vec()));

        // reset buf before next message
        buf = [0; 1024];
    }
}