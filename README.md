Lander
======

Lander mini game implemented using Rust and WebGPU. [Online demo](https://andrepuel.github.io/lander/).

![image Screenshot](./readme/ss.png)

Controls
--------

 * Left arrow key - activates left booster
 * Right arrow key - activates right booster
 * Up arrow key - activates central booster

Steer the space ship by using the lateral boosters to change the direction the ship is pointing. Use the central booster to move forward.

Compiling
---------
### Desktop version

The desktop version runs over Wgpu. The binary project is on the subfolder `bin` and may be launched quickly by running the following command:

    cargo run --manifest-path bin/Cargo.toml

### Wasm version
Run the NPM scripts to invoke the build:

    npm run build:dev
