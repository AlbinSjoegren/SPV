# SPV

Calculates a stars position and velocity in the cartesian coordinate system.

![Screenshot_2021-11-10_22-32-21](https://user-images.githubusercontent.com/23136737/141196735-22a798b9-7920-46d0-97cb-ec847d1b12c4.png)

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

![image](https://user-images.githubusercontent.com/23136737/138668712-e3c6ab2f-90bd-486a-81ea-1ed1e62d3e0a.png)

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
