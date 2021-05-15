# ❤️ LUVE RUST ❤️ #2

It's a personal project that allows me to growth my skills in the [Rust language](https://www.rust-lang.org/) and how to use the [WebGPU API](https://gpuweb.github.io/gpuweb/) with [wgpu-rs](https://github.com/gfx-rs/wgpu-rs). This project also allows me to learn how to create an [ECS architecture](https://en.wikipedia.org/wiki/Entity_component_system) myself in [Rust](https://www.rust-lang.org/).

## Prerequisite

* [Rust](https://www.rust-lang.org/) installed of course.
* Because I use [shaderc crate](https://crates.io/crates/shaderc) you will need to have [Python 3](https://www.python.org/) and [CMake 3](https://cmake.org/) installed on your computer.

> I don't know if you need to have [Vulkan]() already installed on your computer, tell me if its the case.

## How to run

```zsh
# Clone the repos
git clone https://github.com/5aitama/wgpu-ecs-rs.git $CUSTOM_DIR

# Move into the cloned repo and run
cd $CUSTOM_DIR && cargo run --bin main
```