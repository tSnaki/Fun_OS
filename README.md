# A Fun Os

## Description
I decided to make this os because it sounded fun; however, I'll probably get bored of it before it becomes anywhere neer finished.

This os is developed with Rust and the Uefi package [here](https://rust-osdev.github.io/uefi-rs/introduction.html)

## Required setup for building
You'll need rust, qemu, and make installed on your computer.

You'll need to add the files OVMF_CODE.fd and OVMF_VARS.fd to the root of the project. (A tutorial for both this can be found in the rust Uefi package docs).

You'll then need to change the target in the makefile to the target that you're computer will need.

Then, run`make build` or `make run`.
