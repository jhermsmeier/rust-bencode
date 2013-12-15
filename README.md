# bencode

A rust library for (encoding and) decoding bencoded data,
according to the [BitTorrent specification](http://www.bittorrent.org/beps/bep_0003.html).

## About BEncoding

from [Wikipedia](https://en.wikipedia.org/wiki/Bencoding):

Bencode (pronounced like B encode) is the encoding used by the peer-to-peer
file sharing system BitTorrent for storing and transmitting loosely structured data.

It supports four different types of values:
- byte strings
- integers
- lists
- dictionaries

Bencoding is most commonly used in torrent files.
These metadata files are simply bencoded dictionaries.

## Install / Build

Just clone this repo, then use "rustpkg build bencode" to build everything.

## Performance

### decode
```
running 4 tests
test bench_full_torrent  ... bench:         0 ns/iter (+/- 0)
test bench_integer       ... bench:       958 ns/iter (+/- 103)
test bench_longer_string ... bench:     30719 ns/iter (+/- 3476)
test bench_short_string  ... bench:      1658 ns/iter (+/- 60)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```

Currently benchmarks aren't really usefull because there was/is a bug in rust
that breaks benchmarks that take more then 1ms.
That basically means 2 things: currently no good comparison AND we need to
become faster!
