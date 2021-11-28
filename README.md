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

## Governing equations

### Position

![eq1](https://quicklatex.com/cache3/e4/ql_0f90d3f97a91306f78ad27e2e8a555e4_l3.png)

Where x, y and z are the coordinates in the cartesian coordinate system in [km].

ra is Right ascension in [degrees] and dec is Declination in [degrees]

### Radial velocity

![image](https://user-images.githubusercontent.com/23136737/138673318-70199707-b2e7-48f1-aefc-1c7e17bb3417.png) 

x_u, y_u and z_u are the coordinates for a unitvector pointing in the direction of the _Radial Velocity_ vector.

Removing the **0**'s is used for positive _Radial Velocity_ vectors.

x, y and z are still the position coordinates in cartesian coordinates [km].

![image](https://user-images.githubusercontent.com/23136737/138673372-e8fee272-fe89-49a7-b22c-41afdc4554ee.png) 

Multiplying by the radial velocity gives us the coordinates of the resulting vector.


### Proper motion

![Screenshot_2021-11-10_22-29-13](https://user-images.githubusercontent.com/23136737/141196522-d05177d0-b6fd-4052-b36e-5d7d9a0d26a8.png)

Getting the position the object will be at in one second by using the same formula as for position.

![Screenshot_2021-11-10_22-29-29](https://user-images.githubusercontent.com/23136737/141196405-2007ae92-b2a0-4685-82d9-62cb3d37b9df.png)

Resetting the origin of the vector to the current position

![image](https://user-images.githubusercontent.com/23136737/138673506-a15b866f-e014-43a6-a1cf-da586e3e2ef1.png) 

Multiplying by the new position gives us the coordinates of the resulting vector.

### Total velocity

![image](https://user-images.githubusercontent.com/23136737/138673587-a335d274-a141-4f42-a88b-ad8a474b14a6.png) 

Just taking the sum of the two velocities.
