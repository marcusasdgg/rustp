use std::{env, fs::File, io::{self, BufRead, Read, Write}, path::Path, thread::{self, sleep}, time::Duration};
use basicftp::RustTP;
fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_lines("paths.txt").unwrap();
    
    let list = lines.flatten().map(|e| e.to_string()).collect::<Vec<String>>();
    //let list = vec!("C:/Users/marcu/Downloads".to_string(), "F:/Anime".to_string(), "C:/Users/marcu/Documents".to_string());
    let (server,thread) = RustTP::new_with_paths(list, &args[1], &args[2]);
    thread.join().unwrap();
}   

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//let s = std::net::TcpListener::bind("0.0.0.0:21").unwrap();
// while let t = s.accept() {
//     let p = t.unwrap();
//     let p = thread::spawn(move || {
//         println!("new device connected with deets {:?}",p.0);
//         let mut stream = p.0;
//         let mut byteArr: Vec<u8> = vec![0; 1024];
//         stream.write(b"220 FTP Server ready\r\n").unwrap();
//         stream.flush().unwrap();
//         sleep(Duration::from_millis(1000));
//         while let Ok(size) = stream.read(&mut byteArr){
//             let data = &byteArr[..size];
//             if size == 0 {
//                 println!("connection closed");
//                 break;
//             }
//             let str = String::from_utf8_lossy(data);
//             println!("New message arrived of size {size}\n\t\"{:?}\"", str);
//         }
//     });
// }