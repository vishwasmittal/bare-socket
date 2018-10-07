extern mod sync;

// str op trait
use std::str::StrSlice;
// for tcp listen
use std::io::{TcpListener, TcpStream};
use std::io::net::ip::SocketAddr;
// for trait
use std::io::{Listener, Writer, Acceptor, Buffer};
// for spawn
use std::task::spawn;
// for read_line
use std::io::BufferedStream;
// for gathering chat msg
use std::comm::SharedChan;
// for chat msg storage
use sync::RWArc;


fn chat_loop(ss: &mut BufferedStream<TcpStream>, chan: SharedChan<~str>, arc: RWArc<~[~str]>) {
    ss.write(bytes!("Welcome to Simple Chat Server!\n"));
    ss.write(bytes!("Plz input yourname: "));
    ss.flush();
    let mut name : ~str = ss.read_line().unwrap();
    name = name.trim_right().into_owned();
    ss.write_str(format!("Hello, {}!\n", name));
    ss.flush();
    let mut pos = 0;
    loop {
        arc.read(|lines| {
                println!("DEBUG arc.read() => {}", lines.to_str());
                for i in range(pos, lines.len()) {
                    ss.write_str(lines[i]);
                }
                pos = lines.len();
            });
        ss.write(bytes!(" > "));
        ss.flush();
        let reads = ss.read_line().unwrap();
        if reads.trim().len() != 0 {
            println!("DEBUG reads len =>>>>> {}", reads.len());
            chan.send(format!("[{}] said: {}", name, reads));
            println!("DEBUG: got '{}' from {}", reads.trim(), name);
        }
    }
}

fn main () {
    //let ip : IpAddr = from_str("127.0.0.1").unwrap();
    //let addr = SocketAddr{ip:ip, port:8888};
    let addr : SocketAddr = from_str("127.0.0.1:8888").unwrap();

    let listener = TcpListener::bind(addr).unwrap();
    let mut acceptor = listener.listen().unwrap();
    let (port, chan) : (Port<~str>, SharedChan<~str>)= SharedChan::new();
    let arc : RWArc<~[~str]> = RWArc::new(~[]);

    let arc_w = arc.clone();
    spawn(proc() {
        loop {
            let msg = port.recv();
            print!("DEBUG: {}", msg);
            arc_w.write(|lines| lines.push(msg.to_str()));
        }
    });
    loop {
        match acceptor.accept() {
            Err(_) => println!("error listen"),
            Ok(mut stream) => {
                println!("DEBUG: got connection from {} to {}",
                         stream.peer_name().unwrap().to_str(),
                         stream.socket_name().unwrap().to_str());
                let chan = chan.clone();
                let arc = arc.clone();
                spawn(proc() {
                    let mut stream = BufferedStream::new(stream);
                    chat_loop(&mut stream, chan, arc);
                });
            }
        }
    }
}
