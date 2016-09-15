extern crate ssh2;
extern crate ruru;
// extern crate thrussh_client;

use std::net::TcpStream;
use self::ssh2::Session;
// use self::thrussh_client::*;
use ruru::types::Argc;
use ruru::{AnyObject, Boolean, Class, NilClass, RString, VM};
// use std::io::Write;
use std::path::Path;

extern fn connect(_: Argc, _: *const AnyObject, _: AnyObject) -> NilClass {
    // let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let tcp = TcpStream::connect("192.241.226.7:22").unwrap();
    let mut sess = Session::new().unwrap();

    match sess.handshake(&tcp) {
        Ok(_) => println!("hands shaken"),
        Err(e) => {
            println!("handshake error: {}", e.message());
            return NilClass::new()
        }
    }

    // match sess.userauth_password("sloveless", "voyageur##") {
    let pubkey_file = Path::new("/Users/sloveless/.ssh/id_rsa.pub");
    let privkey_file = Path::new("/Users/sloveless/.ssh/id_rsa");

    match sess.userauth_pubkey_file("root", Some(pubkey_file), privkey_file, None) {
        Ok(_) => println!("agent authed"),
        Err(e) => {
            println!("auth failure: {}", e.message());
            return NilClass::new()
        }
    }

    // // Make sure we succeeded
    // assert!(sess.authenticated());
    // let mut client = thrussh_client::Client::new();
    // client.set_host("192.241.226.7");
    // // client.set_auth_user("sloveless");
    // // client.set_auth_password("voyageur##".to_string());
    // println!("Valid auth methods: {:?}", client.session.valid_auth_methods());
    // let mut client = client.connect().unwrap();

    // if let Some(key) = client.authenticate().unwrap() {
    //     client.learn_host(&key).unwrap();
    //     assert!(client.authenticate().unwrap().is_none());
    //     println!("connected");
    // }

    NilClass::new()
}

pub extern fn init() {
    let mut rfile = Class::new("RSSH");

    Class::from_existing("RSSH").define(|itself| {
        itself.def("connect", connect);
    });
}
