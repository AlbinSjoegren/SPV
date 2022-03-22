//! # Welcome to `spv-rs`!
//!
//! [![crates](https://img.shields.io/crates/v/spv-rs)](https://crates.io/crates/spv-rs)
//! [![Rust](https://github.com/AlbinSjoegren/SPV/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/AlbinSjoegren/SPV/actions/workflows/rust.yml)
//! ![License](https://img.shields.io/github/license/AlbinSjoegren/SPV)
//! [![Discord](https://img.shields.io/discord/831904736219365417)](https://discord.gg/x2vwWx9SsS)
//! [![DOI](https://zenodo.org/badge/416674887.svg)](https://zenodo.org/badge/latestdoi/416674887)
//!
//! [![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/S6S77U98I)
//!
//! This crate is a set of functions for either extracting or manipulating astronomcial data.
//! However it is (at least for now) mainly focused on position and velocity data.
//!
//! ### Examples
//! First if you want to see a calculator like application usecase please check the sourcecode for the `SPV` gui utility at [SPV github repo](https://github.com/AlbinSjoegren/SPV/tree/main/SPV).
//!
//! Now we will look at the position function as an example:
//! For the [`position::position`] function we use three input variables. `Parallax` for the distance to the object, you can read more about parallax [here](https://en.wikipedia.org/wiki/Stellar_parallax).
//! `Right ascension` and `Declination` is basically a dots position on a sphere where the distance we get from the `Parllax` is the radius of the sphere.
//!
//! One easy way to use this function if you had the required variables would be like this:
//!
//! ```rust
//! use spv_rs::position::position;
//! use glam::f64::DVec3;
//!
//! fn main() {
//!     let parallax = 1.5_f64;
//!     let right_ascension = 35.8_f64;
//!     let declination = 67.3_f64;
//!
//!     let body_name = "x";
//!
//!     let position = position(parallax, right_ascension, declination).to_array();
//!
//!     println!("The body {} was at the position x: {}, y: {}, z: {} at epoch J2000",
//!     body_name, position[0], position[1], position[2]);
//! }
//! ```
//! The same general principles apply to most functions.
//! 
//! ### Extra
//!
//! Feel free to propose additions/changes, file issues and or help with the project over on [GitHub](https://github.com/AlbinSjoegren/SPV)!

/// Set of functions to calculate the position of either primary or companion bodies for diffrent usecases.
/// All outputs are in the cartesian coordinate system.
pub mod position {
    use super::common::semi_parameter;
    use super::common::true_anomaly;
    use super::coordinate_transforms::euler_angle_transformations;
    use glam::f32::Vec3;
    use glam::f64::{DVec2, DVec3};

    /// Position of a single celestial object relative to the sun.
    /// Can be used in conjuction with companion functions to place a twobody system relative to the sun.
    /// parallax is in mas (milliarcseconds), right_ascension is in degrees and declination in degrees.
    /// Output is a 3-dimensional vector with x, y and z in that order all in meters.
    pub fn position(parallax: f64, right_ascension: f64, declination: f64) -> DVec3 {
        let distance = 1. / (parallax / 1000.);

        let distnace_si = distance * (3.0856778570831 * 10_f64.powf(16.));

        let right_ascension_rad = right_ascension.to_radians();
        let declination_rad = (declination + 90.).to_radians();

        let x = distnace_si * right_ascension_rad.cos() * declination_rad.sin();

        let y = distnace_si * right_ascension_rad.sin() * declination_rad.sin();

        let z = distnace_si * declination_rad.cos();

        DVec3::new(x, y, z)
    }

    /// Same as [position::position] but with a f32 vector returned if you need that.
    pub fn position_f32(parallax: f32, right_ascension: f32, declination: f32) -> Vec3 {
        let distance = 1. / (parallax / 1000.);

        let distnace_si = distance * (3.0856778570831 * 10_f32.powf(16.));

        let right_ascension_rad = right_ascension.to_radians();
        let declination_rad = (declination + 90.).to_radians();

        let x = distnace_si * right_ascension_rad.cos() * declination_rad.sin();

        let y = distnace_si * right_ascension_rad.sin() * declination_rad.sin();

        let z = distnace_si * declination_rad.cos();

        Vec3::new(x, y, z)
    }

