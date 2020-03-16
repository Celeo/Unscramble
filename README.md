# Unscramble

Web app to unscramble a list of letters into a valid English word.

List of English words provided by [English words](https://github.com/dwyl/english-words).

## Building

```sh
git clone https://github.com/Celeo/Unscramble --recurse-submodules
```

### Client

You'll need [elm](https://elm-lang.org/) installed somehow. Through npm is probably the easiest: `npm i -g elm`.

```sh
cd client
./build.sh
```

### Server

You'll need [Rust](https://www.rust-lang.org/).

```sh
cp ./english-words/words_alpha.txt server/src
cd server
cargo build
```

## Running

In 2 separate terminal windows:

1. `cd client && python3 -m http.server 8000` or whatever static file server you prefer, like one from npm
1. `cd server && cargo run`

Take note that the _first_ request made to the server will take several seconds. This is normal - the server
is doing a bunch of text processing. Subsequent requests will be fast.
