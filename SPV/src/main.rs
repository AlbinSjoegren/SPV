fn main() {
    let input = Input {
        distance: 0., //In Lightyears
        declination_degree: 0., //In Degreees
        declination_min: 0., //In Minutes (')
        declination_s: 0., //In Seconds ('')
        right_ascension_h: 0., //In Hours
        right_ascension_min: 0., //In Minutes
        right_ascension_s: 0., //In Seconds
        radial_velocity: 0., //In km/s
        proper_motion_ra: 0., //In Arcseconds/year
        proper_motion_dec: 0., //In Arcseconds/year
    };
    
    //Gather inputs

    //Unit conversion 
    let declination = input.declination_degree + (input.declination_min * (1./60.)) + (input.declination_s * (1./3600.));
    let right_ascension = (input.right_ascension_h * 15.) + (input.right_ascension_min * (1./4.)) + (input.right_ascension_s * (1./240.));


    //Coordinates
    let x = input.distance
        * (right_ascension.to_radians()).cos()
        * ((declination + 90.).to_radians()).sin();
    let y = input.distance
        * (right_ascension.to_radians()).sin()
        * ((declination + 90.).to_radians()).sin();
    let z = input.distance * ((declination + 90.).to_radians()).cos();
    let cords = vec3::new(x, y, z);


    //Vector normalizing with the invers square root
    let mut normalized_vector_x = 0.;
    let mut normalized_vector_y = 0.;
    let mut normalized_vector_z = 0.;

    if input.radial_velocity < 0. {
        normalized_vector_x = (0_f64 - x)
            * (1. / ((0_f64 - x).powf(2.) + (0_f64 - y).powf(2.) + (0_f64 - z).powf(2.)).sqrt());
        normalized_vector_y = (0_f64 - y)
            * (1. / ((0_f64 - x).powf(2.) + (0_f64 - y).powf(2.) + (0_f64 - z).powf(2.)).sqrt());
        normalized_vector_z = (0_f64 - z)
            * (1. / ((0_f64 - x).powf(2.) + (0_f64 - y).powf(2.) + (0_f64 - z).powf(2.)).sqrt());
    }
    else if input.radial_velocity > 0. {
        normalized_vector_x = (x)
            * (1. / ((x).powf(2.) + (y).powf(2.) + (z).powf(2.)).sqrt());
        normalized_vector_y = (y)
            * (1. / ((x).powf(2.) + (y).powf(2.) + (z).powf(2.)).sqrt());
        normalized_vector_z = (z)
            * (1. / ((x).powf(2.) + (y).powf(2.) + (z).powf(2.)).sqrt());
    }
    

    let radial_velocity_vector_x = normalized_vector_x * input.radial_velocity;
    let radial_velocity_vector_y = normalized_vector_y * input.radial_velocity;
    let radial_velocity_vector_z = normalized_vector_z * input.radial_velocity;

    let proper_motion_x = input.distance
        * ((right_ascension
            + (((input.proper_motion_ra) / (3.154 * 10_f64.powf(7.))) / 3600.))
        .to_radians())
        .cos()
        * (((declination
            + (((input.proper_motion_dec) / (3.154 * 10_f64.powf(7.))) / 3600.))
            + 90.)
            .to_radians())
        .sin();
    let proper_motion_y = input.distance
        * ((right_ascension
            + (((input.proper_motion_ra) / (3.154 * 10_f64.powf(7.))) / 3600.))
        .to_radians())
        .sin()
        * (((declination
            + (((input.proper_motion_dec) / (3.154 * 10_f64.powf(7.))) / 3600.))
            + 90.)
            .to_radians())
        .sin();
    let proper_motion_z = input.distance
        * (((declination
            + (((input.proper_motion_dec) / (3.154 * 10_f64.powf(7.))) / 3600.))
            + 90.)
            .to_radians())
        .cos();

    let normalized_vector_proper_motion_x = (proper_motion_x - x)
        * (1.
            / ((proper_motion_x - x).powf(2.)
                + (proper_motion_y - y).powf(2.)
                + (proper_motion_z - z).powf(2.))
            .sqrt());
    let normalized_vector_proper_motion_y = (proper_motion_y - y)
        * (1.
            / ((proper_motion_x - x).powf(2.)
                + (proper_motion_y - y).powf(2.)
                + (proper_motion_z - z).powf(2.))
            .sqrt());
    let normalized_vector_proper_motion_z = (proper_motion_z - z)
        * (1.
            / ((proper_motion_x - x).powf(2.)
                + (proper_motion_y - y).powf(2.)
                + (proper_motion_z - z).powf(2.))
            .sqrt());

    let vector_proper_motion_x = normalized_vector_proper_motion_x * input.radial_velocity;
    let vector_proper_motion_y = normalized_vector_proper_motion_y * input.radial_velocity;
    let vector_proper_motion_z = normalized_vector_proper_motion_z * input.radial_velocity;

    let v = vec3::new(
        radial_velocity_vector_x + vector_proper_motion_x,
        radial_velocity_vector_y + vector_proper_motion_y,
        radial_velocity_vector_z + vector_proper_motion_z,
    );


    //Printing
    println!("{:?}, {:?}", cords, v);
}

struct Input {
    distance: f64, //In Lightyears
    declination_degree: f64, //In Degreees
    declination_min: f64, //In Minutes (')
    declination_s: f64, //In Seconds ('')
    right_ascension_h: f64, //In Hours
    right_ascension_min: f64, //In Minutes
    right_ascension_s: f64, //In Seconds
    radial_velocity: f64, //In km/s
    proper_motion_ra: f64, //In Arcseconds/year
    proper_motion_dec: f64, //In Arcseconds/year
}