    /// Position on the surface of a sphere with radius in meters.
    pub fn position_surface(radius: f64, right_ascension: f64, declination: f64) -> DVec3 {
        let right_ascension_rad = right_ascension.to_radians();
        let declination_rad = (declination + 90.).to_radians();

        let x = radius * right_ascension_rad.cos() * declination_rad.sin();

        let y = radius * right_ascension_rad.sin() * declination_rad.sin();

        let z = radius * declination_rad.cos();

        DVec3::new(x, y, z)
    }

    /// Position of the companion star in a twobody system with no rotation applied.
    /// a is semi major-axis in au, e is eccentricity, period is in years and t_p is time since periastron in years.
    /// Output is a 2-dimensional vector with x and y in that order all in meters. We only need a 2-dimensional vector here
    /// due to the fact that everything is on a plane in 2D.
    pub fn companion_position(a: f64, e: f64, period: f64, t_p: f64) -> DVec2 {
        //Prep Values
        let p = semi_parameter(a, e);
        let v = true_anomaly(e, period, t_p);

        //Position of Companion in ellipse base
        let x = (p * v.cos()) / (1. + e * v.cos());
        let y = (p * v.sin()) / (1. + e * v.cos());

        DVec2::new(x, y)
    }

    /// Position of the companion star in a twobody system with rotation relative to the earth/sun plane applied.
    /// a is semi major-axis in au, e is eccentricity, period is in years, t_p is time since periastron in years,
    /// lotn is Longitude of the node (Omega) in degrees, aop is Argument of periastron (omega) in degrees and finally i is the Inclination in degrees.
    /// Output is a 3-dimensional vector with x, y and z in that order all in meters.
    pub fn companion_relative_position(
        a: f64,
        e: f64,
        period: f64,
        t_p: f64,
        lotn: f64,
        aop: f64,
        i: f64,
    ) -> DVec3 {
        //Prep Values
        let p = semi_parameter(a, e);
        let v = true_anomaly(e, period, t_p);

        //Position of Companion in ellipse base
        let x = (p * v.cos()) / (1. + e * v.cos());
        let y = (p * v.sin()) / (1. + e * v.cos());

        //Ellipse base
        let euler_angle_transformations = euler_angle_transformations(lotn, aop, i).to_cols_array();
        let x1 = euler_angle_transformations[0];
        let x2 = euler_angle_transformations[1];
        let x3 = euler_angle_transformations[2];

        let y1 = euler_angle_transformations[3];
        let y2 = euler_angle_transformations[4];
        let y3 = euler_angle_transformations[5];

        //Position in original base
        let companion_position_x = (x1 * x) + (y1 * y);
        let companion_position_y = (x2 * x) + (y2 * y);
        let companion_position_z = (x3 * x) + (y3 * y);

        DVec3::new(
            companion_position_x,
            companion_position_y,
            companion_position_z,
        )
    }
}

/// Set of functions to calculate the velocity of either primary or companion bodies for diffrent usecases.
/// All outputs are in the cartesian coordinate system.
pub mod velocity {
    use super::common::radius;
    use super::common::semi_parameter;
    use super::common::specific_mechanical_energy;
    use super::common::standard_gravitational_parameter;
    use super::common::true_anomaly;
    use super::coordinate_transforms::euler_angle_transformations;
    use super::position::position;
    use glam::f64::{DVec2, DVec3};

