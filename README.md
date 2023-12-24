# Shuttle Christmas Code Hunt

This project contains my solutions for the [2023 Shuttle Christmas Code Hunt](https://console.shuttle.rs/cch). During
the code hunt, I learned a lot about Rust and its ecosystem. I also learned a lot about algorithms and data structures. 
I got in touch with many new concepts and ideas. I am very grateful for this experience. So thanks to the
[Shuttle](https://www.shuttle.rs) team and the community for making this possible.

The code here is still somewhat work in progress. I will try to improve it over time. There was not enough time here
and there to add proper tests etc. I will try to add them later.

## Build and Run

To run this project, you will need to have [Rust](https://www.rust-lang.org) installed. I recommend using [rustup](https://rustup.rs) 
to install and manage your Rust installation. Besides Rust, you will need cargo-shuttle. This can be installed with the following command:
```shell
$ cargo install cargo-shuttle
```
Once you have Rust and cargo-shuttle installed, you can build and run the project with the following command:
```shell
$ cargo shuttle run -r
```
This will create a [Postgres](https://hub.docker.com/_/postgres) docker container for persistance. After that, the 
[Axum](https://github.com/tokio-rs/axum) application should serve on `http://127.0.0.1:8000`.

## Validation

Shuttle created the [cch23-validator](https://crates.io/crates/cch23-validator) to test solutions. By running the 
following command, you can validate your solutions:
```shell
$ cch23-validator --all
```
The result should look something like this in the end:
```shell
⋆｡°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆
.・゜゜・・゜゜・．                .・゜゜・・゜゜・．
｡･ﾟﾟ･          SHUTTLE CCH23 VALIDATOR          ･ﾟﾟ･｡
.・゜゜・・゜゜・．                .・゜゜・・゜゜・．
⋆｡°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆


Validating Challenge -1...

Task 1: completed 🎉
Core tasks completed ✅
Task 2: completed 🎉

...

Validating Challenge 22...

Task 1: completed 🎉
Core tasks completed ✅
Task 2: completed 🎉
Bonus points: 600 ✨


Completed 17 challenges and gathered a total of 4430 bonus points.
```
