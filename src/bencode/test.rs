extern mod bencode;
use bencode::parser::Parser;
use bencode::common::Error;
use bencode::common::Bencode;
use bencode::common::{Null, Dictionary, List, String, Number};
use std::path::Path;
use std::io::fs::File;
use std::io::Reader;

#[test]
fn test_integer()
{
    let input = ~"i42e";

    let mut p = Parser::new(input.as_bytes());
    match p.parse() {
        Err(e) => fail!(e.msg),
        Ok(val) => match val {
            Number(num) => {
                assert!(num == 42);
            },
            _ => fail!()
        }
    }
}

#[test]
fn test_negative_integer()
{
    let input = ~"i-42e";

    let mut p = Parser::new(input.as_bytes());
    match p.parse() {
        Err(e) => fail!(e.msg),
        Ok(val) => match val {
            Number(num) => {
                assert!(num == -42);
            },
            _ => fail!()
        }
    }
}

#[test]
fn test_bytes()
{
    let input = ~"4:asdf";

    let mut p = Parser::new(input.as_bytes());
    match p.parse() {
        Err(e) => fail!(e.msg),
        Ok(val) => match val {
            String(bytes) => {
                assert!(bytes == (~"asdf").into_bytes());
            },
            _ => fail!()
        }
    }
}

#[test]
fn test_full_torrent()
{
    let path : Path   = Path::new("./test.torrent");
    let on_error      = || fail!("open of {:?} failed", path);
    let mut reader : File = File::open(&path).unwrap_or_else(on_error);

    let input = reader.read_to_end();
    let mut p = Parser::new(input);
    match p.parse() {
        Err(e) => fail!(e.msg),
        Ok(val) => match val {
            Dictionary(dict) => {
                match dict.find(&~"announce") {
                    Some(ben) => match(ben) {
                        &String(ref bufr) => {
                            let temp : ~str = std::str::from_utf8_owned(bufr.clone());
                            assert!(temp == ~"http://torrent.ubuntu.com:6969/announce")
                        },
                        _ => fail!("wrong type on announce")
                    },
                    None => fail!("no announce")
                };
                match dict.find(&~"comment") {
                    Some(ben) => match(ben) {
                        &String(ref bufr) => {
                            let temp : ~str = std::str::from_utf8_owned(bufr.clone());
                            assert!(temp == ~"Ubuntu CD releases.ubuntu.com")
                        },
                        _ => fail!("wrong type on announce")
                    },
                    None => fail!("no announce")
                };
            },
            _ => fail!()
        }
    }
}
