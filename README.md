needs `cargo-wasi`

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
