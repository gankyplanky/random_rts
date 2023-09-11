# random_rts
A starcraft inspired RTS made in rust, with SDL2. This is a work in progress and is intended to be playable on Linux and Windows.

# Try it
If you want to test it out you can clone the repo:
```
git clone https://github.com/gankyplanky/random_rts.git
```
### Note:
You need cargo and rust to build it, [get them here](https://www.rust-lang.org/tools/install).<br>
As well as git, [here](https://git-scm.com/downloads).<br>

After, enter the newly created folder and execute the following commands:
```
cargo install cargo-vcpkg
cargo vcpkg build
cargo build --release
```
Then navigate to 'target/release' and execute random_rts.exe, while in the game press ESC to quit.
