# SPV

Calculates a stars position and velocity in the cartesian coordinate system.

![Screenshot_20211126_212905](https://user-images.githubusercontent.com/23136737/143627341-dd6598b4-4dde-4d64-98db-f3c59813a4be.png)

## Building and running

To build SPV you need rust, you can install it with [these instructions](https://www.rust-lang.org/learn/get-started).

Then simply run:

```
cargo run --release
```
in the directory with the `src` directory and `Cargo.toml` in it.

I _HIGHLY_ recommend you build from the latest created tag an not the main branch.

If you don't want to build it, then you can download the program from the _Releases_ panel.

On Linux simply give the AppImage executable rights (properties of the file if you right click on it) and double click it in your file manager. You could also run it as a script with `./filename.AppImage` in the terminal.

On Windows simply download the `.exe` and double click to run it.

The data will _ALLWAYS_ be in directories based on the name given under _General_. These directories will _ALLWAYS_ be in the same directory as the actual `.exe` or `.Appimage` file.

## Todo

1. Expand the number of available operation
2. Batch processing by taking file inputs
3. Rust crate version
4. Organizing/file structure options for batch output
5. The ability to forward information from input files to the output files (example: _Luminosity_)

## What it can't do

1. Get any radial velocity if the position is 0
2. Yeah that's about it :)