    /// Velocity of a single celestial object relative to the sun.
    /// Can be used in conjuction with companion functions to place a twobody system relative to the sun.
    /// parallax is in mas (milliarcseconds), right_ascension is in degrees and declination in degrees,
    /// proper_motion_ra is the right ascension part of the proper motion variable in as (arcseconds),
    /// proper_motion_dec is the declination part of the proper motion variable in as (arcseconds) and
    /// radial_velocity is in km/s.
    /// Output is a 3-dimensional vector with x, y and z in that order all in meters/second.
    pub fn velocity(
        parallax: f64,
        right_ascension: f64,
        declination: f64,
        proper_motion_ra: f64,
        proper_motion_dec: f64,
        radial_velocity: f64,
    ) -> DVec3 {
        let distance = 1. / (parallax / 1000.);

        //SI
        let distnace_si = distance * (3.0856778570831 * 10_f64.powf(16.));
        let radial_velocity_si = radial_velocity * 1000.;

        let proper_motion_x = distnace_si
            * (((right_ascension + ((proper_motion_ra * 0.00027777777777778) / 31556926.))
                .to_radians())
            .cos())
            * ((((declination + ((proper_motion_dec * 0.00027777777777778) / 31556926.)) + 90.)
                .to_radians())
            .sin());

        let proper_motion_y = distnace_si
            * (((right_ascension + ((proper_motion_ra * 0.00027777777777778) / 31556926.))
                .to_radians())
            .sin())
            * ((((declination + ((proper_motion_dec * 0.00027777777777778) / 31556926.)) + 90.)
                .to_radians())
            .sin());

        let proper_motion_z = distnace_si
            * ((((declination + ((proper_motion_dec * 0.00027777777777778) / 31556926.)) + 90.)
                .to_radians())
            .cos());

        let position = position(parallax, right_ascension, declination).to_array();

        let x = position[0];
        let y = position[1];
        let z = position[2];

        let proper_motion_vector_x = proper_motion_x - x;
        let proper_motion_vector_y = proper_motion_y - y;
        let proper_motion_vector_z = proper_motion_z - z;

        let normalized_vector_x = x / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt();
        let normalized_vector_y = y / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt();
        let normalized_vector_z = z / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt();

        let radial_velocity_vector_x = normalized_vector_x * radial_velocity_si;
        let radial_velocity_vector_y = normalized_vector_y * radial_velocity_si;
        let radial_velocity_vector_z = normalized_vector_z * radial_velocity_si;

        let x_v = radial_velocity_vector_x + proper_motion_vector_x;
        let y_v = radial_velocity_vector_y + proper_motion_vector_y;
        let z_v = radial_velocity_vector_z + proper_motion_vector_z;

        DVec3::new(x_v, y_v, z_v)
    }

    /// Velocity of the companion star in a twobody system with no rotation applied.
    /// a is semi major-axis in au, e is eccentricity, period is in years and t_p is time since periastron in years.
    /// Output is a 2-dimensional vector with x and y in that order all in meters/second. We only need a 2-dimensional vector here
    /// due to the fact that everything is on a plane in 2D.
    pub fn companion_velocity(a: f64, e: f64, period: f64, t_p: f64) -> DVec2 {
        //Prep Values
        let mu = standard_gravitational_parameter(a, period);
        let p = semi_parameter(a, e);
        let v = true_anomaly(e, period, t_p);

        //Velocity of Companion in ellipse base
        let x_v = (0. - ((mu / p).sqrt())) * (v.sin());
        let y_v = ((mu / p).sqrt()) * (e + (v.cos()));

        DVec2::new(x_v, y_v)
    }

