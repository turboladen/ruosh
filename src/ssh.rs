extern crate ssh2;
extern crate ruru;
// extern crate thrussh_client;

use std::net::TcpStream;
use self::ssh2::Session;
// use self::thrussh_client::*;
use self::ruru::types::Argc;
use self::ruru::{AnyObject, Class, Object, NilClass, RString, VM};
// use std::io::Write;
use std::path::Path;

use ruby::exceptions;

struct SSH {
    host: String,
    port: u8,
    username: String,
    password: String,
    identity_file: String
}

impl SSH {
    pub fn new(host: String, username: String, password: String) -> SSH {
        SSH {
            host: host,
            port: 22,
            username: username,
            password: password,
            identity_file: "/Users/sloveless/.ssh/id_rsa.pub".to_string()
        }
    }

    // Make sure to build a string that is: ip:port.
    fn host_and_port(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

extern fn connect(argc: Argc, argv: *const AnyObject, _: AnyObject) -> NilClass {
    let args = VM::parse_arguments(argc, argv);

    if args.len() != 3 {
        exceptions::raise_argument_error(1usize, args.len());
    }

    let (hostname, username, password) = extract_password_auth_args(args);

    let my_ssh = SSH::new(hostname, username, password);
    let tcp = TcpStream::connect(my_ssh.host_and_port().as_str()).unwrap();
    let mut sess = Session::new().unwrap();

    match sess.handshake(&tcp) {
        Ok(_) => println!("hands shaken"),
        Err(e) => {
            println!("handshake error: {}", e.message());
            return NilClass::new()
        }
    }

    match sess.userauth_password(my_ssh.username.as_str(), my_ssh.password.as_str()) {
        Ok(_) => {
            assert!(sess.authenticated());
            println!("agent authed")
        },
        Err(e) => {
            println!("auth failure: {}", e.message());
            return NilClass::new()
        }
    }

    // let pubkey_file = Path::new("/Users/sloveless/.ssh/id_rsa.pub");
    // let privkey_file = Path::new("/Users/sloveless/.ssh/id_rsa");

    // match sess.userauth_pubkey_file("sloveless", Some(pubkey_file), privkey_file, None) {
    //     Ok(_) => println!("agent authed"),
    //     Err(e) => {
    //         println!("auth failure: {}", e.message());
    //         return NilClass::new()
    //     }
    // }

    NilClass::new()
}

fn extract_password_auth_args(args: Vec<AnyObject>) -> (String, String, String) {
    let hostname = args[0].try_convert_to::<RString>().unwrap().to_string();
    let username = args[1].try_convert_to::<RString>().unwrap().to_string();
    let password = args[2].try_convert_to::<RString>().unwrap().to_string();

    (hostname, username, password)
}

pub extern fn init() {
    Class::from_existing("Rosh").define(|rosh| {
        rosh.define_nested_class("SSH", None).define(|itself| {
            itself.def("connect", connect);
        });
    });
}
