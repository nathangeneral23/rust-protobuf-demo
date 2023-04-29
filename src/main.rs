use std::{fs::{self, File}, io::Write};

use bytes::{BytesMut, BufMut};
use prost::Message;

mod lib;

use lib::proto;

fn main() {
    let foo = proto::types::Foo {
        text: "Hello, World!".to_owned(),
        author: Some("NG".to_owned()),
    };

    println!("{:#?}", foo);

    let mut buf = BytesMut::new();

    match proto::types::Foo::encode(&foo, &mut buf) {
        Ok(_) => {
            let mut file = File::create("./out/foo.buf").unwrap();
            file.write_all(&buf).unwrap();
        },
        Err(err) => panic!("{}", err),
    }

    match fs::read("./out/foo.buf") {
        Ok(file) => {
            let mut buf = BytesMut::new();

            buf.put_slice(&file);

            match proto::types::Foo::decode(buf) {
                Ok(foo) => {
                    println!("{:#?}", foo);
                },
                Err(err) => panic!("{}", err),
            }
        },
        Err(err) => panic!("{}", err),
    }
}
