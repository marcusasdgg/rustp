use std::io::{Read, Write};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{ collections::HashMap, net::{SocketAddr, TcpListener, TcpStream}, thread::Thread};

type ArcClient = Arc<Client>;
pub struct RustTP {
    server_con: TcpListener,
    
    client_addrs: Mutex<HashMap<SocketAddr, ArcClient>>,
}

struct Client {
    addr: SocketAddr,
    stream: Mutex<TcpStream>,
    datastream: Mutex<Option<TcpStream>>,
}
#[allow(dead_code)]
trait SendResponse {
    
    //series 100
    fn send110(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send125(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send150(stream: &mut TcpStream){
        stream.write(b"150 Opening data connection for file list.\r\n").unwrap();
    }


    // series 200
    fn send200(stream: &mut TcpStream){

        stream.write(b"200 Command Okay\r\n").unwrap();
        stream.flush().unwrap();
    }
    fn send202(stream: &mut TcpStream){
        stream.write(b"200 Command Okay yay\r\n").unwrap();
        stream.flush().unwrap();
    }
    
    fn send211(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send212(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send213(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send214(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send215(stream: &mut TcpStream){
        stream.write(b"215 Windows_NT\r\n").unwrap();
    }
    
    fn send220(stream: &mut TcpStream){
        stream.write(b"220 FTP Server ready\r\n").unwrap();
    }

    fn send221(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send225(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send226(stream: &mut TcpStream){
        stream.write(b"226 Transfer complete\r\n").unwrap();
    }

    fn send227(stream: &mut TcpStream) -> TcpListener{
    
        let listener = TcpListener::bind("0.0.0.0:0").unwrap(); // 0.0.0.0 binds to any available IP on a random port

        let port = listener.local_addr().unwrap().port();  // Get the randomly assigned port

        // Convert the port to the format required by PASV (p1, p2)
        let p1 = port / 256;
        let p2 = port % 256;

        let binding = listener.local_addr().unwrap().ip().to_string();
        let vec: Vec<&str> = binding.split(".").collect();

        let str = format!("227 Entering Passive Mode (192,168,0,35,{},{})\r\n",p1,p2);
        //vec[0], vec[1], vec[2],vec[3]
        print!("port: {:?}\n",port);
        println!("{str}");
        

        stream.write(str.as_bytes()).unwrap();
        stream.flush().unwrap();
        listener
    }

    fn send228(stream: &mut TcpStream){
        stream
        .write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send229(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send230(stream: &mut TcpStream){
        stream.write(b"230 User logged in, shutup\r\n").unwrap();
    }

    fn send232(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send234(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send235(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send250(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send257(stream: &mut TcpStream, path: String){
        stream.write(format!("257 \"{path}\" is directory\r\n").as_bytes()).unwrap();
        stream.flush();
    }
    
    //series 300

    fn send300(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send331(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send332(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send334(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send336(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    //series 400
    fn send421(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send425(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send426(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send430(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send431(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send434(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send450(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send451(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send452(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    //series 500

    fn send501(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send502(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send503(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send504(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send530(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send532(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send533(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send534(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send535(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send536(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send537(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send550(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send551(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send552(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send553(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    //series 600

    fn send600(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send631(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send632(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    fn send633(stream: &mut TcpStream){
        stream.write(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }
    
    //new commands
}

trait SendData {

}

//what this project is, i want this to be a ftp file server for windows
// initial MVP:
// directory designation
// file retreival
// active mode.
//anonymous no auth check

impl RustTP {
    pub fn new_with_paths(path_list: Vec<String>) -> Arc<Self> {
        let toc = TcpListener::bind("0.0.0.0:21").unwrap();
        let rs = Arc::new(RustTP {server_con: toc, client_addrs: Mutex::new(HashMap::new())});
        rs.clone().start_event_loop();
        rs
    }

    pub fn new() -> Arc<Self> {
        let toc = TcpListener::bind("0.0.0.0:21").unwrap();
        let rs = Arc::new(RustTP {server_con: toc, client_addrs: Mutex::new(HashMap::new())});
        rs.clone().start_event_loop();
        rs
    }


    fn start_event_loop(self: Arc<Self>){
        thread::spawn(move || {
            while let Ok(t) = self.server_con.accept() {
                let stream = t.0;
                let addr = t.1;
                println!("new client connected with {:?}", addr);
                let client = Client::new(addr, stream);
                let mut lock = self.client_addrs.lock().unwrap();
                lock.insert(addr, client);
                drop(lock);
            }
        });
    }

    


}

impl SendResponse for Client{

}

impl Client {
    fn new(addr: SocketAddr, stream: TcpStream) -> ArcClient{
        //loop
        let s = Arc::new(Client {addr, stream: Mutex::new(stream), datastream: Mutex::new(None)}) ;
        s.clone().start_event_loop();
        return s;
    }

    fn start_event_loop(self: Arc<Self>){
        let mut buf: [u8; 1000] = [0;1000];
        let mut stream = self.stream.lock().unwrap();
        Client::send220(&mut stream);
        let stream = &mut stream;
        while let Ok(size) = stream.read(&mut buf){
            let data = &buf[..size];

            if size == 0 {
                println!("connection closed");
                return;
            }

            let comm = String::from_utf8_lossy(data);

            println!("command received from {:?}: {:?}", self.addr, comm);
            let word = &comm[..4].to_uppercase();


            match word.as_str() {
                "USER" => {
                    Client::send230(stream);
                },
                "PASS" => {},
                "ACCT" => {},
                "CWD " => {
                    Client::send200(stream);
                },
                "CDUP" => {},
                "SMNT" => {},
                "QUIT" => {},
                "REIN" => {},
                "PORT" => {},
                "PASV" => {
                    self.clone().handle_pasv(stream);
                },
                "TYPE" => {
                    Client::send200(stream);
                },
                "STRU" => {},
                "MODE" => {},
                "RETR" => {
                    Client::send150(stream);
                    let data = "hello hello testbytes".as_bytes();
                    self.clone().send_rudimentary_data(data);
                    Client::send226(stream);
                },
                "STOR" => {},
                "STOU" => {},
                "APPE" => {},
                "ALLO" => {},
                "REST" => {},
                "RNFR" => {},
                "RNTO" => {},
                "ABOR" => {},
                "DELE" => {},
                "RMD " => {},
                "MKD " => {},
                "PWD\r" => {
                    Client::send257(stream, "C:/Users/Downloads".to_string());
                },
                "LIST" => {
                    Client::send150(stream);
                    //send directory data over coolstreams
                    self.clone().sendDirectoryInfo();
                    Client::send226(stream);

                },
                "NLST" => {},
                "SITE" => {
                    Client::send502(stream);
                },
                "SYST" => {Client::send215(stream);},
                "STAT" => {},
                "HELP" => {},
                "NOOP" => {Client::send200(stream);},
                "OPTS" => {Client::send200(stream);},
                _ => Client::send502(stream),
            }
        }
    }

    fn sendDirectoryInfo(self: Arc<Self>){
        let mut lock = self.datastream.lock().unwrap();

        if let Some(mut stream) = lock.take() {
            stream.write(b"-rw-r--r--  1 user group  21 Oct  1 10:00 example.py\r\n").unwrap();
            stream.flush().unwrap();
        }
        drop(lock);
        self.clone().close_data_stream();
    }

    fn close_data_stream(self: Arc<Self>){
        let mut lock = self.datastream.lock().unwrap();
        if lock.is_some() {
            lock.take();
        }
    }
    
    fn send_rudimentary_data(self: Arc<Self>, data: &[u8]){
        let mut lock = self.datastream.lock().unwrap();
        if let Some(mut stream) = lock.take() {
            stream.write(&data).unwrap();
        }
        drop(lock);

        self.clone().close_data_stream();
    }


    fn handle_pasv(self: Arc<Self>,stream: &mut TcpStream){
       let list = Client::send227(stream);
       println!("stream: started waiting onconnection");
        let mut clie = list.accept().unwrap();
        println!("stream: accepted connection at {:?}",clie.1);
        self.datastream.lock().unwrap().replace(clie.0);
        //setConnection create field in Client, mutex<listner>.
    }


}