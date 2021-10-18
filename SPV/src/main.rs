use eframe::{egui, epi};

fn main() {
    let app = Canvas::default();
    let options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
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
    x: f64,
    y: f64,
    z: f64,
    radial_velocity: f64,
    distance: f64,
    right_ascension: f64,
    declination: f64,
    proper_motion_ra: f64,
    proper_motion_dec: f64,
) -> (f64, f64, f64) {
    //Vector normalizing with the invers square root
    let mut normalized_vector_x = 0.;
    let mut normalized_vector_y = 0.;
    let mut normalized_vector_z = 0.;

    if radial_velocity < 0. {
        normalized_vector_x = (0_f64 - x)
            * (1. / ((0_f64 - x).powf(2.) + (0_f64 - y).powf(2.) + (0_f64 - z).powf(2.)).sqrt());
        normalized_vector_y = (0_f64 - y)
            * (1. / ((0_f64 - x).powf(2.) + (0_f64 - y).powf(2.) + (0_f64 - z).powf(2.)).sqrt());
        normalized_vector_z = (0_f64 - z)
            * (1. / ((0_f64 - x).powf(2.) + (0_f64 - y).powf(2.) + (0_f64 - z).powf(2.)).sqrt());
    } else if radial_velocity > 0. {
        normalized_vector_x = (x) * (1. / ((x).powf(2.) + (y).powf(2.) + (z).powf(2.)).sqrt());
        normalized_vector_y = (y) * (1. / ((x).powf(2.) + (y).powf(2.) + (z).powf(2.)).sqrt());
        normalized_vector_z = (z) * (1. / ((x).powf(2.) + (y).powf(2.) + (z).powf(2.)).sqrt());
    }

    let radial_velocity_vector_x = normalized_vector_x * radial_velocity;
    let radial_velocity_vector_y = normalized_vector_y * radial_velocity;
    let radial_velocity_vector_z = normalized_vector_z * radial_velocity;

    let proper_motion_x = distance
        * ((right_ascension + (((proper_motion_ra) / (3.154 * 10_f64.powf(7.))) / 3600.))
            .to_radians())
        .cos()
        * (((declination + (((proper_motion_dec) / (3.154 * 10_f64.powf(7.))) / 3600.)) + 90.)
            .to_radians())
        .sin();
    let proper_motion_y = distance
        * ((right_ascension + (((proper_motion_ra) / (3.154 * 10_f64.powf(7.))) / 3600.))
            .to_radians())
        .sin()
        * (((declination + (((proper_motion_dec) / (3.154 * 10_f64.powf(7.))) / 3600.)) + 90.)
            .to_radians())
        .sin();
    let proper_motion_z = distance
        * (((declination + (((proper_motion_dec) / (3.154 * 10_f64.powf(7.))) / 3600.)) + 90.)
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

    let vector_proper_motion_x = normalized_vector_proper_motion_x * radial_velocity;
    let vector_proper_motion_y = normalized_vector_proper_motion_y * radial_velocity;
    let vector_proper_motion_z = normalized_vector_proper_motion_z * radial_velocity;

    let x_v = radial_velocity_vector_x + vector_proper_motion_x;
    let y_v = radial_velocity_vector_y + vector_proper_motion_y;
    let z_v = radial_velocity_vector_z + vector_proper_motion_z;

    return (x_v, y_v, z_v);
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Export {
    x_pos: f64,
    y_pos: f64,
    z_pos: f64,
    x_vel: f64,
    y_vel: f64,
    z_vel: f64,
    name: String,
}

use serde_json;
use std::fs::File;
use std::io::BufWriter;
fn export_json(x: f64, y: f64, z: f64, x_v: f64, y_v: f64, z_v: f64, name_str: String) {
    let data = Export {
        x_pos: x,
        y_pos: y,
        z_pos: z,
        x_vel: x_v,
        y_vel: y_v,
        z_vel: z_v,
        name: name_str,
    };
    // write out the file
    let writer = BufWriter::new(File::create("data.json").unwrap());
    serde_json::to_writer_pretty(writer, &data).unwrap();
}

fn export_txt(x: f64, y: f64, z: f64, x_v: f64, y_v: f64, z_v: f64, name_str: String) {}

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
    declination: f64,        //degrees
    declination_degree: f64, //In Degreees
    declination_degree_str: String,
    declination_min: f64, //In Minutes (')
    declination_min_str: String,
    declination_s: f64, //In Seconds ('')
    declination_s_str: String,
    right_ascension: f64,   //degrees
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
}

impl epi::App for Canvas {
    fn name(&self) -> &str {
        "SPV"
    }

    #[allow(unused_variables)]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("System name")).heading());
                            let response = ui.add(egui::TextEdit::singleline(&mut self.name_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {}
                            ui.add(egui::Label::new(format!("{}", self.name_str)).italics());
                        });

                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("Export file")).heading());
                            ui.horizontal_wrapped(|ui| {
                                if ui.add(egui::Button::new("JSON")).clicked() {
                                    export_json(
                                        self.x,
                                        self.y,
                                        self.z,
                                        self.x_v,
                                        self.y_v,
                                        self.z_v,
                                        self.name_str.clone(),
                                    );
                                }

                                if ui.add(egui::Button::new("TXT")).clicked() {
                                    //do_stuff();
                                }
                            });
                        });
                    });
                });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("Distance (lightyears)")).heading());
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.distance_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.distance = self.distance_str.clone().parse().unwrap();
                            }
                            ui.add(egui::Label::new(format!("{} ly", self.distance)).italics());
                            self.distance_km = self.distance * 9.461 * 10_f64.powf(12.);
                            ui.add(egui::Label::new(format!("{} km", self.distance_km)).italics());
                        });
                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("Declination")).heading());
                            ui.add(egui::Label::new(format!("Degrees (°)")).monospace());
                            let response = ui
                                .add(egui::TextEdit::singleline(&mut self.declination_degree_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.declination_degree =
                                    self.declination_degree_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}°", self.declination_degree)).italics(),
                            );

                            ui.add(egui::Label::new(format!("Minutes (')")).monospace());
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.declination_min_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.declination_min =
                                    self.declination_min_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}'", self.declination_min)).italics(),
                            );

                            ui.add(egui::Label::new(format!("Seconds ('')")).monospace());
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.declination_s_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.declination_s =
                                    self.declination_s_str.clone().parse().unwrap();
                            }
                            ui.add(egui::Label::new(format!("{}''", self.declination_s)).italics());

                            self.declination = self.declination_degree
                                + (self.declination_min / 60.)
                                + (self.declination_s / 3600.);

                            ui.add(egui::Label::new(format!("Total")).heading());
                            ui.add(egui::Label::new(format!("{}°", self.declination)).monospace());
                        });
                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("Right ascension")).heading());
                            ui.add(egui::Label::new(format!("Hours (h)")).monospace());
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.right_ascension_h_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.right_ascension_h =
                                    self.right_ascension_h_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}h", self.right_ascension_h)).italics(),
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
                                egui::Label::new(format!("{}m", self.right_ascension_min))
                                    .italics(),
                            );

                            ui.add(egui::Label::new(format!("Seconds (s)")).monospace());
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.right_ascension_s_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.right_ascension_s =
                                    self.right_ascension_s_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}s", self.right_ascension_s)).italics(),
                            );

                            self.right_ascension = (self.right_ascension_h * 15.)
                                + (self.right_ascension_min * (1. / 4.))
                                + (self.right_ascension_s * (1. / 240.));

                            ui.add(egui::Label::new(format!("Total")).heading());
                            ui.add(
                                egui::Label::new(format!("{}°", self.right_ascension)).monospace(),
                            );
                        });

                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("Resulting position (km)")).heading());

                            ui.add(
                                egui::Label::new(format!(
                                    "{:?}",
                                    position(
                                        self.distance_km,
                                        self.right_ascension,
                                        self.declination
                                    )
                                ))
                                .monospace(),
                            );
                            self.x =
                                position(self.distance_km, self.right_ascension, self.declination)
                                    .0;
                            self.y =
                                position(self.distance_km, self.right_ascension, self.declination)
                                    .1;
                            self.z =
                                position(self.distance_km, self.right_ascension, self.declination)
                                    .2;
                        });
                    });
                });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.group(|ui| {
                            ui.add(egui::Label::new(format!("Radial velocity (km/s)")).heading());
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.radial_velocity_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.radial_velocity =
                                    self.radial_velocity_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{} km/s", self.radial_velocity))
                                    .italics(),
                            );
                        });
                        ui.group(|ui| {
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
                                    .italics(),
                            );
                            ui.add(
                                egui::Label::new(format!("Declination (arcsecons/year)"))
                                    .monospace(),
                            );
                            let response =
                                ui.add(egui::TextEdit::singleline(&mut self.proper_motion_dec_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.proper_motion_dec =
                                    self.proper_motion_dec_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{} as/yr", self.proper_motion_dec))
                                    .italics(),
                            );
                        });

                        ui.group(|ui| {
                            ui.add(
                                egui::Label::new(format!("Resulting velocity (km/s)")).heading(),
                            );

                            ui.add(
                                egui::Label::new(format!(
                                    "{:?}",
                                    velocity(
                                        self.x,
                                        self.y,
                                        self.z,
                                        self.radial_velocity,
                                        self.distance,
                                        self.right_ascension,
                                        self.declination,
                                        self.proper_motion_ra,
                                        self.proper_motion_dec
                                    )
                                ))
                                .monospace(),
                            );
                            self.x_v = velocity(
                                self.x,
                                self.y,
                                self.z,
                                self.radial_velocity,
                                self.distance,
                                self.right_ascension,
                                self.declination,
                                self.proper_motion_ra,
                                self.proper_motion_dec,
                            )
                            .0;
                            self.y_v = velocity(
                                self.x,
                                self.y,
                                self.z,
                                self.radial_velocity,
                                self.distance,
                                self.right_ascension,
                                self.declination,
                                self.proper_motion_ra,
                                self.proper_motion_dec,
                            )
                            .1;
                            self.z_v = velocity(
                                self.x,
                                self.y,
                                self.z,
                                self.radial_velocity,
                                self.distance,
                                self.right_ascension,
                                self.declination,
                                self.proper_motion_ra,
                                self.proper_motion_dec,
                            )
                            .2;
                        });
                    });
                });
            });
        });
    }
}