    /// Velocity of the companion star in a twobody system with rotation relative to the earth/sun plane applied.
    /// a is semi major-axis in au, e is eccentricity, period is in years, t_p is time since periastron in years,
    /// lotn is Longitude of the node (Omega) in degrees, aop is Argument of periastron (omega) in degrees and finally i is the Inclination in degrees.
    /// Output is a 3-dimensional vector with x, y and z in that order all in meters/second.
    pub fn companion_relative_velocity(
        a: f64,
        e: f64,
        period: f64,
        t_p: f64,
        lotn: f64,
        aop: f64,
        i: f64,
    ) -> DVec3 {
        //Prep Values
        let mu = standard_gravitational_parameter(a, period);
        let p = semi_parameter(a, e);
        let v = true_anomaly(e, period, t_p);

        //Velocity of Companion in ellipse base
        let x_v = (0. - ((mu / p).sqrt())) * (v.sin());
        let y_v = ((mu / p).sqrt()) * (e + (v.cos()));

        //Ellipse base
        let euler_angle_transformations = euler_angle_transformations(lotn, aop, i).to_cols_array();
        let x1 = euler_angle_transformations[0];
        let x2 = euler_angle_transformations[1];
        let x3 = euler_angle_transformations[2];

        let y1 = euler_angle_transformations[3];
        let y2 = euler_angle_transformations[4];
        let y3 = euler_angle_transformations[5];

        //Velocity in original base
        let companion_velocity_x = (x1 * x_v) + (y1 * y_v);
        let companion_velocity_y = (x2 * x_v) + (y2 * y_v);
        let companion_velocity_z = (x3 * x_v) + (y3 * y_v);

        DVec3::new(
            companion_velocity_x,
            companion_velocity_y,
            companion_velocity_z,
        )
    }

    /// Just the companion velocity but as a value and not coordinates.
    pub fn companion_velocity_value(a: f64, e: f64, period: f64, t_p: f64) -> f64 {
        let mu = standard_gravitational_parameter(a, period);
        let epsilon = specific_mechanical_energy(a, e);
        let r = radius(a, e, period, t_p);

        (2. * ((mu / r) + epsilon)).sqrt()
    }
}

/// Set of common functions used by `spv-rs` exposed if you want to used them for your own calculations.
pub mod common {
    use super::coordinate_transforms::euler_angle_transformations;
    use super::position::companion_relative_position;
    use super::velocity::companion_relative_velocity;
    use glam::f64::DVec3;

    /// Takes a in as (arcseconds) and parllax in mas (milliarcsecond) and outputs a in au.
    pub fn a_to_au(parallax: f64, a: f64) -> f64 {
        a * parallax_to_parsec(parallax)
    }

    /// Calculates total declination in degrees with declination_degree, declination_min and declination_s in degrees, minutes and seconds respectively.
    pub fn declination_total(
        declination_degree: f64,
        declination_min: f64,
        declination_s: f64,
    ) -> f64 {
        declination_degree + (declination_min / 60.) + (declination_s / 3600.)
    }

    /// Calculates total right ascension in degrees with right_ascension_h, right_ascension_min and right_ascension_s in hours, minutes and seconds respectively.
    pub fn right_ascension_total(
        right_ascension_h: f64,
        right_ascension_min: f64,
        right_ascension_s: f64,
    ) -> f64 {
        (right_ascension_h * 15.)
            + (right_ascension_min * (1. / 4.))
            + (right_ascension_s * (1. / 240.))
    }

    /// Calculates r min or the minimum distance between the primary and companion boides in a twobody system also known as perigee
    /// (suffix may change depending on what object it reffers to).
    /// Output is just the x coordinate in the ellipses plane in au.
    pub fn perigee(a: f64, e: f64) -> f64 {
        a * (1. - e)
    }

    /// Calculates r max or the maximum distance between the primary and companion boides in a twobody system also known as apogee
    /// (suffix may change depending on what object it reffers to).
    /// Output is just the x coordinate in the ellipses plane in au.
    pub fn apogee(a: f64, e: f64) -> f64 {
        a * (1. + e)
    }

    /// Calculates r min or the minimum distance between the primary and companion boides in a twobody system also known as perigee
    /// (suffix may change depending on what object it reffers to).
    /// Output is 3-dimensional vector that represents the coordinates for perigee rotated to be relative to the earth/sun plane in meters.
    pub fn relative_perigee(a: f64, e: f64, lotn: f64, aop: f64, i: f64) -> DVec3 {
        let a_si = au_to_m(a);
        let x = a_si * (1. - e);

        let euler_angle_transformations = euler_angle_transformations(lotn, aop, i).to_cols_array();
        let x1 = euler_angle_transformations[0];
        let x2 = euler_angle_transformations[1];
        let x3 = euler_angle_transformations[2];

        DVec3::new(x * x1, x * x2, x * x3)
    }

