# BUILD MANUAL
- This `README` file serves as a comprehensive guide on how to build and run the application.
- For now, all you have to do is build it for the bare-metal environment of `thumbv7em-none-eabihf`, which got no underlying operating system. In order to build it for the bare-metal environment, you need to do the following:
  - Install the `thumbv7em-none-eabihf` environment by running: `rustup add target thumbv7em-none-eabihf`
  - Build the application by running: `cargo build --target thumbv7em-none-eabihf`

- You can also build it natively to your target machine operating system. To do native builds, you can run the following commands:
  - For `linux` system, run: `cargo rustc -- -C link-arg=-nostartfiles`
  - For `windows` system, run: `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`
  - For `mac` system, run: `cargo rustc -- -C link-args="-e __start -static -nostartfiles"`

