# Instructions
Install wasm-pack with:

`cargo install wasm-pack`
(You might need to add wasm-pack to path to call it ```export PATH="$HOME/.cargo/bin:$PATH"```)

Then build project with:

`wasm-pack build --target web`

Next start python webserver

`python -m http.server`

open browser on `localhost:8000` and open pkg/ directory.


Test roms are sourced from https://github.com/Timendus/chip8-test-suite
