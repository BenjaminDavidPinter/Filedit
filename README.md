# Png Parser

The goal of this project is to read, parse, and make sense of a png file's bytes via the spec, and only the spec.

Current Chunk Completion;
- Png Signature
- IHDR
- iCCP (Partial)


## 3/30/23
The next step is understanding how these zlib/deflate compression libraries work.


Example output;

```
===="IHDR"====
Width: 1446
Height: 986
Bit Depth: 8
Color Type: TruecolorWithAlpha
Compression Method: DeflateInflate
Filter Method: Method0
Interface Method: Method0
===="iCCP"====
Profile Name: "ICC Profile"
Compression Method: Method0
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }', src/png/chunks/iCCP.rs:35:59
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```