    /// Calculates r max or the maximum distance between the primary and companion boides in a twobody system also known as apogee
    /// (suffix may change depending on what object it reffers to).
    /// Output is 3-dimensional vector that represents the coordinates for apogee rotated to be relative to the earth/sun plane in meters.
    pub fn relative_apogee(a: f64, e: f64, lotn: f64, aop: f64, i: f64) -> DVec3 {
        let a_si = au_to_m(a);
        let x = a_si * (1. + e);

        let euler_angle_transformations = euler_angle_transformations(lotn, aop, i).to_cols_array();
        let x1 = euler_angle_transformations[0];
        let x2 = euler_angle_transformations[1];
        let x3 = euler_angle_transformations[2];

        DVec3::new(x * x1, x * x2, x * x3)
    }

    /// Calculates the eccentric anomaly in degrees
    pub fn eccentric_anomaly(e: f64, period: f64, t_p: f64) -> f64 {
        //SI units
        let p_si = period * 31557600.;
        let t_p_si = t_p * 31557600.;

        //Defining angles
        let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
        let mut ecc_anom = mean_anom;
        for _i in (0..=20).step_by(1) {
            ecc_anom = mean_anom + (e * ecc_anom.sin());
        }

        ecc_anom
    }

    /// Calculates the true anomaly in degrees
    pub fn true_anomaly(e: f64, period: f64, t_p: f64) -> f64 {
        //SI units
        let p_si = period * 31557600.;
        let t_p_si = t_p * 31557600.;

        //Defining angles
        let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
        let mut ecc_anom = mean_anom;
        for _i in (0..=20).step_by(1) {
            ecc_anom = mean_anom + (e * ecc_anom.sin());
        }

        2. * (((1. + e) / (1. - e)).sqrt() * (ecc_anom * 0.5).tan()).atan()
    }

    /// Calculates the flight path angle for the companion body in degrees
    pub fn flight_path_angle(e: f64, period: f64, t_p: f64) -> f64 {
        //SI units
        let p_si = period * 31557600.;
        let t_p_si = t_p * 31557600.;

        //Defining angles
        let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
        let mut ecc_anom = mean_anom;
        for _i in (0..=20).step_by(1) {
            ecc_anom = mean_anom + (e * ecc_anom.sin());
        }

        ((e * ecc_anom.sin()) / ((1. - ((e.powf(2.)) * (ecc_anom.cos().powf(2.)))).sqrt()))
            .asin()
            .to_degrees()
    }

    /// Calculates the semi parameter for a twobody system and returns it in meters.
    pub fn semi_parameter(a: f64, e: f64) -> f64 {
        let a_si = au_to_m(a);
        let b_si = semi_minor_axis(a, e);

        (b_si.powf(2.)) / a_si
    }

    /// Calculates the semi minor axis for a twobody system and returns it in meters.
    pub fn semi_minor_axis(a: f64, e: f64) -> f64 {
        let a_si = au_to_m(a);

        a_si * ((1. - e.powf(2.)).sqrt())
    }

    /// Calculates the total radius for a twobody system
    pub fn radius(a: f64, e: f64, period: f64, t_p: f64) -> f64 {
        let nu = true_anomaly(e, period, t_p);
        let p = semi_parameter(a, e);

        p / (1. + (e * nu.cos()))
    }

    /// Calculates the specific angular momentum value
    pub fn specific_angular_momentum_value(a: f64, e: f64, period: f64) -> f64 {
        let p = semi_parameter(a, e);
        let mu = standard_gravitational_parameter(a, period);

        (mu * p).sqrt()
    }

