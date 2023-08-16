# daedalus
master server for the Icarus project


## Building for Raspberry Pi 
*note: only works on linux*

Install `cross` tool:
```bash
cargo install cross
```

Now, we can simply use `cross` instead of `cargo` when compiling, ensuring we still specify the target:
```bash
cross build --release –target=arm-unknown-linux-gnueabihf 
```