extern mod bencode;

use std::path::Path;
use std::io::fs::File;
use std::io::Reader;
use bencode::parser::{Parser, from_reader};
use bencode::common::{Bencode, Error, Null, Dictionary, List, String, Number};

fn test_str(input: ~str)
{
    let mut parser = Parser::new(input.into_bytes());
    let result = parser.parse();
    match result {
        Err(err) => println(err.msg),
        Ok(val) => print_bencode(val)
    }
}


fn print_bencode(data: Bencode)
{
    match data {
        Number(val) => println!(" int({})", val),
        String(val) => println!(" bytes({})", std::str::from_utf8_owned(val)),
        List(val) => {
            println("list(");
            for dat in val.move_iter() {
                print_bencode(dat);
            }
            println(")");
        },
        Dictionary(val) => {
            println("dictionary(");
            for (key, dat) in val.move_iter() {
                println!(" {} => ", key);
                print_bencode(dat);
            }
            println(")");
        }
        _ => println("got a result")
    }
}

fn main() {
    let path : Path   = Path::new("./test.torrent");
    let on_error      = || fail!("open of {:?} failed", path);
    let mut reader : File = File::open(&path).unwrap_or_else(on_error);

    test_str(~"i42e");
    test_str(~"5:hello");
    test_str(~"li32ei52ee");
    test_str(~"li32e5:helloe");
    test_str(~"d5:hello5:worlde");

    let mut parser = Parser::new(reader.read_to_end());
    let result = parser.parse();
    match result {
        Err(err) => println(err.msg),
        Ok(val) => print_bencode(val)
    }
}
