use glam::f64::{DMat3, DVec2, DVec3};

pub fn companion_relative_position(
    a: f64,
    e: f64,
    period: f64,
    t_p: f64,
    lotn: f64,
    aop: f64,
    i: f64,
) -> DVec3 {
    //SI units
    let p_si = period * 31557600.;
    let t_p_si = t_p * 31557600.;
    let a_si = a * 1000.;

    //In rad
    let lotn_rad = lotn.to_radians();
    let aop_rad = aop.to_radians();
    let i_rad = i.to_radians();

    //Defining angles
    let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
    let mut ecc_anom = mean_anom;
    for _i in (0..=20).step_by(1) {
        ecc_anom = mean_anom + (e * ecc_anom.sin());
    }

    //Defining the semi minor-axis
    let b_si = a_si * ((1. - e.powf(2.)).sqrt());

    //Prep Values
    let p = (b_si.powf(2.)) / a_si;
    let v = 2. * (((1. + e) / (1. - e)).sqrt() * (ecc_anom * 0.5).tan()).atan();

    //Position of Companion in ellipse base
    let x = (p * v.cos()) / (1. + e * v.cos());
    let y = (p * v.sin()) / (1. + e * v.cos());

    //Ellipse base
    let x1 = (lotn_rad.cos() * aop_rad.cos()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.sin());
    let x2 = (lotn_rad.sin() * aop_rad.cos()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.sin());
    let x3 = i_rad.sin() * aop_rad.sin();

    let y1 =
        ((0. - lotn_rad.cos()) * aop_rad.sin()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.cos());
    let y2 =
        ((0. - lotn_rad.sin()) * aop_rad.sin()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.cos());
    let y3 = i_rad.sin() * aop_rad.cos();

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

pub fn companion_relative_velocity(
    a: f64,
    e: f64,
    period: f64,
    t_p: f64,
    lotn: f64,
    aop: f64,
    i: f64,
) -> DVec3 {
    //SI units
    let p_si = period * 31557600.;
    let t_p_si = t_p * 31557600.;
    let a_si = a * 1000.;

    //In rad
    let lotn_rad = lotn.to_radians();
    let aop_rad = aop.to_radians();
    let i_rad = i.to_radians();

    //Defining angles
    let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
    let mut ecc_anom = mean_anom;
    for _i in (0..=20).step_by(1) {
        ecc_anom = mean_anom + (e * ecc_anom.sin());
    }

    //Defining the semi minor-axis
    let b_si = a_si * ((1. - e.powf(2.)).sqrt());

    //Prep Values
    let mu = ((a_si.powf(3.)) * 4. * (std::f64::consts::PI.powf(2.))) / (p_si.powf(2.));
    let p = (b_si.powf(2.)) / a_si;
    let v = 2. * (((1. + e) / (1. - e)).sqrt() * (ecc_anom * 0.5).tan()).atan();

    //Velocity of Companion in ellipse base
    let x_v = (0. - ((mu / p).sqrt())) * (v.sin());
    let y_v = ((mu / p).sqrt()) * (e + (v.cos()));

    //Ellipse base
    let x1 = (lotn_rad.cos() * aop_rad.cos()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.sin());
    let x2 = (lotn_rad.sin() * aop_rad.cos()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.sin());
    let x3 = i_rad.sin() * aop_rad.sin();

    let y1 =
        ((0. - lotn_rad.cos()) * aop_rad.sin()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.cos());
    let y2 =
        ((0. - lotn_rad.sin()) * aop_rad.sin()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.cos());
    let y3 = i_rad.sin() * aop_rad.cos();

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

pub fn companion_position(a: f64, e: f64, period: f64, t_p: f64) -> DVec2 {
    //SI units
    let p_si = period * 31557600.;
    let t_p_si = t_p * 31557600.;
    let a_si = a * 1000.;

    //Defining angles
    let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
    let mut ecc_anom = mean_anom;
    for _i in (0..=20).step_by(1) {
        ecc_anom = mean_anom + (e * ecc_anom.sin());
    }

    //Defining the semi minor-axis
    let b_si = a_si * ((1. - e.powf(2.)).sqrt());

    //Prep Values
    let p = (b_si.powf(2.)) / a_si;
    let v = 2. * (((1. + e) / (1. - e)).sqrt() * (ecc_anom * 0.5).tan()).atan();

    //Position of Companion in ellipse base
    let x = (p * v.cos()) / (1. + e * v.cos());
    let y = (p * v.sin()) / (1. + e * v.cos());

    DVec2::new(x, y)
}

pub fn companion_velocity(a: f64, e: f64, period: f64, t_p: f64) -> DVec2 {
    //SI units
    let p_si = period * 31557600.;
    let t_p_si = t_p * 31557600.;
    let a_si = a * 1000.;

    //Defining angles
    let mean_anom = std::f64::consts::PI * 2. * t_p_si / p_si;
    let mut ecc_anom = mean_anom;
    for _i in (0..=20).step_by(1) {
        ecc_anom = mean_anom + (e * ecc_anom.sin());
    }

    //Defining the semi minor-axis
    let b_si = a_si * ((1. - e.powf(2.)).sqrt());

    //Prep Values
    let mu = ((a_si.powf(3.)) * 4. * (std::f64::consts::PI.powf(2.))) / (p_si.powf(2.));
    let p = (b_si.powf(2.)) / a_si;
    let v = 2. * (((1. + e) / (1. - e)).sqrt() * (ecc_anom * 0.5).tan()).atan();

    //Velocity of Companion in ellipse base
    let x_v = (0. - ((mu / p).sqrt())) * (v.sin());
    let y_v = ((mu / p).sqrt()) * (e + (v.cos()));

    DVec2::new(x_v, y_v)
}

pub fn euler_angle_transformations(lotn: f64, aop: f64, i: f64) -> DMat3 {
    //In rad
    let lotn_rad = lotn.to_radians();
    let aop_rad = aop.to_radians();
    let i_rad = i.to_radians();

    //Ellipse base
    let x1 = (lotn_rad.cos() * aop_rad.cos()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.sin());
    let x2 = (lotn_rad.sin() * aop_rad.cos()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.sin());
    let x3 = i_rad.sin() * aop_rad.sin();

    let y1 =
        ((0. - lotn_rad.cos()) * aop_rad.sin()) - (lotn_rad.sin() * i_rad.cos() * aop_rad.cos());
    let y2 =
        ((0. - lotn_rad.sin()) * aop_rad.sin()) + (lotn_rad.cos() * i_rad.cos() * aop_rad.cos());
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
