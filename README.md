# Blockochen

This project is the blockchain powering PitPet hackathon project at Wasmer.
Learn more about it here: https://wasmer.io/posts/wasm-on-amazon-lambda-lessons-learned

## How to build

This project needs `cargo-wasi`

```shell
$ make
```

then:

```shell
$ wasmer run target/wasm32-wasi/debug/blockochen.wasm
```

Request format:

```
{"type":"newBlockchain"}
{"type":"addBlock","birth_data":[116,101,115,116],"data":[102,111,111,98,97,114]}
{"type":"getTTL","birth_hash":[116,101,115,116]}
{"type":"getEvents","birth_hash":[116,101,115,116]}
{"type":"quit"}
```
