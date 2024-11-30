use std::{io::{self, Read, Write}, thread::{self, sleep}, time::Duration};
use rustp::RustTP;
fn main() {
    let server = RustTP::new();
    let mut guess = String::new();
    loop {
        io::stdin()
    .read_line(&mut guess).unwrap();
        match guess.trim() {
            "clear" => {print!("\x1B[2J\x1B[1;1H");
            std::io::stdout().flush().unwrap(); 
        },
            "exit" => {return;}
            _ => {print!("not valid")}
        }
        guess.clear();
    }
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