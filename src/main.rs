use std::net::UdpSocket;
use std::thread;
use std::process::Command;
use chrono::Local;


const BROADCAST_ADDR: &str = "255.255.255.255:4000";
const LISTEN_ADDR: &str = "0.0.0.0:4000";
const MAX_INCOMING_BEACON_SIZE: usize = 1024;
const SEND_SLEEP_TIME: u64 = 3;


fn sender() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    socket.set_broadcast(true).expect("Failed to set broadcast");
    let out_put = Command::new("hostname").output().expect("Failed to execute command");
    let host_name = String::from_utf8(out_put.stdout).unwrap().strip_suffix("\n").expect("Failed to strip suffix for host_name").to_string();
    loop {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string().to_owned();
        let buf = Vec::from(format!("{{\"host\": \"{}\", \"time\": \"{}\", \"from\": \"rust\"}}", host_name, now));
        socket.send_to(buf.as_slice(), BROADCAST_ADDR).expect("Failed to send data");
        thread::sleep(std::time::Duration::from_secs(SEND_SLEEP_TIME));
    }
}

fn receiver() {
    let socket = UdpSocket::bind(LISTEN_ADDR).expect("Failed to bind socket");
    let mut buffer = vec![0; MAX_INCOMING_BEACON_SIZE];
    loop {
        socket.recv(&mut buffer).expect("Failed to receive data");
        let data = buffer
            .clone()
            .into_iter()
            .filter(|item| item != &0)
            .collect();
        println!("recv: {}", String::from_utf8(data).unwrap());
    }

}

fn main() {
    let send_thread = thread::spawn(|| {
        sender();
    });
    let receive_thread = thread::spawn(|| {
        receiver();
    });

    send_thread.join().expect("send thread panicked");
    receive_thread.join().expect("receive thread panicked");
}
