# SPV

[![Rust](https://github.com/AlbinSjoegren/SPV/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/AlbinSjoegren/SPV/actions/workflows/rust.yml)
![License](https://img.shields.io/github/license/AlbinSjoegren/SPV)
![Downloads](https://img.shields.io/github/downloads/AlbinSjoegren/SPV/total)
[![Discord](https://img.shields.io/discord/831904736219365417)](https://discord.gg/x2vwWx9SsS)
[![DOI](https://zenodo.org/badge/416674887.svg)](https://zenodo.org/badge/latestdoi/416674887)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/S6S77U98I)


Calculates a stars position and velocity in the cartesian coordinate system.

https://youtu.be/dsc2zcIbCNQ

![Screenshot from 2022-01-05 11-30-22](https://user-images.githubusercontent.com/23136737/148202851-88c5abf4-6d99-40d1-bfc5-8ae001f04017.png)

## Building and running

To build SPV you need rust, you can install it with [these instructions](https://www.rust-lang.org/learn/get-started).

Then simply run:

```
cargo run --release
```
in the directory with the `src` directory and `Cargo.toml`.

I _HIGHLY_ recommend you build from the latest created tag an not the main branch.

If you don't want to build it, then you can download the program from the _Releases_ panel.

On Linux simply give the AppImage executable rights (properties of the file if you right click on it) and double click it in your file manager. You could also run it as a script with `./filename.AppImage` in the terminal.

On Windows simply download the `.exe` and double click to run it.

The data will _ALLWAYS_ be in directories based on the name given under _General_. These directories will _ALLWAYS_ be in the same directory as the actual `.exe` or `.Appimage` file.

## How to use

Give the fields the inputs they want with correct units. You need to press enter after you finished your input. The row below will reflect your input if you did it correctly. Remember that the string to f64 conversion is very strict. No spaces allowed, only numbers and dots to signify decimals. Absolutely nothing else.
Apart from that, it is pretty straightforward. 

If you have any questions what so ever feel free to submit an issue and I will get to answering as quickly as possible 

Feature requests are also welcome.

## How it works

__SPV__ can currently do position and velocity calculations based on raw input data (spherical coordinates and velocities) but this is only really good for singular bodies and not for when you need multiple bodies to have a good enough accuracy to interact in a simulation of some kind. This is NOT due to __SPV__, the input data is just not good enough in most cases even for close stars. However, we have a system if you need the position and velocity of several interacting bodies using the orbital elements of the orbits. This uses eulers angle transforms to get a plane in which we have the intended ellipse, a is parallel with x, b with y, and z with the normal. We can take every point on the actual orbit and then draw a line from each point to the proposed position by the first method (the position we get here is the position for the companion and we might need to give the position of the primary to change the origin of the system). Then we can check for the shortest line and pick the point that it belongs to. That will be our new position relative to the primary. We can then use a velocity equation meant for satellites but defining mu with the orbital period, this is good enough to get the initial velocity.

## Todo

1. Expand the number of available operation
2. Batch processing by taking file inputs
3. Rust crate version
4. Organizing/file structure options for batch output
5. The ability to forward information from input files to the output files (example: _Luminosity_)
