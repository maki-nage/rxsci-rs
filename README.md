# RxSci-Rs

This is the - under development - Rust backend for RxSci.


build:

cd python
cargo build --release
python setup.py build --force
python setup.py develop
python test.py


backlog:

- use cbindgen to generate c header
- split crate in rxsci and rxsci-sys
- state: add global-scope support
- state: add copy operation for base types
- python callback: implement flextuple access code-gen
- python callback: implement VM.
- flextuple: rename to natuple?
- flextuple: add datetime and date types
