use eframe::{egui, epi};

fn main() {
    let app = Canvas::default();
    let options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);


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

#[derive(Default)]
pub struct Canvas {
    distance: f64, //In Lightyears
    distance_str: String,
    declination_degree: f64, //In Degreees
    declination_degree_str: String,
    declination_min: f64, //In Minutes (')
    declination_min_str: String,
    declination_s: f64, //In Seconds ('')
    declination_s_str: String,
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

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.group(|ui| {
                            ui.add(
                                egui::Label::new(format!("Distance (lightyears)"))
                                    .heading(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.distance_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.distance = self.distance_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{} ly", self.distance)).italics(),
                            );
                        });
                        
                        ui.group(|ui| {
                            ui.add(
                                egui::Label::new(format!("Declination"))
                                    .heading(),
                            );
                            ui.add(
                                egui::Label::new(format!("Degrees (°)"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.declination_degree_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.declination_degree = self.declination_degree_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}°", self.declination_degree)).italics(),
                            );
            
            
            
                            ui.add(
                                egui::Label::new(format!("Minutes (')"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.declination_min_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.declination_min = self.declination_min_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}'", self.declination_min)).italics(),
                            );
            
            
            
                            ui.add(
                                egui::Label::new(format!("Seconds ('')"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.declination_s_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.declination_s = self.declination_s_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}''", self.declination_s)).italics(),
                            );
                        });
            
                        ui.group(|ui| {
                            ui.add(
                                egui::Label::new(format!("Right ascension"))
                                    .heading(),
                            );
                            ui.add(
                                egui::Label::new(format!("Hours (h)"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.right_ascension_h_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.right_ascension_h = self.right_ascension_h_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}h", self.right_ascension_h)).italics(),
                            );
            
            
            
                            ui.add(
                                egui::Label::new(format!("Minutes (m)"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.right_ascension_min_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.right_ascension_min = self.right_ascension_min_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}m", self.right_ascension_min)).italics(),
                            );
            
            
            
                            ui.add(
                                egui::Label::new(format!("Seconds (s)"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.right_ascension_s_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.right_ascension_s = self.right_ascension_s_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{}s", self.right_ascension_s)).italics(),
                            );
                        });
                    });
                });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.group(|ui| {
                            ui.add(
                                egui::Label::new(format!("Radial velocity (km/s)"))
                                    .heading(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.radial_velocity_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.radial_velocity = self.radial_velocity_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{} km/s", self.radial_velocity)).italics(),
                            );
                        });
        
                        ui.group(|ui| {
                            ui.add(
                                egui::Label::new(format!("Proper motion"))
                                    .heading(),
                            );
                            ui.add(
                                egui::Label::new(format!("Right ascension (arcsecons/year)"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.proper_motion_ra_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.proper_motion_ra = self.proper_motion_ra_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{} as/yr", self.proper_motion_ra)).italics(),
                            );
            
            
            
                            ui.add(
                                egui::Label::new(format!("Declination (arcsecons/year)"))
                                    .monospace(),
                            );
                            let response = ui.add(egui::TextEdit::singleline(&mut self.proper_motion_dec_str));
                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                self.proper_motion_dec = self.proper_motion_dec_str.clone().parse().unwrap();
                            }
                            ui.add(
                                egui::Label::new(format!("{} as/yr", self.proper_motion_dec)).italics(),
                            );
                        });
                    });
                });
            });
        });
    }
}

