# Unscramble

Web app to unscramble a list of letters into a valid English word.

List of English words provided by [English words](https://github.com/dwyl/english-words).

## Building

```sh
git clone https://github.com/Celeo/Unscramble --recurse-submodules
cp ./english-words/words_alpha.txt server/src
cd server
cargo build
```

## Running

```sh
cd server
cargo run
```

Example using [httpie](https://httpie.org/):

```sh
http :3030/word/tcat
```