    /// Calculates the specific angular momentum coordinates
    pub fn specific_angular_momentum_coordinates(
        a: f64,
        e: f64,
        period: f64,
        t_p: f64,
        lotn: f64,
        aop: f64,
        i: f64,
    ) -> DVec3 {
        let r = companion_relative_position(a, e, period, t_p, lotn, aop, i);
        let v = companion_relative_velocity(a, e, period, t_p, lotn, aop, i);

        DVec3::cross(r, v)
    }

    /// Calculates the stadard gravitational parameter
    pub fn standard_gravitational_parameter(a: f64, period: f64) -> f64 {
        let period_si = period * 31557600.;
        let a_si = au_to_m(a);
        

        ((a_si.powf(3.)) * 4. * (std::f64::consts::PI.powf(2.))) / (period_si.powf(2.))
    }

    /// Specific mechanical energy (used by other equation but exposed here if you need it)
    pub fn specific_mechanical_energy(a: f64, period: f64) -> f64 {
        let a_si = au_to_m(a);
        let mu = standard_gravitational_parameter(a, period);

        0. - (mu / (2. * a_si))
    }

    /// If you dind't have the period already
    pub fn period(a: f64, mu: f64) -> f64 {
        let a_si = au_to_m(a);

        2. * std::f64::consts::PI * (((a_si.powf(3.)) / mu).sqrt())
    }

    /// If you for some reason had these parameters and not a then here ya go
    pub fn semi_major_axis(
        standard_gravitational_parameter: f64,
        specific_mechanical_energy: f64,
    ) -> f64 {
        0. - (standard_gravitational_parameter / (2. * specific_mechanical_energy))
    }

    /// Mean motion or n
    pub fn mean_motion(a: f64, period: f64) -> f64 {
        let a_si = au_to_m(a);
        let mu = standard_gravitational_parameter(a, period);

        (mu / (a_si.powf(3.))).sqrt()
    }

    /// If you for some reason had these parameters and not e then here ya go.
    pub fn eccentricity(
        standard_gravitational_parameter: f64,
        specific_mechanical_energy: f64,
        specific_angular_momentum_value: f64,
    ) -> f64 {
        (1. - ((2. * specific_mechanical_energy * (specific_angular_momentum_value.powf(2.)))
            / (standard_gravitational_parameter.powf(2.))))
        .sqrt()
    }

    /// Distance between one foci and the center of the ellipse in au.
    pub fn linear_eccentricity(a: f64, e: f64) -> f64 {
        a * e
    }

    /// Flattening is another way to defining eccentricity for an ellipse.
    pub fn flattening(a: f64, e: f64) -> f64 {
        let a_si = au_to_m(a);
        let b = semi_minor_axis(a_si, e);

        (a_si - b) / a_si
    }

    /// Parallax to parsec convert function where the parallax parameter is in mas (milliarcseconds).
    pub fn parallax_to_parsec(parallax: f64) -> f64 {
        1. / (parallax / 1000.)
    }

    /// Simple function motsly used by `spv-rs` to convert au to meters (used for semi major-axis in most cases but can be used for other stuff too).
    pub fn au_to_m(a: f64) -> f64 {
        a * 1000. * 149597870.7
    }

    ///
    pub fn apparent_mag_to_absolute_mag(parallax: f64, apparent_magnitude: f64) -> f64 {
        apparent_magnitude + 5. * (parallax.log10() + 1.)
    }

    /// 
    pub fn temperature(b_v_index: f64) -> f64 {
        4600. * ((1. / ((0.92 * b_v_index) + 1.7)) + (1. / ((0.92 * b_v_index) + 0.62)))
    }
}

/// Transform fucntions used by `spv-rs` but exposed her if you want to use them yourself.
pub mod coordinate_transforms {
    use glam::f64::{DMat3, DVec3};

