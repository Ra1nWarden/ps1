//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitor_count: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
	unsafe {
            	   visitor_count = visitor_count + 1;
	    }
        do spawn {
            let mut stream = stream;
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            let mut index: int = 0;
	    let mut file_name = ~"";
	    for splitted in request_str.split(' ') {
	    	if(index == 1) {
			 file_name.push_str(splitted);
	     		 break;
		}
		index = index + 1;
	    }
	    let mut response = ~"";
	    let homepage = match file_name.len() {
	    	1 => true,
		_ => false
	    };
	    if(homepage) {
	    	response.push_str("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 </body></html>\r\n");
            }
	    else {
	    	 let mut valid = true;
		 if(file_name.len() < 7) {
		 	valid = false;
		 }
		 if(valid) {
		 	let extension = file_name.slice_from(file_name.len() - 5).to_owned();
			let html_extension = ~".html";
			if(str::eq(&extension, &html_extension)) {
			     let file_name_abs = file_name.slice_from(1);
			     let path = Path::new(file_name_abs);
			     let opened_file: Option<File>;
			     if path.exists() && path.is_file() {
			     	opened_file = File::open(&path);
			     }
			     else {
			     	opened_file = None;
			     }
		             match opened_file {
			     	   Some(html_file) => {
				              let mut html_file_mut = html_file;
					      let msg_bytes: ~[u8] = html_file_mut.read_to_end();
					      response.push_str(str::from_utf8(msg_bytes));
				   },
				   None => {
				        println("not found!");
				   	valid = false;
				   }
			     }
			}
			else {
			     valid = false;
			}
		 }
		 if(!valid) {
		 	    response.push_str("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>HTTP 403 Not Found</title>
                 </head>
                 <body>
                 <h1>HTTP 403 Error</h1>
		 <p>Sorry, the page you requested does not exist. Please check the url.</p>
                 </body></html>\r\n");
		 }
	    }
            stream.write(response.as_bytes());
            println!("Connection terminates.");
	    unsafe {
	    	   println!("Visitor count: {:d}", visitor_count);
            }
	}
    }
}
