use std::io::Reader;
use extra::treemap::TreeMap;
use std::str;
use common::Error;
use common::Bencode;
// srsly rust?
use common::{Null, Dictionary, List, String, Number};
use std::vec::append_one;
use std::from_str::FromStr;

pub struct Parser<'p> {
    priv data: &'p [u8],
    priv chr: u8,
    priv pos: uint
}

impl<'p> Parser<'p> {

    pub fn new(data: &'p [u8]) -> Parser<'p>
    {
        let mut P = Parser {
            data: data,
            chr: 0,
            pos: 0
        };

        return P
    }

    pub fn parse(&mut self) -> Result<Bencode, Error>
    {
        match self.parse_next() {
            Ok(value) => {
                // skip left tailing whitespaces
                self.skip_whitespace();

                if self.eof() {
                    return Ok(value)
                }
                else {
                    return self.error(~"tailing character")
                }
            }
            Err(e) => Err(e)
        }
    }

    fn eof(&self) -> bool
    {
        self.data.len() <= self.pos
    }

    fn skip_whitespace(&mut self)
    {

    }

    fn find(&mut self, chr: u8) -> Option<uint>
    {
        let mut start = self.pos;
        let end   = self.data.len();
        while start < end {
            if self.data[start] == chr {
                return Some(start);
            }
            start += 1
        }
        None
    }

    fn error<T>(&self, msg: ~str) -> Result<T, Error> {
        Err(Error { col: self.pos, msg: msg })
    }

    fn parse_next(&mut self) -> Result<Bencode, Error>
    {
        match self.data[self.pos] {
            0x64u8 => self.parse_dictionary(),
            0x69u8 => self.parse_integer(),
            0x6Cu8 => self.parse_list(),
            0x31u8 .. 0x39u8 => self.parse_bytes(),
            _ => self.error(~"invalid syntax")
        }
    }

    fn parse_dictionary(&mut self) -> Result<Bencode, Error>
    {
        let mut d : ~TreeMap<~str, Bencode> = ~TreeMap::new();
        let mut key: ~str = ~"";
        self.pos += 1;
        while self.data[self.pos] != 0x65u8 {
            match self.parse_bytes() {
                Ok(value) => {
                    match value {
                        String(buffr) => {
                            key = str::from_utf8_owned(buffr)
                        },
                        _ => {
                            return self.error(~"invalid key")
                        }
                    }
                },
                Err(err) => {
                    return Err(err)
                }
            }

            match self.parse_next() {
                Ok(value) => {
                    d.insert(key, value)
                },
                Err(err) => {
                    return Err(err)
                }
            };
        }
        self.pos += 1;
        Ok(Dictionary(d))
    }

    fn parse_integer(&mut self) -> Result<Bencode, Error>
    {
        self.pos += 1;
        match self.parse_number::<int>(0x65u8) {
            None => self.error(~"invalid integer"),
            Some(val) => Ok(Number(val))
        }
    }

    fn parse_number<T: FromStr>(&mut self, limit: u8) -> Option<T>
    {
        let end = self.find(limit);
        println!("parse_number {}", self.pos)
        match end {
            None => None,
            Some(num) => {
                let slice = str::from_utf8(
                    self.data.slice_from(self.pos)
                                .slice_to(num - self.pos)
                );
                self.pos = num + 1;
                let result : Option<T> = from_str::<T>(slice);
                result
            }
        }
    }

    fn parse_list(&mut self) -> Result<Bencode, Error>
    {
        let mut list : List = ~[];
        self.pos += 1;
        while self.data[self.pos] != 0x65u8 {
            match self.parse_next() {
                Ok(value) => {
                    list.push(value)
                },
                Err(err) => {
                    return Err(err)
                }
            };
        }
        self.pos += 1;
        return Ok(List(list));
    }

    fn parse_bytes(&mut self) -> Result<Bencode, Error>
    {
        match self.parse_number::<uint>(0x3Au8) {
            None => {
                self.error(~"invalid byte length")
            },
            Some(len) => {
                let slice = self.data.slice_from(self.pos)
                                      .slice_to(len);

                self.pos = self.pos + len;
                Ok(String(slice.into_owned()))
            }
        }
    }
}

pub fn from_reader(rdr: &mut Reader) -> Result<Bencode, Error>
{
    let s = rdr.read_to_end();
    let mut parser = Parser::new(s);

    parser.parse()
}
pub fn from_string(data: ~str) -> Result<Bencode, Error>
{
    let mut parser = Parser::new(data.as_bytes());
    parser.parse()
}