    /// Method for getting base manipulation matrix that is used to rotate the companion star in a twobody system
    /// relative to the earth/sun plane.
    /// lotn is Longitude of the node (Omega) in degrees, aop is Argument of periastron (omega) in degrees and finally i is the Inclination in degrees.
    /// Output is a 3-dimensional matrix with x1, x2 and x3 in the first collum, y1, y2 and y3 in the second collum and z1, z2 and z3 in the third collum.
    pub fn euler_angle_transformations(lotn: f64, aop: f64, i: f64) -> DMat3 {
        //In rad
        let lotn_rad = lotn.to_radians();
        let aop_rad = aop.to_radians();
        let i_rad = i.to_radians();

        //Ellipse base
        let x1 = (lotn_rad.cos() * aop_rad.cos()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.sin());
        let x2 = (lotn_rad.sin() * aop_rad.cos()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.sin());
        let x3 = i_rad.sin() * aop_rad.sin();

        let y1 = ((0. - lotn_rad.cos()) * aop_rad.sin())
            - (lotn_rad.sin() * i_rad.cos() * aop_rad.cos());
        let y2 = ((0. - lotn_rad.sin()) * aop_rad.sin())
            + (lotn_rad.cos() * i_rad.cos() * aop_rad.cos());
        let y3 = i_rad.sin() * aop_rad.cos();

        let z1 = i_rad.sin() * lotn_rad.sin();
        let z2 = (0. - i_rad.sin()) * lotn_rad.cos();
        let z3 = i_rad.cos();

        DMat3::from_cols(
            DVec3::new(x1, x2, x3),
            DVec3::new(y1, y2, y3),
            DVec3::new(z1, z2, z3),
        )
    }
}

/// Basic csv parsing for extracting real world data or any old data table you want to parse really.
/// To get a csv if you got some other format from something like [Vizier](https://vizier.cds.unistra.fr/viz-bin/VizieR)
/// I would recomend a tool like [Topcat](http://www.star.bris.ac.uk/~mbt/topcat/).
pub mod input_data {
    use csv::{ReaderBuilder, Terminator};
    use std::error::Error;

    /// General usecase parsing function for csv files.
    pub fn parse_csv<T: for<'de> serde::Deserialize<'de>>(filename: &str, has_headers: bool, cols_split: u8, row_split: u8) -> Result<std::vec::Vec<T>, Box<dyn Error>> {
        let mut vec = vec![];
        let mut rdr = ReaderBuilder::new()
            .delimiter(cols_split)
            .terminator(Terminator::Any(row_split))
            .has_headers(has_headers)
            .from_path(filename)?;
        for result in rdr.deserialize() {
            let record: T = result?;
            vec.push(record);
        }
        Ok(vec)
    }
}

/// Basic csv writing utility for saving large datasets to file.
pub mod output_data {
    use csv::{Terminator, WriterBuilder};
    use std::error::Error;

    /// General usecase writing function for csv files.
    pub fn write_csv<T: serde::Serialize>(
        output_filename: &str, has_headers: bool, cols_split: u8, row_split: u8,
        vec: std::vec::Vec<T>,
    ) -> Result<(), Box<dyn Error>> {
        let mut writer = WriterBuilder::new()
            .delimiter(cols_split)
            .terminator(Terminator::Any(row_split))
            .has_headers(has_headers)
            .from_path(output_filename)?;

        for n in vec {
            writer.serialize(n)?;
        }

        Ok(())
    }
}

/// Specific usecase functions for NBSS 
pub mod nbss {   
    use serde::Deserialize;
    use serde::Serialize; 

    /// NBSS Input struct
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NBSSInputCollums {
        pub name: String,
        pub mass: f64,
        pub radius: f64,
        pub reference_body: String,
        pub a: f64,
        pub e: f64,
        pub period: f64,
        pub time_since_periapsis: f64,
        pub lotn: f64,
        pub aop: f64,
        pub i: f64,
    }

    /// NBSS Output struct
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NBSSOutputCollums {
        pub name: String,
        pub mass: f64,
        pub radius: f64,
        pub reference_body: String,
        pub e: f64,
        pub semi_parameter: f64,
        pub position_x: f64,
        pub position_y: f64,
        pub position_z: f64,
        pub velocity_x: f64,
        pub velocity_y: f64,
        pub velocity_z: f64,
    }

