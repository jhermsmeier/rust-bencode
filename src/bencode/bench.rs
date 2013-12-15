extern mod bencode;
extern mod extra;
use std::path::Path;
use std::io::fs::File;
use std::io::Reader;
use bencode::parser::Parser;
use bencode::common::Error;
use bencode::common::Bencode;
use bencode::common::{Null, Dictionary, List, String, Number};

#[bench]
fn bench_integer(b: &mut extra::test::BenchHarness)
{
    let input = "i42e".as_bytes();
    b.iter(|| {
        let mut p = Parser::new(input);
        p.parse();
    })
}

#[bench]
fn bench_short_string(b: &mut extra::test::BenchHarness)
{
    let input = "4:asdf".as_bytes();
    b.iter(|| {
        let mut p = Parser::new(input);
        p.parse();
    })
}

#[bench]
fn bench_longer_string(b: &mut extra::test::BenchHarness)
{
    let input = "400:asdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdf".as_bytes();
    b.iter(|| {
        let mut p = Parser::new(input);
        p.parse();
    })
}

// this benchmark is currently to slow to be shown. bug in rust
#[bench]
fn bench_full_torrent(b: &mut extra::test::BenchHarness)
{
    let path : Path   = Path::new("./test.torrent");
    let on_error      = || fail!("open of {:?} failed", path);
    let mut reader : File = File::open(&path).unwrap_or_else(on_error);

    let input = reader.read_to_end();
    b.iter(|| {
        println!("RUNNING {}", input.len())
        let mut p = Parser::new(input);
        p.parse();
        println!("DONE")
    })
}
