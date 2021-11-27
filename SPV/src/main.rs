#![windows_subsystem = "windows"]
#![allow(unused_assignments)]

use eframe::{egui, epi};

fn main() {
    let app = Canvas::default();

    let options = eframe::NativeOptions {
        transparent: true,

        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}

fn pos_vel_relative(
    a: f64,
    e: f64,
    period: f64,
    pos_a_x: f64,
    pos_a_y: f64,
    pos_a_z: f64,
    pos_b_x: f64,
    pos_b_y: f64,
    pos_b_z: f64,
    new_base_x_x: f64,
    new_base_x_y: f64,
    new_base_x_z: f64,
    new_base_y_x: f64,
    new_base_y_y: f64,
    new_base_y_z: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let mut angle_vec = vec![];

    let mut vec_x_b = vec![];
    let mut vec_y_b = vec![];
    let mut vec_z_b = vec![];

    let mut distance_vec = vec![];

    for n in (0_i32..=3600_i32).step_by(1) {
        //SI units (meters and seconds)
        let a_si = a * 1000.;
        let pos_a_x_si = pos_a_x * 1000.;
        let pos_a_y_si = pos_a_y * 1000.;
        let pos_a_z_si = pos_a_z * 1000.;
        let pos_b_x_si = pos_b_x * 1000.;
        let pos_b_y_si = pos_b_y * 1000.;
        let pos_b_z_si = pos_b_z * 1000.;

        //Push the angle to a vector
        angle_vec.push((f64::from(n)) / 10.);

        //Defining the semi minor axis
        let b_si = a_si * ((1. - e.powf(2.)).sqrt());

        //Position of B in new base
        let x = a_si * (((f64::from(n)) / 10.).to_radians().cos());
        let y = b_si * (((f64::from(n)) / 10.).to_radians().sin());

        //Get non relative position of B in original base
        let rel_x = pos_b_x_si - pos_a_x_si;
        let rel_y = pos_b_y_si - pos_a_y_si;
        let rel_z = pos_b_z_si - pos_a_z_si;

        //Get relative position in original base
        let res_x_old = (new_base_x_x * x) + (new_base_y_x * y);
        let res_y_old = (new_base_x_y * x) + (new_base_y_y * y);
        let res_z_old = (new_base_x_z * x) + (new_base_y_z * y);

        //Get difference in x, y and z
        let res_x = res_x_old - rel_x;
        let res_y = res_y_old - rel_y;
        let res_z = res_z_old - rel_z;

        //Get distance from relative positions to actual position in original base
        let distance = (res_x.powf(2.) + res_y.powf(2.) + res_z.powf(2.)).sqrt();

        //Pushing to vector
        vec_x_b.push(res_x_old);
        vec_y_b.push(res_y_old);
        vec_z_b.push(res_z_old);

        //Pushing to vector
        distance_vec.push(distance);
    }

    let min = distance_vec
        .iter()
        .min_by(|&&v1, &&v2| v1.abs().partial_cmp(&v2.abs()).unwrap());

    let closest_distance = *min.unwrap();

    let distance_pos = distance_vec
        .iter()
        .position(|&x| x == closest_distance)
        .unwrap();

    let distance = *distance_vec.iter().nth(distance_pos).unwrap();

    let angle = *angle_vec.iter().nth(distance_pos).unwrap();

    let b_pos_x = *vec_x_b.iter().nth(distance_pos).unwrap();
    let b_pos_y = *vec_y_b.iter().nth(distance_pos).unwrap();
    let b_pos_z = *vec_z_b.iter().nth(distance_pos).unwrap();

    //SI units (meters and seconds)
    let period_si = period * 31557600.;
    let a_si = a * 1000.;

    //Defining the semi minor axis
    let b_si = a_si * ((1. - e.powf(2.)).sqrt());

    //Velocity of B
    //Prep Values
    let mu = ((a_si.powf(3.)) * 4. * (std::f64::consts::PI.powf(2.))) / (period_si.powf(2.));
    let p = (b_si.powf(2.)) / a_si;

    //Velocity in new base
    let x_v = (0. - ((mu / p).sqrt())) * (angle.to_radians().sin());
    let y_v = ((mu / p).sqrt()) * (e + (angle.to_radians().cos()));

    //Velocity in original base
    let b_vel_x = (new_base_x_x * x_v) + (new_base_y_x * y_v);
    let b_vel_y = (new_base_x_y * x_v) + (new_base_y_y * y_v);
    let b_vel_z = (new_base_x_z * x_v) + (new_base_y_z * y_v);

    return (
        distance, angle, b_pos_x, b_pos_y, b_pos_z, b_vel_x, b_vel_y, b_vel_z,
    );
}

fn euler_angle_transformations(
    lotn: f64,
    aop: f64,
    i: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let x1 = (lotn.to_radians().cos() * aop.to_radians().cos())
        - (lotn.to_radians().sin() * i.to_radians().cos() * aop.to_radians().sin());
    let x2 = (lotn.to_radians().sin() * aop.to_radians().cos())
        + (lotn.to_radians().cos() * i.to_radians().cos() * aop.to_radians().sin());
    let x3 = i.to_radians().sin() * aop.to_radians().sin();

    let y1 = ((0. - lotn.to_radians().cos()) * aop.to_radians().sin())
        - (lotn.to_radians().sin() * i.to_radians().cos() * aop.to_radians().cos());
    let y2 = ((0. - lotn.to_radians().sin()) * aop.to_radians().sin())
        + (lotn.to_radians().cos() * i.to_radians().cos() * aop.to_radians().cos());
    let y3 = i.to_radians().sin() * aop.to_radians().cos();

    let z1 = i.to_radians().sin() * lotn.to_radians().sin();
    let z2 = (0. - i.to_radians().sin()) * lotn.to_radians().cos();
    let z3 = i.to_radians().cos();

    return (x1, x2, x3, y1, y2, y3, z1, z2, z3);
}

fn position(distance: f64, right_ascension: f64, declination: f64) -> (f64, f64, f64) {
    let x =
        distance * (right_ascension.to_radians()).cos() * ((declination + 90.).to_radians()).sin();

    let y =
        distance * (right_ascension.to_radians()).sin() * ((declination + 90.).to_radians()).sin();

    let z = distance * ((declination + 90.).to_radians()).cos();

    return (x, y, z);
}
fn velocity(
    distance: f64,
    right_ascension: f64,
    declination: f64,
    proper_motion_ra: f64,
    proper_motion_dec: f64,
    x: f64,
    y: f64,
    z: f64,
    radial_velocity: f64,
) -> (f64, f64, f64) {
    let proper_motion_x = distance
        * (((right_ascension + ((proper_motion_ra * 0.00027777777777778) / 31556926.))
            .to_radians())
        .cos())
        * ((((declination + ((proper_motion_dec * 0.00027777777777778) / 31556926.)) + 90.)
            .to_radians())
        .sin());

    let proper_motion_y = distance
        * (((right_ascension + ((proper_motion_ra * 0.00027777777777778) / 31556926.))
            .to_radians())
        .sin())
        * ((((declination + ((proper_motion_dec * 0.00027777777777778) / 31556926.)) + 90.)
            .to_radians())
        .sin());

    let proper_motion_z = distance
        * ((((declination + ((proper_motion_dec * 0.00027777777777778) / 31556926.)) + 90.)
            .to_radians())
        .cos());

    let proper_motion_vector_x = proper_motion_x - x;
    let proper_motion_vector_y = proper_motion_y - y;
    let proper_motion_vector_z = proper_motion_z - z;

    let mut normalized_vector_x = 0.;
    let mut normalized_vector_y = 0.;
    let mut normalized_vector_z = 0.;
    if radial_velocity < 0. {
        normalized_vector_x = 0. - (x / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt());
        normalized_vector_y = 0. - (y / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt());
        normalized_vector_z = 0. - (z / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt());
    } else if radial_velocity > 0. {
        normalized_vector_x = x / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt();
        normalized_vector_y = y / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt();
        normalized_vector_z = z / (x.powf(2.) + y.powf(2.) + z.powf(2.)).sqrt();
    } else {
        normalized_vector_x = 0.;
        normalized_vector_y = 0.;
        normalized_vector_z = 0.;
    }
    let radial_velocity_vector_x = normalized_vector_x * radial_velocity;
    let radial_velocity_vector_y = normalized_vector_y * radial_velocity;
    let radial_velocity_vector_z = normalized_vector_z * radial_velocity;

    let x_v = radial_velocity_vector_x + proper_motion_vector_x;
    let y_v = radial_velocity_vector_y + proper_motion_vector_y;
    let z_v = radial_velocity_vector_z + proper_motion_vector_z;

    return (x_v, y_v, z_v);
}

use egui::{FontDefinitions, FontFamily};
use serde::{Deserialize, Serialize};

/*
#[derive(Serialize, Deserialize, Debug)]
struct Export {
    name: String,
    x_pos: f64,
    y_pos: f64,
    z_pos: f64,
    x_vel: f64,
    y_vel: f64,
    z_vel: f64,
    new_base_x_x: f64,
    new_base_x_y: f64,
    new_base_x_z: f64,
    new_base_y_x: f64,
    new_base_y_y: f64,
    new_base_y_z: f64,
    new_base_z_x: f64,
    new_base_z_y: f64,
    new_base_z_z: f64,
    mass: f64,
    distance: f64,
    angle: f64,
    b_pos_x: f64,
    b_pos_y: f64,
    b_pos_z: f64,
    b_vel_x: f64,
    b_vel_y: f64,
    b_vel_z: f64,
}
*/
#[derive(Serialize, Deserialize, Debug)]
struct ExportPos {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportVel {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportBase {
    x_x: f64,
    x_y: f64,
    x_z: f64,
    y_x: f64,
    y_y: f64,
    y_z: f64,
    z_x: f64,
    z_y: f64,
    z_z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportPass {
    mass: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportRelPos {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportRelVel {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportRelDebug {
    distance: f64,
    angle: f64,
}

use serde_json;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufWriter, Write};

fn export_json(
    name_str: String,
    x: f64,
    y: f64,
    z: f64,
    x_v: f64,
    y_v: f64,
    z_v: f64,
    new_base_x_x: f64,
    new_base_x_y: f64,
    new_base_x_z: f64,
    new_base_y_x: f64,
    new_base_y_y: f64,
    new_base_y_z: f64,
    new_base_z_x: f64,
    new_base_z_y: f64,
    new_base_z_z: f64,
    mass: f64,
    distance: f64,
    angle: f64,
    b_pos_x: f64,
    b_pos_y: f64,
    b_pos_z: f64,
    b_vel_x: f64,
    b_vel_y: f64,
    b_vel_z: f64,
) {
    /*
    let data = Export {
        name: name_str,
        x_pos: x,
        y_pos: y,
        z_pos: z,
        x_vel: x_v,
        y_vel: y_v,
        z_vel: z_v,
        new_base_x_x: new_base_x_x,
        new_base_x_y: new_base_x_y,
        new_base_x_z: new_base_x_z,
        new_base_y_x: new_base_y_x,
        new_base_y_y: new_base_y_y,
        new_base_y_z: new_base_y_z,
        new_base_z_x: new_base_z_x,
        new_base_z_y: new_base_z_y,
        new_base_z_z: new_base_z_z,
        mass: mass,
        distance: distance,
        angle: angle,
        b_pos_x: b_pos_x,
        b_pos_y: b_pos_y,
        b_pos_z: b_pos_z,
        b_vel_x: b_vel_x,
        b_vel_y: b_vel_y,
        b_vel_z: b_vel_z,
    };
    */
    let pos_data = ExportPos { x: x, y: y, z: z };
    let vel_data = ExportVel {
        x: x_v,
        y: y_v,
        z: z_v,
    };
    let base_data = ExportBase {
        x_x: new_base_x_x,
        x_y: new_base_x_y,
        x_z: new_base_x_z,
        y_x: new_base_y_x,
        y_y: new_base_y_y,
        y_z: new_base_y_z,
        z_x: new_base_z_x,
        z_y: new_base_z_y,
        z_z: new_base_z_z,
    };
    let pass_data = ExportPass { mass: mass };
    let rel_pos_data = ExportRelPos {
        x: b_pos_x,
        y: b_pos_y,
        z: b_pos_z,
    };
    let rel_vel_data = ExportRelVel {
        x: b_vel_x,
        y: b_vel_y,
        z: b_vel_z,
    };
    let rel_debug_data = ExportRelDebug {
        distance: distance,
        angle: angle,
    };

    let mut path = std::path::PathBuf::from("./");
    path.push(name_str.clone().replace(" ", "_") + "_json");
    if std::path::Path::new(path.as_path()).is_dir() == false {
        std::fs::create_dir("./".to_string() + &name_str.clone().replace(" ", "_") + "_json")
            .expect("dir fail");
    }

    let mut file_path_pos = std::path::PathBuf::from("./");

    file_path_pos.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_pos.push("position_kilometers");

    file_path_pos.set_extension("json");

    let mut file_path_vel = std::path::PathBuf::from("./");

    file_path_vel.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_vel.push("velocity_in_kilometers_per_second");

    file_path_vel.set_extension("json");

    let mut file_path_base = std::path::PathBuf::from("./");

    file_path_base.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_base.push("base_in_NAN");

    file_path_base.set_extension("json");

    let mut file_path_pass = std::path::PathBuf::from("./");

    file_path_pass.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_pass.push("passtrough_in_unknown");

    file_path_pass.set_extension("json");

    let mut file_path_rel_pos = std::path::PathBuf::from("./");

    file_path_rel_pos.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_rel_pos.push("relative_position_in_meters");

    file_path_rel_pos.set_extension("json");

    let mut file_path_rel_vel = std::path::PathBuf::from("./");

    file_path_rel_vel.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_rel_vel.push("relative_velocity_in_meters_per_second");

    file_path_rel_vel.set_extension("json");

    let mut file_path_rel_debug = std::path::PathBuf::from("./");

    file_path_rel_debug.push(name_str.clone().replace(" ", "_") + "_json");
    file_path_rel_debug.push("relative_debug_in_unknown");

    file_path_rel_debug.set_extension("json");

    let writer_pos = BufWriter::new(File::create(file_path_pos).expect("path invalid"));
    let writer_vel = BufWriter::new(File::create(file_path_vel).expect("path invalid"));
    let writer_base = BufWriter::new(File::create(file_path_base).expect("path invalid"));
    let writer_pass = BufWriter::new(File::create(file_path_pass).expect("path invalid"));
    let writer_rel_pos = BufWriter::new(File::create(file_path_rel_pos).expect("path invalid"));
    let writer_rel_vel = BufWriter::new(File::create(file_path_rel_vel).expect("path invalid"));
    let writer_rel_debug = BufWriter::new(File::create(file_path_rel_debug).expect("path invalid"));

    serde_json::to_writer_pretty(writer_pos, &pos_data).unwrap();
    serde_json::to_writer_pretty(writer_vel, &vel_data).unwrap();
    serde_json::to_writer_pretty(writer_base, &base_data).unwrap();
    serde_json::to_writer_pretty(writer_pass, &pass_data).unwrap();
    serde_json::to_writer_pretty(writer_rel_pos, &rel_pos_data).unwrap();
    serde_json::to_writer_pretty(writer_rel_vel, &rel_vel_data).unwrap();
    serde_json::to_writer_pretty(writer_rel_debug, &rel_debug_data).unwrap();
}

fn export_txt(
    name_str: String,
    x: f64,
    y: f64,
    z: f64,
    x_v: f64,
    y_v: f64,
    z_v: f64,
    new_base_x_x: f64,
    new_base_x_y: f64,
    new_base_x_z: f64,
    new_base_y_x: f64,
    new_base_y_y: f64,
    new_base_y_z: f64,
    new_base_z_x: f64,
    new_base_z_y: f64,
    new_base_z_z: f64,
    mass: f64,
    distance: f64,
    angle: f64,
    b_pos_x: f64,
    b_pos_y: f64,
    b_pos_z: f64,
    b_vel_x: f64,
    b_vel_y: f64,
    b_vel_z: f64,
) {
    /*
    let data = Export {
        name: name_str,
        x_pos: x,
        y_pos: y,
        z_pos: z,
        x_vel: x_v,
        y_vel: y_v,
        z_vel: z_v,
        new_base_x_x: new_base_x_x,
        new_base_x_y: new_base_x_y,
        new_base_x_z: new_base_x_z,
        new_base_y_x: new_base_y_x,
        new_base_y_y: new_base_y_y,
        new_base_y_z: new_base_y_z,
        new_base_z_x: new_base_z_x,
        new_base_z_y: new_base_z_y,
        new_base_z_z: new_base_z_z,
        mass: mass,
        distance: distance,
        angle: angle,
        b_pos_x: b_pos_x,
        b_pos_y: b_pos_y,
        b_pos_z: b_pos_z,
        b_vel_x: b_vel_x,
        b_vel_y: b_vel_y,
        b_vel_z: b_vel_z,
    };
    */
    let pos_data = ExportPos { x: x, y: y, z: z };
    let vel_data = ExportVel {
        x: x_v,
        y: y_v,
        z: z_v,
    };
    let base_data = ExportBase {
        x_x: new_base_x_x,
        x_y: new_base_x_y,
        x_z: new_base_x_z,
        y_x: new_base_y_x,
        y_y: new_base_y_y,
        y_z: new_base_y_z,
        z_x: new_base_z_x,
        z_y: new_base_z_y,
        z_z: new_base_z_z,
    };
    let pass_data = ExportPass { mass: mass };
    let rel_pos_data = ExportRelPos {
        x: b_pos_x,
        y: b_pos_y,
        z: b_pos_z,
    };
    let rel_vel_data = ExportRelVel {
        x: b_vel_x,
        y: b_vel_y,
        z: b_vel_z,
    };
    let rel_debug_data = ExportRelDebug {
        distance: distance,
        angle: angle,
    };

    let mut path = std::path::PathBuf::from("./");
    path.push(name_str.clone().replace(" ", "_") + "_txt");
    if std::path::Path::new(path.as_path()).is_dir() == false {
        std::fs::create_dir("./".to_string() + &name_str.clone().replace(" ", "_") + "_txt")
            .expect("dir fail");
    }

    let mut file_path_pos = std::path::PathBuf::from("./");

    file_path_pos.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_pos.push("position_kilometers");

    file_path_pos.set_extension("txt");

    let mut file_path_vel = std::path::PathBuf::from("./");

    file_path_vel.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_vel.push("velocity_in_kilometers_per_second");

    file_path_vel.set_extension("txt");

    let mut file_path_base = std::path::PathBuf::from("./");

    file_path_base.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_base.push("base_in_NAN");

    file_path_base.set_extension("txt");

    let mut file_path_pass = std::path::PathBuf::from("./");

    file_path_pass.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_pass.push("passtrough_in_unknown");

    file_path_pass.set_extension("txt");

    let mut file_path_rel_pos = std::path::PathBuf::from("./");

    file_path_rel_pos.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_rel_pos.push("relative_position_in_meters");

    file_path_rel_pos.set_extension("txt");

    let mut file_path_rel_vel = std::path::PathBuf::from("./");

    file_path_rel_vel.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_rel_vel.push("relative_velocity_in_meters_per_second");

    file_path_rel_vel.set_extension("txt");

    let mut file_path_rel_debug = std::path::PathBuf::from("./");

    file_path_rel_debug.push(name_str.clone().replace(" ", "_") + "_txt");
    file_path_rel_debug.push("relative_debug_in_unknown");

    file_path_rel_debug.set_extension("txt");

    let mut buffer_pos = File::create(file_path_pos).expect("path invalid");
    let mut buffer_vel = File::create(file_path_vel).expect("path invalid");
    let mut buffer_base = File::create(file_path_base).expect("path invalid");
    let mut buffer_pass = File::create(file_path_pass).expect("path invalid");
    let mut buffer_rel_pos = File::create(file_path_rel_pos).expect("path invalid");
    let mut buffer_rel_vel = File::create(file_path_rel_vel).expect("path invalid");
    let mut buffer_rel_debug = File::create(file_path_rel_debug).expect("path invalid");

    buffer_pos
        .write_all(serde_json::to_string(&pos_data).unwrap().as_bytes())
        .unwrap();
    buffer_vel
        .write_all(serde_json::to_string(&vel_data).unwrap().as_bytes())
        .unwrap();
    buffer_base
        .write_all(serde_json::to_string(&base_data).unwrap().as_bytes())
        .unwrap();
    buffer_pass
        .write_all(serde_json::to_string(&pass_data).unwrap().as_bytes())
        .unwrap();
    buffer_rel_pos
        .write_all(serde_json::to_string(&rel_pos_data).unwrap().as_bytes())
        .unwrap();
    buffer_rel_vel
        .write_all(serde_json::to_string(&rel_vel_data).unwrap().as_bytes())
        .unwrap();
    buffer_rel_debug
        .write_all(serde_json::to_string(&rel_debug_data).unwrap().as_bytes())
        .unwrap();
}

#[derive(Default)]

pub struct Canvas {
    name_str: String,

    x: f64,
    y: f64,
    z: f64,

    x_v: f64,
    y_v: f64,
    z_v: f64,

    distance: f64,    //In Lightyears
    distance_km: f64, //In km
    distance_str: String,

    declination: f64, //degrees

    declination_degree: f64, //In Degreees
    declination_degree_str: String,

    declination_min: f64, //In Minutes (')
    declination_min_str: String,

    declination_s: f64, //In Seconds ('')
    declination_s_str: String,

    right_ascension: f64, //degrees

    right_ascension_h: f64, //In Hours
    right_ascension_h_str: String,

    right_ascension_min: f64, //In Minutes
    right_ascension_min_str: String,

    right_ascension_s: f64, //In Seconds
    right_ascension_s_str: String,

    radial_velocity: f64, //In km/s
    radial_velocity_str: String,

    proper_motion_ra: f64, //In Arcseconds/year
    proper_motion_ra_str: String,

    proper_motion_dec: f64, //In Arcseconds/year
    proper_motion_dec_str: String,

    lotn: f64,
    aop: f64,
    i: f64,

    lotn_str: String,
    aop_str: String,
    i_str: String,

    new_base_x_x: f64,
    new_base_x_y: f64,
    new_base_x_z: f64,
    new_base_y_x: f64,
    new_base_y_y: f64,
    new_base_y_z: f64,
    new_base_z_x: f64,
    new_base_z_y: f64,
    new_base_z_z: f64,

    a: f64,
    e: f64,
    period: f64,

    a_str: String,
    e_str: String,
    period_str: String,

    pos_a_x: f64,
    pos_a_y: f64,
    pos_a_z: f64,
    pos_b_x: f64,
    pos_b_y: f64,
    pos_b_z: f64,

    pos_a_x_str: String,
    pos_a_y_str: String,
    pos_a_z_str: String,
    pos_b_x_str: String,
    pos_b_y_str: String,
    pos_b_z_str: String,

    pass_mass: f64,

    pass_mass_str: String,

    distance_btob: f64,
    angle: f64,
    b_pos_x: f64,
    b_pos_y: f64,
    b_pos_z: f64,
    b_vel_x: f64,
    b_vel_y: f64,
    b_vel_z: f64,

    general_toggle: bool,
    pos_vel_toggle: bool,
    rel_pos_vel_toggle: bool,
    export_toggle: bool,
    passtrough_toggle: bool,
    results_toggle: bool,
}

impl epi::App for Canvas {
    fn name(&self) -> &str {
        "SPV"
    }

    #[allow(unused_variables)]

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(30, 34, 39);

        style.visuals.faint_bg_color = egui::Color32::from_rgb(30, 34, 39);

        style.visuals.code_bg_color = egui::Color32::from_rgb(30, 34, 39);

        style.visuals.hyperlink_color = egui::Color32::from_rgb(255, 0, 0);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(160, 167, 179));

        style.visuals.window_corner_radius = 0.1;

        style.visuals.button_frame = true;

        style.visuals.collapsing_header_frame = true;

        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(35, 39, 46);

        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(0., egui::Color32::from_rgb(160, 167, 179));

        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;

        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(30, 34, 39);

        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(30, 34, 39);

        style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(30, 34, 39);

        ctx.set_style(style);

        let font_droidsansmono = include_bytes!("data/Droid Sans Mono Nerd Font Complete Mono.otf");
        let mut font = FontDefinitions::default();

        font.font_data.insert(
            "Droid Sans Mono".to_string(),
            Cow::from(&font_droidsansmono[..]),
        );
        font.fonts_for_family
            .insert(FontFamily::Monospace, vec!["Droid Sans Mono".to_string()]);

        font.fonts_for_family.insert(
            FontFamily::Proportional,
            vec!["Droid Sans Mono".to_string()],
        );
        /*
        font.family_and_size.insert(
            epaint::text::TextStyle::Body,
            (epaint::text::FontFamily::Proportional, 10.0),
        );
        font.family_and_size.insert(
            epaint::text::TextStyle::Body,
            (epaint::text::FontFamily::Monospace, 10.0),
        );
        */
        ctx.set_fonts(font);
    }

    #[cfg(feature = "persistence")]

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn warm_up_enabled(&self) -> bool {
        return true;
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::SidePanel::left("Tabs").show(ctx, |ui| {
            if ui.add(egui::Button::new(format!("General"))).clicked() {
                self.general_toggle = !self.general_toggle
            }

            if ui.add(egui::Button::new(format!("Pos & Vel"))).clicked() {
                self.pos_vel_toggle = !self.pos_vel_toggle
            }

            if ui
                .add(egui::Button::new(format!(
                    "Relative
Pos & Vel"
                )))
                .clicked()
            {
                self.rel_pos_vel_toggle = !self.rel_pos_vel_toggle
            }

            if ui.add(egui::Button::new(format!("Passtrough"))).clicked() {
                self.passtrough_toggle = !self.passtrough_toggle
            }

            if ui.add(egui::Button::new(format!("Results"))).clicked() {
                self.results_toggle = !self.results_toggle
            }

            if ui.add(egui::Button::new(format!("Export"))).clicked() {
                self.export_toggle = !self.export_toggle
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.add(egui::Button::new(format!("Organize"))).clicked() {
                    ui.ctx().memory().reset_areas();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let naming_window = egui::Window::new("Name")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.general_toggle == true {
                ui.vertical(|_ui| {
                    naming_window.show(ctx, |ui| {
                        ui.add(egui::Label::new(format!("System name")).heading());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.name_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {}

                        ui.add(egui::Label::new(format!("{}", self.name_str)).monospace());
                    });
                });
            }

            let distance_window = egui::Window::new("Distance")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.pos_vel_toggle == true {
                ui.vertical(|_ui| {
                    distance_window.show(ctx, |ui| {
                        ui.add(egui::Label::new(format!("Distance (lightyears)")).heading());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.distance_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.distance = self.distance_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} ly", self.distance)).monospace());

                        self.distance_km = self.distance * 9.461 * 10_f64.powf(12.);

                        ui.add(egui::Label::new(format!("{} km", self.distance_km)).monospace());
                    });
                });
            }

            let ra_window = egui::Window::new("Right ascension")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.pos_vel_toggle == true {
                ui.vertical(|_ui| {
                    ra_window.show(ctx, |ui| {
                        ui.add(egui::Label::new(format!("Right ascension")).heading());

                        ui.add(egui::Label::new(format!("Hours (h)")).monospace());

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.right_ascension_h_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.right_ascension_h =
                                self.right_ascension_h_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{}h", self.right_ascension_h)).monospace(),
                        );

                        ui.add(egui::Label::new(format!("Minutes (m)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(
                            &mut self.right_ascension_min_str,
                        ));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.right_ascension_min =
                                self.right_ascension_min_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{}m", self.right_ascension_min)).monospace(),
                        );

                        ui.add(egui::Label::new(format!("Seconds (s)")).monospace());

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.right_ascension_s_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.right_ascension_s =
                                self.right_ascension_s_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{}s", self.right_ascension_s)).monospace(),
                        );

                        self.right_ascension = (self.right_ascension_h * 15.)
                            + (self.right_ascension_min * (1. / 4.))
                            + (self.right_ascension_s * (1. / 240.));

                        ui.add(egui::Label::new(format!("Total")).heading());

                        ui.add(egui::Label::new(format!("{}°", self.right_ascension)).monospace());
                    });
                });
            }

            let dec_window = egui::Window::new("Declination")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.pos_vel_toggle == true {
                ui.vertical(|_ui| {
                    dec_window.show(ctx, |ui| {
                        ui.add(egui::Label::new(format!("Declination")).heading());

                        ui.add(egui::Label::new(format!("Degrees (°)")).monospace());

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_degree_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.declination_degree =
                                self.declination_degree_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{}°", self.declination_degree)).monospace(),
                        );

                        ui.add(egui::Label::new(format!("Minutes (')")).monospace());

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_min_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.declination_min =
                                self.declination_min_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{}'", self.declination_min)).monospace());

                        ui.add(egui::Label::new(format!("Seconds ('')")).monospace());

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_s_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.declination_s = self.declination_s_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{}''", self.declination_s)).monospace());

                        self.declination = self.declination_degree
                            + (self.declination_min / 60.)
                            + (self.declination_s / 3600.);

                        ui.add(egui::Label::new(format!("Total")).heading());

                        ui.add(egui::Label::new(format!("{}°", self.declination)).monospace());
                    });
                });
            }

            let rv_window = egui::Window::new("Radial velocity")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.pos_vel_toggle == true {
                ui.vertical(|_ui| {
                    rv_window.show(ctx, |ui| {
                        ui.add(egui::Label::new(format!("Radial velocity (km/s)")).heading());

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.radial_velocity_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.radial_velocity =
                                self.radial_velocity_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{} km/s", self.radial_velocity)).monospace(),
                        );
                    });
                });
            }

            let pm_window = egui::Window::new("Proper motion")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.pos_vel_toggle == true {
                pm_window.show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::Label::new(format!("Proper motion")).heading());

                        ui.add(
                            egui::Label::new(format!("Right ascension (arcsecons/year)"))
                                .monospace(),
                        );

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.proper_motion_ra_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.proper_motion_ra =
                                self.proper_motion_ra_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{} as/yr", self.proper_motion_ra))
                                .monospace(),
                        );

                        ui.add(
                            egui::Label::new(format!("Declination (arcsecons/year)")).monospace(),
                        );

                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.proper_motion_dec_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.proper_motion_dec =
                                self.proper_motion_dec_str.clone().parse().unwrap();
                        }

                        ui.add(
                            egui::Label::new(format!("{} as/yr", self.proper_motion_dec))
                                .monospace(),
                        );
                    });
                });
            }

            let rel_pos_vel_window = egui::Window::new("Relative position and velocity")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.rel_pos_vel_toggle == true {
                rel_pos_vel_window.show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::Label::new(format!("Orbital elements")).heading());

                        ui.add(egui::Label::new(format!("Semi-major axis (a in km)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.a_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.a = self.a_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} km", self.a)).monospace());

                        ui.label("");

                        ui.add(egui::Label::new(format!("Eccentricity (e)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.e_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.e = self.e_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{}", self.e)).monospace());

                        ui.label("");

                        ui.add(egui::Label::new(format!("Period (P in years)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.period_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.period = self.period_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} years", self.period)).monospace());

                        ui.label(
                            "
                        ",
                        );

                        ui.add(egui::Label::new(format!("Old positions")).heading());

                        ui.add(egui::Label::new(format!("A star position (km)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pos_a_x_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pos_a_x = self.pos_a_x_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("x = {} km", self.pos_a_x)).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pos_a_y_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pos_a_y = self.pos_a_y_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("y = {} km", self.pos_a_y)).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pos_a_z_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pos_a_z = self.pos_a_z_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("z = {} km", self.pos_a_z)).monospace());

                        ui.label(
                            "
                        ",
                        );

                        ui.add(egui::Label::new(format!("B star position (km)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pos_b_x_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pos_b_x = self.pos_b_x_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("x = {} km", self.pos_b_x)).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pos_b_y_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pos_b_y = self.pos_b_y_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("y = {} km", self.pos_b_y)).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pos_b_z_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pos_b_z = self.pos_b_z_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("z = {} km", self.pos_b_z)).monospace());
                    });
                });
            }

            let euler_angle_transformations_window =
                egui::Window::new("Euler angle transformations")
                    .auto_sized()
                    .collapsible(true)
                    .resizable(false);

            if self.rel_pos_vel_toggle == true {
                euler_angle_transformations_window.show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::Label::new(format!("Angle values")).heading());

                        ui.add(egui::Label::new(format!("Longitude of the node (Ω)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.lotn_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.lotn = self.lotn_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} degrees", self.lotn)).monospace());

                        ui.add(egui::Label::new(format!("Argument of periastron (ω)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.aop_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.aop = self.aop_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} degrees", self.aop)).monospace());

                        ui.add(egui::Label::new(format!("Inclination (i)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.i_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.i = self.i_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} degrees", self.i)).monospace());
                    });
                });
            }

            let passtrough_window = egui::Window::new("Passtrough")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.passtrough_toggle == true {
                ui.vertical(|_ui| {
                    passtrough_window.show(ctx, |ui| {
                        ui.add(egui::Label::new(format!("Mass (kg)")).monospace());

                        let response = ui.add(egui::TextEdit::singleline(&mut self.pass_mass_str));

                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.pass_mass = self.pass_mass_str.clone().parse().unwrap();
                        }

                        ui.add(egui::Label::new(format!("{} kg", self.pass_mass)).monospace());
                    });
                });
            }

            let results_window = egui::Window::new("Results")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.results_toggle == true {
                ui.vertical(|_ui| {
                    results_window.show(ctx, |ui| {
                        self.distance_btob = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .0;
                        self.angle = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .1;
                        self.b_pos_x = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .2;
                        self.b_pos_y = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .3;
                        self.b_pos_z = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .4;
                        self.b_vel_x = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .5;
                        self.b_vel_y = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .6;
                        self.b_vel_z = pos_vel_relative(
                            self.a.clone(),
                            self.e.clone(),
                            self.period.clone(),
                            self.pos_a_x.clone(),
                            self.pos_a_y.clone(),
                            self.pos_a_z.clone(),
                            self.pos_b_x.clone(),
                            self.pos_b_y.clone(),
                            self.pos_b_z.clone(),
                            self.new_base_x_x.clone(),
                            self.new_base_x_y.clone(),
                            self.new_base_x_z.clone(),
                            self.new_base_y_x.clone(),
                            self.new_base_y_y.clone(),
                            self.new_base_y_z.clone(),
                        )
                        .7;

                        self.x = position(
                            self.distance_km.clone(),
                            self.right_ascension.clone(),
                            self.declination.clone(),
                        )
                        .0;

                        self.y = position(
                            self.distance_km.clone(),
                            self.right_ascension.clone(),
                            self.declination.clone(),
                        )
                        .1;

                        self.z = position(
                            self.distance_km.clone(),
                            self.right_ascension.clone(),
                            self.declination.clone(),
                        )
                        .2;

                        self.x_v = velocity(
                            self.distance_km.clone(),
                            self.right_ascension.clone(),
                            self.declination.clone(),
                            self.proper_motion_ra.clone(),
                            self.proper_motion_dec.clone(),
                            self.x.clone(),
                            self.y.clone(),
                            self.z.clone(),
                            self.radial_velocity.clone(),
                        )
                        .0;

                        self.y_v = velocity(
                            self.distance_km.clone(),
                            self.right_ascension.clone(),
                            self.declination.clone(),
                            self.proper_motion_ra.clone(),
                            self.proper_motion_dec.clone(),
                            self.x.clone(),
                            self.y.clone(),
                            self.z.clone(),
                            self.radial_velocity.clone(),
                        )
                        .1;

                        self.z_v = velocity(
                            self.distance_km.clone(),
                            self.right_ascension.clone(),
                            self.declination.clone(),
                            self.proper_motion_ra.clone(),
                            self.proper_motion_dec.clone(),
                            self.x.clone(),
                            self.y.clone(),
                            self.z.clone(),
                            self.radial_velocity.clone(),
                        )
                        .2;

                        self.new_base_x_x =
                            euler_angle_transformations(self.lotn, self.aop, self.i).0;
                        self.new_base_x_y =
                            euler_angle_transformations(self.lotn, self.aop, self.i).1;
                        self.new_base_x_z =
                            euler_angle_transformations(self.lotn, self.aop, self.i).2;
                        self.new_base_y_x =
                            euler_angle_transformations(self.lotn, self.aop, self.i).3;
                        self.new_base_y_y =
                            euler_angle_transformations(self.lotn, self.aop, self.i).4;
                        self.new_base_y_z =
                            euler_angle_transformations(self.lotn, self.aop, self.i).5;
                        self.new_base_z_x =
                            euler_angle_transformations(self.lotn, self.aop, self.i).6;
                        self.new_base_z_y =
                            euler_angle_transformations(self.lotn, self.aop, self.i).7;
                        self.new_base_z_z =
                            euler_angle_transformations(self.lotn, self.aop, self.i).8;

                        ui.add(egui::Label::new(format!("Resulting position (km)")).heading());

                        ui.add(egui::Label::new(format!("x = {} km", self.x)).monospace());

                        ui.add(egui::Label::new(format!("y = {} km", self.y)).monospace());

                        ui.add(egui::Label::new(format!("z = {} km", self.z)).monospace());

                        ui.add(egui::Label::new(format!("Resulting velocity (km/s)")).heading());

                        ui.add(egui::Label::new(format!("x = {} km/s", self.x_v)).monospace());

                        ui.add(egui::Label::new(format!("y = {} km/s", self.y_v)).monospace());

                        ui.add(egui::Label::new(format!("z = {} km/s", self.z_v)).monospace());

                        ui.add(egui::Label::new(format!("New base")).heading());

                        ui.add(
                            egui::Label::new(format!(
                                "ẋ: x({}), y({}), z({})",
                                self.new_base_x_x, self.new_base_x_y, self.new_base_x_z
                            ))
                            .monospace(),
                        );
                        ui.add(
                            egui::Label::new(format!(
                                "ẏ: x({}), y({}), z({})",
                                self.new_base_y_x, self.new_base_y_y, self.new_base_y_z
                            ))
                            .monospace(),
                        );
                        ui.add(
                            egui::Label::new(format!(
                                "ż: x({}), y({}), z({})",
                                self.new_base_z_x, self.new_base_z_y, self.new_base_z_z
                            ))
                            .monospace(),
                        );

                        ui.add(
                            egui::Label::new(format!("Distance from new B to old B (m)")).heading(),
                        );

                        ui.add(
                            egui::Label::new(format!("distance = {} m", self.distance_btob))
                                .monospace(),
                        );

                        ui.add(egui::Label::new(format!("Angle (degrees)")).heading());

                        ui.add(egui::Label::new(format!("v = {} degrees", self.angle)).monospace());

                        ui.add(
                            egui::Label::new(format!("Relative resulting position (m)")).heading(),
                        );

                        ui.add(egui::Label::new(format!("Companion star")).monospace());

                        ui.add(egui::Label::new(format!("x = {} m", self.b_pos_x)).monospace());

                        ui.add(egui::Label::new(format!("y = {} m", self.b_pos_y)).monospace());

                        ui.add(egui::Label::new(format!("z = {} m", self.b_pos_z)).monospace());

                        ui.add(
                            egui::Label::new(format!("Relative resulting velocity (m/s)"))
                                .heading(),
                        );

                        ui.add(egui::Label::new(format!("Companion star")).monospace());

                        ui.add(egui::Label::new(format!("x = {} m/s", self.b_vel_x)).monospace());

                        ui.add(egui::Label::new(format!("y = {} m/s", self.b_vel_y)).monospace());

                        ui.add(egui::Label::new(format!("z = {} m/s", self.b_vel_z)).monospace());
                        ui.add(
                            egui::Label::new(format!(
                                "Position and Velocity are 0 for Primary star"
                            ))
                            .monospace(),
                        );
                    });
                });
            }

            let export_window = egui::Window::new("Export file")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.export_toggle == true {
                export_window.show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::Label::new(format!("Export file")).heading());

                        ui.horizontal_wrapped(|ui| {
                            if ui.add(egui::Button::new("JSON")).clicked() {
                                export_json(
                                    self.name_str.clone(),
                                    self.x,
                                    self.y,
                                    self.z,
                                    self.x_v,
                                    self.y_v,
                                    self.z_v,
                                    self.new_base_x_x,
                                    self.new_base_x_y,
                                    self.new_base_x_z,
                                    self.new_base_y_x,
                                    self.new_base_y_y,
                                    self.new_base_y_z,
                                    self.new_base_z_x,
                                    self.new_base_z_y,
                                    self.new_base_z_z,
                                    self.pass_mass,
                                    self.distance_btob,
                                    self.angle,
                                    self.b_pos_x,
                                    self.b_pos_y,
                                    self.b_pos_z,
                                    self.b_vel_x,
                                    self.b_vel_y,
                                    self.b_vel_z,
                                );
                            }

                            if ui.add(egui::Button::new("TXT")).clicked() {
                                export_txt(
                                    self.name_str.clone(),
                                    self.x,
                                    self.y,
                                    self.z,
                                    self.x_v,
                                    self.y_v,
                                    self.z_v,
                                    self.new_base_x_x,
                                    self.new_base_x_y,
                                    self.new_base_x_z,
                                    self.new_base_y_x,
                                    self.new_base_y_y,
                                    self.new_base_y_z,
                                    self.new_base_z_x,
                                    self.new_base_z_y,
                                    self.new_base_z_z,
                                    self.pass_mass,
                                    self.distance_btob,
                                    self.angle,
                                    self.b_pos_x,
                                    self.b_pos_y,
                                    self.b_pos_z,
                                    self.b_vel_x,
                                    self.b_vel_y,
                                    self.b_vel_z,
                                );
                            }
                        });
                    });
                });
            }
        });
    }
}
