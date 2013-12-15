use extra::treemap::TreeMap;

#[deriving(Clone, Eq)]
pub enum Bencode {
    Number(int),
    String(~[u8]),
    List(List),
    Dictionary(~Dictionary),
    Null
}

pub type List = ~[Bencode];
pub type Dictionary = TreeMap<~str, Bencode>;


#[deriving(Eq)]
pub struct Error {
    /// The column number at which the error occurred
    col: uint,
    /// A message describing the type of the error
    msg: ~str,
}
