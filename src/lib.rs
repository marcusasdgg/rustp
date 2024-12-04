use core::panic;
use std::env::Args;
use std::fmt::format;
use std::fs::{self, metadata, read, DirEntry, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::IpAddr;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, UNIX_EPOCH};
use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, TcpStream},
    thread::Thread,
};
use std::{path, thread};

type ArcClient = Arc<Client>;
pub struct RustTP {
    server_con: TcpListener,

    //this is a list of top directorys which comprise the / directory of the fs,
    directories: Arc<Vec<String>>,
    client_addrs: Mutex<HashMap<SocketAddr, ArcClient>>,
    add: Arc<String>,
    root_store: Arc<String>,
}

struct Client {
    addr: SocketAddr,
    stream: Mutex<TcpStream>,
    datastream: Mutex<Option<TcpStream>>,
    stream_type: StreamType,
    current_path: Arc<Mutex<String>>,
    real_path: Arc<Mutex<String>>,
    base_directories:  Arc<Vec<String>>,
    add: Arc<String>,
    root_store: Arc<String>,
}

enum StreamType {
    ASCII,
    BINARY,
    EBCDIC,
}

static DIRECTORYMODE: i32 = 1;

#[allow(dead_code)]
trait SendResponse {
    //series 100
    fn send110(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send125(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send150(stream: &mut TcpStream) {
        stream
            .write(b"150 Opening data connection for file list.\r\n")
            .unwrap();
        stream.flush().unwrap();
        ;
    }

    // series 200
    fn send200(stream: &mut TcpStream) {
        
        stream.write_all(b"200 command ok\r\n").unwrap();
        stream.flush().unwrap();
    }
    fn send202(stream: &mut TcpStream) {
        stream.write_all(b"202 no action needed\r\n").unwrap();
    }

    fn send211(stream: &mut TcpStream) {
        stream.write_all(b"211-Features:\r\nEPSV\r\n211 End\r\n").unwrap();
        stream.flush().unwrap();
    }

    fn send212(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send213(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send214(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send215(stream: &mut TcpStream) {
        stream.write_all(b"215 UNIX Type: L8\r\n").unwrap();
        stream.flush().unwrap();
    }

    fn send220(stream: &mut TcpStream) {
        stream.write_all(b"220 FTP Server ready\r\n").unwrap();
        stream.flush().unwrap();
    }

    fn send221(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send225(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send226(stream: &mut TcpStream) {
        stream.write_all(b"226 Transfer complete\r\n").unwrap();
        stream.flush().unwrap();
    }

    fn send227(stream: &mut TcpStream, addr: String) -> TcpListener {
        let listener = TcpListener::bind("0.0.0.0:0").unwrap(); // 0.0.0.0 binds to any available IP on a random port

        let port = listener.local_addr().unwrap().port(); // Get the randomly assigned port

        // Convert the port to the format required by PASV (p1, p2)
        let p1 = port / 256;
        let p2 = port % 256;
        println!("ad {addr}");
        let ad: Vec<String> = addr.clone().split(".").map(|e| e.to_string()).collect();

        let binding = listener.local_addr().unwrap().ip().to_string();
        let vec: Vec<&str> = binding.split(".").collect();

        let str = format!("227 Entering Passive Mode ({},{},{},{},{},{})\r\n", ad[0],ad[1],ad[2],ad[3],p1, p2);
        //vec[0], vec[1], vec[2],vec[3]
        println!("port is {port}");

        stream.write_all(str.as_bytes()).unwrap();
        stream.flush().unwrap();
        listener
    }

    fn send228(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send229(stream: &mut TcpStream) -> TcpListener {
        let listener = TcpListener::bind("0.0.0.0:0").unwrap(); 
        let port = listener.local_addr().unwrap().port();
        println!("{}", format!("229 (|||{port}|)\r\n"));
        stream.write_all(format!("229 Entering Extended Passive Mode (|||{port}|)\r\n").as_bytes()).unwrap();
        stream.flush().unwrap();
        listener
    }

    fn send230(stream: &mut TcpStream) {
        stream.write_all(b"230 User logged in\r\n").unwrap();
        stream.flush().unwrap();
    }

    fn send232(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send234(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send235(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send250(stream: &mut TcpStream) {
        stream.write_all(b"250 DIRSTYLE set to WINDOWS\r\n").unwrap();
    }

    fn send257(stream: &mut TcpStream, path: String) {
        print!("path is {path}");
        stream
            .write(format!("257 \"{path}\" is directory\r\n").as_bytes())
            .unwrap();
        stream.flush().unwrap();
        ;
    }

    //series 300

    fn send300(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send331(stream: &mut TcpStream) {
        stream.write_all(b"331 Guest login okay, send your complete email address as your password.\r\n").unwrap();
        stream.flush().unwrap();
    }

    fn send332(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send334(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send336(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    //series 400
    fn send421(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send425(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send426(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send430(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send431(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send434(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send450(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send451(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send452(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    //series 500
    fn send500(stream: &mut TcpStream){
        stream.write_all(b"500 do not try again fucker\r\n").unwrap();
    }

    fn send501(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send502(stream: &mut TcpStream) {
        stream.write_all(b"502 Command not implemented.\r\n").unwrap();
    }

    fn send503(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send504(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send530(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send532(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send533(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send534(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send535(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send536(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send537(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send550(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send551(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send552(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send553(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    //series 600

    fn send600(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send631(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send632(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    fn send633(stream: &mut TcpStream) {
        stream.write_all(b"110 MARK yyyy = mmmm\r\n").unwrap();
    }

    //new commands
}

trait SendData {}

//what this project is, i want this to be a ftp file server for windows
// initial MVP:
// directory designation
// file retreival
// active mode.
//anonymous no auth check

impl RustTP {
    pub fn new_with_paths(path_list: Vec<String>,address: &str, root_store: &str) -> Arc<Self> {
        let toc = TcpListener::bind("0.0.0.0:21").unwrap();
        let rs = Arc::new(RustTP {
            server_con: toc,
            client_addrs: Mutex::new(HashMap::new()),
            directories: Arc::new(path_list),
            add: Arc::new(address.to_owned()),
            root_store: Arc::new(root_store.to_owned()),
        });
        rs.clone().start_event_loop();
        rs
    }

    pub fn new(address: &str,root_store: &str) -> Arc<Self> {
        let toc = TcpListener::bind(address.to_owned() + ":21").unwrap();
        let rs = Arc::new(RustTP {
            server_con: toc,
            client_addrs: Mutex::new(HashMap::new()),
            directories: Arc::new(Vec::new()),
            add: Arc::new(address.to_owned()),
            root_store: Arc::new(root_store.to_owned()),
        });
        rs.clone().start_event_loop();
        rs
    }

    fn start_event_loop(self: Arc<Self>) {
        thread::spawn(move || {
            while let Ok(t) = self.server_con.accept() {
                let stream = t.0;
                let addr = t.1;
                println!("new client connected with {:?}", addr);
                let client = Client::new(addr, stream, self.directories.clone(), self.add.clone(), self.root_store.clone());
                let mut lock = self.client_addrs.lock().unwrap();
                lock.insert(addr, client);
                drop(lock);
            }
        });
    }


}

impl SendResponse for Client {}

impl Client {
    fn new(addr: SocketAddr, stream: TcpStream, dir:  Arc<Vec<String>>, ad: Arc<String>, root_store: Arc<String>) -> ArcClient {
        //loop
        println!("ad: {ad}");
        let s = Arc::new(Client {
            addr,
            stream: Mutex::new(stream),
            datastream: Mutex::new(None),
            stream_type: StreamType::ASCII,
            current_path: Arc::new(Mutex::new(String::from("/"))),
            base_directories: dir,
            real_path: Arc::new(Mutex::new("/".to_string())),
            add: ad,
            root_store
        });
        s.clone().start_event_loop();
        return s;
    }

    fn start_event_loop(self: Arc<Self>) {
        thread::spawn(move || {
            let mut buf: [u8; 1000] = [0; 1000];
        let mut stream = self.stream.lock().unwrap();
        Client::send220(&mut stream);
        stream.flush().unwrap();
        let stream = &mut stream;
        stream.set_read_timeout(Some(Duration::new(200, 0))).unwrap();
        while let Ok(size) = stream.read(&mut buf) {
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
                    Client::send331(stream);
                }
                "PASS" => {
                    Client::send230(stream);
                }
                "ACCT" => {}
                "CWD " => {
                    let s = comm[4..].trim().to_string();
                    let mut iter = s.split("/");
                    let firstDir = iter.next().unwrap();
                    if let Some(found) = self.base_directories.iter().find(|string| {
                        string.split("/").last().unwrap() == firstDir
                    }){
                        let mut l2k = self.real_path.lock().unwrap();
                        *l2k = found.to_owned().to_owned() + "/" + &iter.map(|d| d.to_owned() + "/").collect::<String>();
                        println!("real path is {l2k}");
                    } else {
                        let mut l2k = self.real_path.lock().unwrap();
                        *l2k = "/".to_string();
                        println!("real path is /");
                    }
                
                    let mut lock = self.current_path.lock().unwrap();
                    let mut clone = s.clone();
                    if !clone.starts_with("/"){
                        clone.insert(0, '/');
                    }
                    println!("{clone} : os the new directory");
                    *lock = clone;
                    Client::send200(stream);
                }
                "CDUP" => {}
                "SMNT" => {}
                "QUIT" => {
                    panic!();
                }
                "REIN" => {}
                "PORT" => {
                    let args = comm[4..].trim();
                    self.clone().handle_active(args);
                    Client::send226(stream);
                }
                "PASV" => {
                    self.clone().handle_pasv(stream);
                }
                "TYPE" => {
                    Client::send200(stream);
                }
                "STRU" => {}
                "MODE" => {}
                "RETR" => {
                    Client::send150(stream);
                    let args =  comm[4..].trim();
                    self.clone().send_from_buffered(args);
                    Client::send226(stream);
                }
                "STOR" => {
                    Client::send150(stream);
                    let args = comm[4..].trim();
                    self.clone().receive_file(args);
                    Client::send226(stream);
                }
                "STOU" => {}
                "APPE" => {}
                "ALLO" => {}
                "REST" => {}
                "RNFR" => {}
                "RNTO" => {}
                "ABOR" => {}
                "DELE" => {}
                "RMD " => {}
                "MKD " => {
                    let args = comm[4..].trim();
                    self.clone().create_dir(args);
                    Client::send200(stream);
                }
                "PWD\r" => {
                    let path = self.current_path.lock().unwrap();
                    Client::send257(stream, path.clone());
                }
                "LIST" => {
                    Client::send150(stream);
                    //send directory data over coolstreams
                    self.clone().sendDirectoryInfo();
                    Client::send226(stream);
                }
                "NLST" => {}
                "SITE" => {
                    Client::send502(stream);
                }
                "SYST" => {
                    Client::send215(stream);

                }
                "STAT" => {
                    Client::send502(stream);
                }
                "HELP" => {}
                "NOOP" => {
                    Client::send200(stream);
                }
                "OPTS" => {
                    Client::send200(stream);
                }
                "FEAT" => {
                    Client::send211(stream);
                },
                "EPSV" => {
                    if size > 6{
                        Client::send200(stream);
                    } else {
                        self.clone().handle_epsv(stream);
                    }                    
                }
                _ => {
                    println!("command was not parsed: {comm}");
                    Client::send500(stream)
                },
            }
        }
        });
        
    }

    fn top_dir_to_real_path(self: Arc<Self>, name: &str) -> Option<String>{
        return self.base_directories.iter().find(|dir| {
            dir.split("/").last().unwrap() == name
        }).cloned();
    }

    fn create_dir(self: Arc<Self>, name: &str){
        let parsedpath = if name.contains("/") {
            let mut liter = name.split("/");
            let top_most = liter.next().unwrap();
            let path = self.clone().top_dir_to_real_path(top_most).unwrap();
            let total_path = path + "/" + liter.map(|pat| pat.to_owned() + "/").collect::<String>().as_str();
            total_path
        } else {
            self.root_store.clone().to_string() + "/" + name
        };
        fs::create_dir(parsedpath).unwrap();
    }


    fn receive_file(self: Arc<Self>,name: &str){
        let parsedpath = if name.contains("/") {
            let mut liter = name.split("/");
            let top_most = liter.next().unwrap();
            let path = self.clone().top_dir_to_real_path(top_most).unwrap();
            let total_path = path + "/" + liter.map(|pat| pat.to_owned() + "/").collect::<String>().as_str();
            total_path
        } else {
            self.root_store.clone().to_string() + "/" + name
        };

        let parsedpath = if parsedpath.ends_with("/") {parsedpath.strip_suffix("/").unwrap().to_string()} else {parsedpath};

        println!("path to store file is {}",parsedpath);
        let mut file = File::create(parsedpath).unwrap();
        let mut  reader = self.datastream.lock().unwrap();
        let mut buffer = [0u8;32000];

        println!("startng reading process");
        if let Some(mut reeder) = reader.take() {
            reeder.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            println!("set timeout");
            while let Ok(size) = reeder.read(&mut buffer) {
                if size == 0{
                    println!("file contents finished.");
                    break;
                }
                let data = &buffer[..size];
                println!("buffer ereceievedf");
                file.write(data).unwrap();
            }
        }else {
            println!("reader was empty?");
        }
 
    }

    fn sendDirectoryInfo(self: Arc<Self>) {
        let mut lock = self.datastream.lock().unwrap();
        let curr = self.current_path.lock().unwrap().clone();
        let mut ls_list: String = String::new();
        if curr == "/" {
            //print everything in top directories list.
            ls_list = self.base_directories.iter().map(|dir| {
                let metadata = metadata(dir).unwrap();
                let filetype = if metadata.is_dir() {'d'} else {'-'};
                let file_size = metadata.len();
                let name = Path::file_name(Path::new(dir)).unwrap().to_str().unwrap();
                let links = if metadata.is_dir() {2} else {1};
                //let permissions = if metadata.is_dir() {"drwxr-xr-x"} else {"rw-r--r--"};
                format!("{filetype}rw-r--r-- {links} user group {file_size} Oct 1 10:00 {name}\r\n")
            }).collect::<String>();
        } else {
            let realpath = self.real_path.lock().unwrap().clone();
            println!("path given is {realpath}");
            let entries = fs::read_dir(realpath).unwrap();
            ls_list = entries.map(|entry| {
                get_file_info(&entry.unwrap())
            }).collect::<String>();
    
        }

        //println!("directory {curr} list of stuff {:?}",ls_list);

        if let Some(mut stream) = lock.take() {
            stream.write_all(ls_list.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
        }
        drop(lock);
        self.clone().close_data_stream();
    }
    fn handle_epsv(self: Arc<Self>, stream: &mut TcpStream){
        let list = Client::send229(stream);
        println!("stream: started waiting onconnection {:?}",list);
        let clie = list.accept().unwrap();
        println!("stream: accepted connection at {:?}", clie.1);
        self.datastream.lock().unwrap().replace(clie.0);
    }
    fn close_data_stream(self: Arc<Self>) {
        let mut lock = self.datastream.lock().unwrap();
        if lock.is_some() {
            drop(lock.take());
        }
    }

    fn send_rudimentary_data(self: Arc<Self>, data: &[u8]) {
        let mut lock = self.datastream.lock().unwrap();
        if let Some(mut stream) = lock.take() {
            stream.write_all(&data).unwrap();
        }
        drop(lock);

        self.clone().close_data_stream();
    }

    fn send_from_buffered(self: Arc<Self>, file_name: &str) {

        let currdir = self.real_path.lock().unwrap().clone();

        let top_most = file_name.split("/").next().unwrap();

        let real = self.base_directories.iter().find(|e| {
            let bot_most = e.split("/").last().unwrap();
            bot_most.trim() == top_most
        });

        
        let mut ite = file_name.split("/").into_iter();
        ite.next();
        let relpath = real.unwrap().to_owned() + "/" + &ite.map(|e| e.to_owned() + "/").collect::<String>();
        let s = real.unwrap();
        println!("path foudn now :{} given {s}", relpath);
        let relpath = if relpath.ends_with("/") {relpath.strip_suffix("/").unwrap().to_string()} else {relpath};
        let file = File::open(relpath).unwrap();
        let mut buffer = vec![0u8; 32000];
        let mut reader = BufReader::new(file);
        let mut lock = self.datastream.lock().unwrap();
        if let Some(mut stream) = lock.take() {
            stream.set_nodelay(true).unwrap();
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break, // EOF reached
                    Ok(bytes_read) => {
                        stream.write_all(&buffer[..bytes_read]).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        break;
                    }
                }
            }
        }
    }

    fn handle_pasv(self: Arc<Self>, stream: &mut TcpStream) {
        let list = Client::send227(stream, self.add.clone().to_string());
        println!("stream: started waiting onconnection {:?}",list);
        let clie = list.accept().unwrap();
        println!("stream: accepted connection at {:?}", clie.1);
        self.datastream.lock().unwrap().replace(clie.0);
        //setConnection create field in Client, mutex<listner>.
    }

    fn handle_active(self: Arc<Self>, addrstr: &str) {
        let numbers: Vec<&str> = addrstr.split(",").collect();
        let port = numbers[4].parse::<i32>().unwrap() * 256 + numbers[5].parse::<i32>().unwrap();
        let ip = numbers[..4].join(".");
        println!("active add given: {}:{}", ip, port);
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
        self.datastream.lock().unwrap().replace(stream);
    }
}


impl Drop for Client {
    fn drop(&mut self) { 
        println!("client dropped");
     }
}

fn get_file_info(entry: &DirEntry) -> String {
    let metadata = entry.metadata().unwrap();
    
    // File permissions
    let permissions = metadata.permissions();
    let links = if metadata.is_dir() {2} else {1};
    let filetype = if (metadata.is_dir()) {'d'} else {'-'};
    // File size
    let file_size = metadata.len();
    let binding = entry.path().display().to_string();
    let name = binding.split("/").last().unwrap();
    let name =  name.split("\\").last().unwrap();
    println!("{} interpreted",name);

    
    // Last modified time
    let modified_time = metadata.modified().unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH).unwrap()
        .as_secs();
    let s = format!(
        "{filetype}rw-r--r-- {links} user group {file_size} Oct 1 10:00 {name}\r\n"
    );
    s
}

//make paths vector editable in runtime for makedir.