    /// NBSS Calc struct (can be used as an example of what you can do with SPV)
    pub fn position_and_velocity_twobody_serialized(input_filename: &str, output_filename: &str) {
        let mut file:std::vec::Vec<NBSSInputCollums> = vec![];
        match super::input_data::parse_csv(input_filename, false, b',', b'\n') {
            Ok(vec) => file = vec,
            Err(ex) => {
                println!("ERROR -> {}", ex);
            }
        };

        let mut vec = vec![];

        for n in file {
            if n.a == 0. && n.e == 0.{
                let row = NBSSOutputCollums {
                    name: n.name,
                    mass: n.mass,
                    radius: n.radius,
                    reference_body: n.reference_body,
                    e: 0.,
                    semi_parameter: 0.,
                    position_x: 0.,
                    position_y: 0.,
                    position_z: 0.,
                    velocity_x: 0.,
                    velocity_y: 0.,
                    velocity_z: 0.,
                };
    
                vec.push(row);
            }
            else {
                let pos = super::position::companion_relative_position(
                    n.a,
                    n.e,
                    n.period,
                    n.time_since_periapsis,
                    n.lotn,
                    n.aop,
                    n.i,
                )
                .to_array();
                let vel = super::velocity::companion_relative_velocity(
                    n.a,
                    n.e,
                    n.period,
                    n.time_since_periapsis,
                    n.lotn,
                    n.aop,
                    n.i,
                )
                .to_array();
                let semi_parameter = super::common::semi_parameter(n.a, n.e);
    
                let row = NBSSOutputCollums {
                    name: n.name,
                    mass: n.mass,
                    radius: n.radius,
                    reference_body: n.reference_body,
                    e: n.e,
                    semi_parameter,
                    position_x: pos[0],
                    position_y: pos[1],
                    position_z: pos[2],
                    velocity_x: vel[0],
                    velocity_y: vel[1],
                    velocity_z: vel[2],
                };
    
                vec.push(row);
            }
        }

        match super::output_data::write_csv(output_filename, false, b',', b'\n', vec) {
            Ok(_) => (),
            Err(ex) => {
                println!("ERROR -> {}", ex);
            }
        };
    }


    //WIP
    /*
    pub fn position_and_velocity_relative_twobody(input_filename: &str, output_filename: &str) {

    }

    pub fn position_and_velocity_relative(input_filename: &str, output_filename: &str) {

    }
    */
}

// WIP
/*
pub fn above(
    radius_primary: f64,
    observers_longitude_primary: f64,
    observers_latitude_primary: f64,
    obliquity_of_the_ecliptic_primary: f64,
    rotation_rate_primary: f64,
    lotn_primary: f64,
    aop_primary: f64,
    i_primary: f64,
    distance: f64,
    a_companion: f64,
    e_companion: f64,
    period_companion: f64,
    t_p_companion: f64,
    lotn_companion: f64,
    aop_companion: f64,
    i_companion: f64,

) {

    let observer_declination =  ((observers_latitude_primary.sin() * obliquity_of_the_ecliptic_primary.cos()) + (observers_latitude_primary.cos() * obliquity_of_the_ecliptic_primary.sin() * observers_longitude_primary.sin())).asin();
    let observer_right_ascension = ((observers_latitude_primary.cos() * observers_longitude_primary.cos()) / observer_declination.cos()).acos();

    let position_surface = position_surface(radius_primary, observer_right_ascension, observer_declination);

    let primary_rotation = euler_angle_transformations(lotn_primary, aop_primary, i_primary);

    let companion_rotation = euler_angle_transformations(lotn_companion, aop_companion, i_companion);

    let new_companion_rotation = companion_rotation - primary_rotation;

    let companion_position = companion_relative_position(a_companion, e_companion, period_companion, t_p_companion, lotn_companion, aop_companion, i_companion);

    let primary_velocity = (2. * std::f64::consts::PI * radius_primary) / rotation_rate_primary;
}
*/
