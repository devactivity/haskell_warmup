# haskell_warmup

Belajar dasar pemograman FFI Haskell &amp; Rust

## Usage

Jalankan langkah-langkah berikut:

```bash
cd quicksort/
cargo build --release

cd ..
ghc -o sort_example app/Main.hs -lquicksort -L./quicksort/target/release
```
