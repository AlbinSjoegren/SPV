#![windows_subsystem = "windows"]
#![allow(unused_assignments)]

use eframe::{egui, epi};

fn main() {
    let app = Canvas::default();

    let image_bytes = include_bytes!("../icon.png");
    let image_image = image::load_from_memory(image_bytes).unwrap();
    let image_rgba = image_image.as_rgba8().unwrap().clone().into_raw();

    use image::GenericImageView;
    let dimensions = image_image.dimensions();

    let icon = epi::IconData {
        rgba: image_rgba,
        width: dimensions.0,
        height: dimensions.1,
    };

    let options = eframe::NativeOptions {
        transparent: true,
        icon_data: Some(icon),

        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}

use std::error::Error;
fn parse_text_input(
    response: egui::Response,
    string: String,
    mut val: f64,
) -> Result<f64, Box<dyn Error>> {
    if response.changed() && !string.is_empty() && string != "-" {
        val = string.parse()?;
    } else if string.is_empty() || string == "-" {
        val = 0.;
    }
    Ok(val)
}

use egui::{FontDefinitions, FontFamily};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExportPosistion {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportVelocity {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportEulerAngleTransformations {
    x1: f64,
    x2: f64,
    x3: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    z1: f64,
    z2: f64,
    z3: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportCompanionPosition {
    x: f64,
    y: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportCompanionVelocity {
    x: f64,
    y: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportCompanionRelativePosition {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ExportCompanionRelativeVelocity {
    x: f64,
    y: f64,
    z: f64,
}

use std::borrow::Cow;
use std::fs::File;
use std::io::BufWriter;

fn export_json(
    name_str: String,
    x: f64,
    y: f64,
    z: f64,
    x_v: f64,
    y_v: f64,
    z_v: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    z1: f64,
    z2: f64,
    z3: f64,
    companion_position_x: f64,
    companion_position_y: f64,
    companion_velocity_x: f64,
    companion_velocity_y: f64,
    companion_relative_position_x: f64,
    companion_relative_position_y: f64,
    companion_relative_position_z: f64,
    companion_relative_velocity_x: f64,
    companion_relative_velocity_y: f64,
    companion_relative_velocity_z: f64,
) {
    let position_data = ExportPosistion { x, y, z };
    let velocity_data = ExportVelocity {
        x: x_v,
        y: y_v,
        z: z_v,
    };
    let euler_angle_transformations_data = ExportEulerAngleTransformations {
        x1,
        x2,
        x3,
        y1,
        y2,
        y3,
        z1,
        z2,
        z3,
    };
    let companion_position_data = ExportCompanionPosition {
        x: companion_position_x,
        y: companion_position_y,
    };
    let companion_velocity_data = ExportCompanionVelocity {
        x: companion_velocity_x,
        y: companion_velocity_y,
    };
    let companion_relative_position_data = ExportCompanionRelativePosition {
        x: companion_relative_position_x,
        y: companion_relative_position_y,
        z: companion_relative_position_z,
    };
    let companion_relative_velocity_data = ExportCompanionRelativeVelocity {
        x: companion_relative_velocity_x,
        y: companion_relative_velocity_y,
        z: companion_relative_velocity_z,
    };

    let mut path = std::path::PathBuf::from("./");
    path.push(name_str.replace(" ", "_") + "_json");
    if !std::path::Path::new(path.as_path()).is_dir() {
        std::fs::create_dir("./".to_string() + &name_str.replace(" ", "_") + "_json")
            .expect("dir fail");
    }

    let mut file_path_position = std::path::PathBuf::from("./");
    file_path_position.push(name_str.replace(" ", "_") + "_json");
    file_path_position.push("position_meters");
    file_path_position.set_extension("json");

    let mut file_path_velocity = std::path::PathBuf::from("./");
    file_path_velocity.push(name_str.replace(" ", "_") + "_json");
    file_path_velocity.push("velocity_meters_per_second");
    file_path_velocity.set_extension("json");

    let mut file_path_euler_angle_transformations = std::path::PathBuf::from("./");
    file_path_euler_angle_transformations.push(name_str.replace(" ", "_") + "_json");
    file_path_euler_angle_transformations.push("euler_angle_transformations");
    file_path_euler_angle_transformations.set_extension("json");

    let mut file_path_companion_position = std::path::PathBuf::from("./");
    file_path_companion_position.push(name_str.replace(" ", "_") + "_json");
    file_path_companion_position.push("companion_position_meters");
    file_path_companion_position.set_extension("json");

    let mut file_path_companion_velocity = std::path::PathBuf::from("./");
    file_path_companion_velocity.push(name_str.replace(" ", "_") + "_json");
    file_path_companion_velocity.push("companion_velocity_meters_per_second");
    file_path_companion_velocity.set_extension("json");

    let mut file_path_companion_relative_position = std::path::PathBuf::from("./");
    file_path_companion_relative_position.push(name_str.replace(" ", "_") + "_json");
    file_path_companion_relative_position.push("companion_relative_position_meters");
    file_path_companion_relative_position.set_extension("json");

    let mut file_path_companion_relative_velocity = std::path::PathBuf::from("./");
    file_path_companion_relative_velocity.push(name_str.replace(" ", "_") + "_json");
    file_path_companion_relative_velocity.push("companion_relative_velocity_meters_per_second");
    file_path_companion_relative_velocity.set_extension("json");

    let writer_position = BufWriter::new(File::create(file_path_position).expect("path invalid"));
    let writer_velocity = BufWriter::new(File::create(file_path_velocity).expect("path invalid"));
    let writer_euler_angle_transformations =
        BufWriter::new(File::create(file_path_euler_angle_transformations).expect("path invalid"));
    let writer_companion_position =
        BufWriter::new(File::create(file_path_companion_position).expect("path invalid"));
    let writer_companion_velocity =
        BufWriter::new(File::create(file_path_companion_velocity).expect("path invalid"));
    let writer_companion_relative_position =
        BufWriter::new(File::create(file_path_companion_relative_position).expect("path invalid"));
    let writer_companion_relative_velocity =
        BufWriter::new(File::create(file_path_companion_relative_velocity).expect("path invalid"));

    serde_json::to_writer_pretty(writer_position, &position_data).unwrap();
    serde_json::to_writer_pretty(writer_velocity, &velocity_data).unwrap();
    serde_json::to_writer_pretty(
        writer_euler_angle_transformations,
        &euler_angle_transformations_data,
    )
    .unwrap();
    serde_json::to_writer_pretty(writer_companion_position, &companion_position_data).unwrap();
    serde_json::to_writer_pretty(writer_companion_velocity, &companion_velocity_data).unwrap();
    serde_json::to_writer_pretty(
        writer_companion_relative_position,
        &companion_relative_position_data,
    )
    .unwrap();
    serde_json::to_writer_pretty(
        writer_companion_relative_velocity,
        &companion_relative_velocity_data,
    )
    .unwrap();
}

#[derive(Default)]

struct Canvas {
    name_str: String,

    x: f64,
    y: f64,
    z: f64,

    x_v: f64,
    y_v: f64,
    z_v: f64,

    parallax: f64,
    parallax_str: String,
    distance_parsec: f64,

    declination: f64,

    declination_degree: f64,
    declination_degree_str: String,

    declination_min: f64,
    declination_min_str: String,

    declination_s: f64,
    declination_s_str: String,

    right_ascension: f64,

    right_ascension_h: f64,
    right_ascension_h_str: String,

    right_ascension_min: f64,
    right_ascension_min_str: String,

    right_ascension_s: f64,
    right_ascension_s_str: String,

    radial_velocity: f64,
    radial_velocity_str: String,

    proper_motion_ra: f64,
    proper_motion_ra_str: String,

    proper_motion_dec: f64,
    proper_motion_dec_str: String,

    lotn: f64,
    aop: f64,
    i: f64,

    lotn_str: String,
    aop_str: String,
    i_str: String,

    a: f64,
    a_arcsec: f64,
    e: f64,
    period: f64,
    time_since_periapsis: f64,

    a_str: String,
    e_str: String,
    period_str: String,
    time_since_periapsis_str: String,

    companion_position_x: f64,
    companion_position_y: f64,

    companion_velocity_x: f64,
    companion_velocity_y: f64,

    companion_relative_position_x: f64,
    companion_relative_position_y: f64,
    companion_relative_position_z: f64,

    companion_relative_velocity_x: f64,
    companion_relative_velocity_y: f64,
    companion_relative_velocity_z: f64,

    x1: f64,
    x2: f64,
    x3: f64,

    y1: f64,
    y2: f64,
    y3: f64,

    z1: f64,
    z2: f64,
    z3: f64,

    name_toggle: bool,
    position_toggle: bool,
    velocity_toggle: bool,
    euler_angle_transformations_toggle: bool,
    companion_position_toggle: bool,
    companion_velocity_toggle: bool,
    companion_relative_position_toggle: bool,
    companion_relative_velocity_toggle: bool,
    export_toggle: bool,

    au_arcsec_toggle: bool,
}

impl epi::App for Canvas {
    fn name(&self) -> &str {
        "SPV"
    }

    #[allow(unused_variables)]

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.faint_bg_color = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.code_bg_color = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.hyperlink_color = egui::Color32::from_rgb(255, 0, 0);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(173, 186, 199));

        style.visuals.window_corner_radius = 10.0;

        style.visuals.button_frame = true;

        style.visuals.collapsing_header_frame = true;

        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(35, 39, 46);

        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(0., egui::Color32::from_rgb(173, 186, 199));

        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;

        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke {
            width: 10.0,
            color: egui::Color32::from_rgb(173, 186, 199),
        };

        ctx.set_style(style);

        let font_droidsansmono = include_bytes!("data/Droid Sans Mono Nerd Font Complete Mono.otf");
        let mut font = FontDefinitions::default();

        font.font_data.insert(
            "Droid Sans Mono".to_string(),
            egui::FontData {
                font: Cow::from(&font_droidsansmono[..]),
                index: 0,
            },
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
        true
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::SidePanel::left("Tabs").show(ctx, |ui| {
            if ui.add(egui::Button::new("Name")).clicked() {
                self.name_toggle = !self.name_toggle
            }

            if ui.add(egui::Button::new("Position")).clicked() {
                self.position_toggle = !self.position_toggle
            }

            if ui.add(egui::Button::new("Velocity")).clicked() {
                self.velocity_toggle = !self.velocity_toggle
            }

            if ui
                .add(egui::Button::new(
                    "Euler angle
transformations",
                ))
                .clicked()
            {
                self.euler_angle_transformations_toggle = !self.euler_angle_transformations_toggle
            }

            if ui
                .add(egui::Button::new(
                    "Companion
position",
                ))
                .clicked()
            {
                self.companion_position_toggle = !self.companion_position_toggle
            }

            if ui
                .add(egui::Button::new(
                    "Companion
velocity",
                ))
                .clicked()
            {
                self.companion_velocity_toggle = !self.companion_velocity_toggle
            }

            if ui
                .add(egui::Button::new(
                    "Companion
relative position",
                ))
                .clicked()
            {
                self.companion_relative_position_toggle = !self.companion_relative_position_toggle
            }

            if ui
                .add(egui::Button::new(
                    "Companion
relative velocity",
                ))
                .clicked()
            {
                self.companion_relative_velocity_toggle = !self.companion_relative_velocity_toggle
            }

            if ui.add(egui::Button::new("Export")).clicked() {
                self.export_toggle = !self.export_toggle
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.add(egui::Button::new("Clear")).clicked() {
                    self.name_str = "".to_string();
                    self.parallax_str = "".to_string();
                    self.declination_degree_str = "".to_string();
                    self.declination_min_str = "".to_string();
                    self.declination_s_str = "".to_string();
                    self.right_ascension_h_str = "".to_string();
                    self.right_ascension_min_str = "".to_string();
                    self.right_ascension_s_str = "".to_string();
                    self.radial_velocity_str = "".to_string();
                    self.proper_motion_ra_str = "".to_string();
                    self.proper_motion_dec_str = "".to_string();
                    self.lotn_str = "".to_string();
                    self.aop_str = "".to_string();
                    self.i_str = "".to_string();
                    self.a_str = "".to_string();
                    self.e_str = "".to_string();
                    self.period_str = "".to_string();
                    self.time_since_periapsis_str = "".to_string();
                }

                if ui.add(egui::Button::new("Organize")).clicked() {
                    ui.ctx().memory().reset_areas();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let naming_window = egui::Window::new("Name")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.name_toggle {
                ui.vertical(|_ui| {
                    naming_window.show(ctx, |ui| {
                        ui.heading("System name");
                        let response = ui.add(egui::TextEdit::singleline(&mut self.name_str));
                        if response.changed() {}
                        ui.monospace(format!("{}", self.name_str));
                    });
                });
            }

            let position_window = egui::Window::new("Position")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.position_toggle {
                ui.vertical(|_ui| {
                    position_window.show(ctx, |ui| {
                        //Parallax
                        ui.heading("Parallax (mas)");
                        let response_parallax =
                            ui.add(egui::TextEdit::singleline(&mut self.parallax_str));
                        match parse_text_input(
                            response_parallax,
                            self.parallax_str.clone(),
                            self.parallax,
                        ) {
                            Ok(val) => self.parallax = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} mas", self.parallax));

                        //Right ascension
                        ui.heading("Right ascension");
                        ui.monospace("Hours (h)");
                        let response_right_ascension_h =
                            ui.add(egui::TextEdit::singleline(&mut self.right_ascension_h_str));
                        match parse_text_input(
                            response_right_ascension_h,
                            self.right_ascension_h_str.clone(),
                            self.right_ascension_h,
                        ) {
                            Ok(val) => self.right_ascension_h = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} h", self.right_ascension_h));
                        ui.monospace("Minutes (m)");
                        let response_right_ascension_min = ui.add(egui::TextEdit::singleline(
                            &mut self.right_ascension_min_str,
                        ));
                        match parse_text_input(
                            response_right_ascension_min,
                            self.right_ascension_min_str.clone(),
                            self.right_ascension_min,
                        ) {
                            Ok(val) => self.right_ascension_min = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} m", self.right_ascension_min));
                        ui.monospace("Seconds (s)");
                        let response_right_ascension_s =
                            ui.add(egui::TextEdit::singleline(&mut self.right_ascension_s_str));
                        match parse_text_input(
                            response_right_ascension_s,
                            self.right_ascension_s_str.clone(),
                            self.right_ascension_s,
                        ) {
                            Ok(val) => self.right_ascension_s = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} s", self.right_ascension_s));
                        self.right_ascension = spv_rs::common::right_ascension_total(
                            self.right_ascension_h,
                            self.right_ascension_min,
                            self.right_ascension_s,
                        );
                        ui.heading("Total right ascension");
                        ui.monospace(format!("{}°", self.right_ascension));

                        //Declination
                        ui.heading("Declination");
                        ui.monospace("Degrees (°)");
                        let response_declination_degree =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_degree_str));
                        match parse_text_input(
                            response_declination_degree,
                            self.declination_degree_str.clone(),
                            self.declination_degree,
                        ) {
                            Ok(val) => self.declination_degree = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}°", self.declination_degree));
                        ui.monospace("Minutes (')");
                        let response_declination_min =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_min_str));
                        match parse_text_input(
                            response_declination_min,
                            self.declination_min_str.clone(),
                            self.declination_min,
                        ) {
                            Ok(val) => self.declination_min = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}'", self.declination_min));
                        ui.monospace("Seconds ('')");
                        let response_declination_s =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_s_str));
                        match parse_text_input(
                            response_declination_s,
                            self.declination_s_str.clone(),
                            self.declination_s,
                        ) {
                            Ok(val) => self.declination_s = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}''", self.declination_s));

                        self.declination = spv_rs::common::declination_total(
                            self.declination_degree,
                            self.declination_min,
                            self.declination_s,
                        );
                        ui.heading("Total declination");
                        ui.monospace(format!("{}°", self.declination));

                        //Calculations
                        let position = spv_rs::position::position(
                            self.parallax,
                            self.right_ascension,
                            self.declination,
                        )
                        .to_array();
                        self.x = position[0];
                        self.y = position[1];
                        self.z = position[2];

                        //Display results
                        ui.heading("Resulting position (m)");
                        ui.monospace(format!("x = {} m", self.x));
                        ui.monospace(format!("y = {} m", self.y));
                        ui.monospace(format!("z = {} m", self.z));
                    });
                });
            }

            let velocity_window = egui::Window::new("Velocity")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.velocity_toggle {
                ui.vertical(|_ui| {
                    velocity_window.show(ctx, |ui| {
                        //Parallax
                        ui.heading("Parallax (mas)");
                        let response_parallax =
                            ui.add(egui::TextEdit::singleline(&mut self.parallax_str));
                        match parse_text_input(
                            response_parallax,
                            self.parallax_str.clone(),
                            self.parallax,
                        ) {
                            Ok(val) => self.parallax = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} mas", self.parallax));

                        //Right ascension
                        ui.heading("Right ascension");
                        ui.monospace("Hours (h)");
                        let response_right_ascension_h =
                            ui.add(egui::TextEdit::singleline(&mut self.right_ascension_h_str));
                        match parse_text_input(
                            response_right_ascension_h,
                            self.right_ascension_h_str.clone(),
                            self.right_ascension_h,
                        ) {
                            Ok(val) => self.right_ascension_h = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} h", self.right_ascension_h));
                        ui.monospace("Minutes (m)");
                        let response_right_ascension_min = ui.add(egui::TextEdit::singleline(
                            &mut self.right_ascension_min_str,
                        ));
                        match parse_text_input(
                            response_right_ascension_min,
                            self.right_ascension_min_str.clone(),
                            self.right_ascension_min,
                        ) {
                            Ok(val) => self.right_ascension_min = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} m", self.right_ascension_min));
                        ui.monospace("Seconds (s)");
                        let response_right_ascension_s =
                            ui.add(egui::TextEdit::singleline(&mut self.right_ascension_s_str));
                        match parse_text_input(
                            response_right_ascension_s,
                            self.right_ascension_s_str.clone(),
                            self.right_ascension_s,
                        ) {
                            Ok(val) => self.right_ascension_s = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} s", self.right_ascension_s));
                        self.right_ascension = spv_rs::common::right_ascension_total(
                            self.right_ascension_h,
                            self.right_ascension_min,
                            self.right_ascension_s,
                        );
                        ui.heading("Total right ascension");
                        ui.monospace(format!("{}°", self.right_ascension));

                        //Declination
                        ui.heading("Declination");
                        ui.monospace("Degrees (°)");
                        let response_declination_degree =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_degree_str));
                        match parse_text_input(
                            response_declination_degree,
                            self.declination_degree_str.clone(),
                            self.declination_degree,
                        ) {
                            Ok(val) => self.declination_degree = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}°", self.declination_degree));
                        ui.monospace("Minutes (')");
                        let response_declination_min =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_min_str));
                        match parse_text_input(
                            response_declination_min,
                            self.declination_min_str.clone(),
                            self.declination_min,
                        ) {
                            Ok(val) => self.declination_min = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}'", self.declination_min));
                        ui.monospace("Seconds ('')");
                        let response_declination_s =
                            ui.add(egui::TextEdit::singleline(&mut self.declination_s_str));
                        match parse_text_input(
                            response_declination_s,
                            self.declination_s_str.clone(),
                            self.declination_s,
                        ) {
                            Ok(val) => self.declination_s = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}''", self.declination_s));

                        self.declination = spv_rs::common::declination_total(
                            self.declination_degree,
                            self.declination_min,
                            self.declination_s,
                        );
                        ui.heading("Total declination");
                        ui.monospace(format!("{}°", self.declination));

                        //Proper motion
                        ui.heading("Proper motion");
                        ui.monospace("Right ascension (arcsecons/year)");
                        let response_proper_motion_ra =
                            ui.add(egui::TextEdit::singleline(&mut self.proper_motion_ra_str));
                        match parse_text_input(
                            response_proper_motion_ra,
                            self.proper_motion_ra_str.clone(),
                            self.proper_motion_ra,
                        ) {
                            Ok(val) => self.proper_motion_ra = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} as/yr", self.proper_motion_ra));
                        ui.monospace("Declination (arcsecons/year)");
                        let response_proper_motion_dec =
                            ui.add(egui::TextEdit::singleline(&mut self.proper_motion_dec_str));
                        match parse_text_input(
                            response_proper_motion_dec,
                            self.proper_motion_dec_str.clone(),
                            self.proper_motion_dec,
                        ) {
                            Ok(val) => self.proper_motion_dec = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} as/yr", self.proper_motion_dec));

                        //Radial velocity
                        ui.heading("Radial velocity (km/s)");
                        let response_radial_velocity =
                            ui.add(egui::TextEdit::singleline(&mut self.radial_velocity_str));
                        match parse_text_input(
                            response_radial_velocity,
                            self.radial_velocity_str.clone(),
                            self.radial_velocity,
                        ) {
                            Ok(val) => self.radial_velocity = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} km/s", self.radial_velocity));

                        //Calculations
                        let velocity = spv_rs::velocity::velocity(
                            self.parallax,
                            self.right_ascension,
                            self.declination,
                            self.proper_motion_ra,
                            self.proper_motion_dec,
                            self.radial_velocity,
                        )
                        .to_array();
                        self.x_v = velocity[0];
                        self.y_v = velocity[1];
                        self.z_v = velocity[2];

                        //Display results
                        ui.heading("Resulting velocity (m/s)");
                        ui.monospace(format!("x = {} m/s", self.x_v));
                        ui.monospace(format!("y = {} m/s", self.y_v));
                        ui.monospace(format!("z = {} m/s", self.z_v));
                    });
                });
            }

            let euler_angle_transformations_window =
                egui::Window::new("Euler angle transformations")
                    .auto_sized()
                    .collapsible(true)
                    .resizable(false);

            if self.euler_angle_transformations_toggle {
                ui.vertical(|_ui| {
                    euler_angle_transformations_window.show(ctx, |ui| {
                        //Longitude of the node
                        ui.heading("Longitude of the node (Ω)");
                        let response_lotn = ui.add(egui::TextEdit::singleline(&mut self.lotn_str));
                        match parse_text_input(response_lotn, self.lotn_str.clone(), self.lotn) {
                            Ok(val) => self.lotn = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.lotn));

                        //Argument of periastron
                        ui.heading("Argument of periastron (ω)");
                        let response_aop = ui.add(egui::TextEdit::singleline(&mut self.aop_str));
                        match parse_text_input(response_aop, self.aop_str.clone(), self.aop) {
                            Ok(val) => self.aop = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.aop));

                        //Inclination
                        ui.heading("Inclination (i)");
                        let response_i = ui.add(egui::TextEdit::singleline(&mut self.i_str));
                        match parse_text_input(response_i, self.i_str.clone(), self.i) {
                            Ok(val) => self.i = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.i));

                        //Calculations
                        let euler_angle_transformations =
                            spv_rs::coordinate_transforms::euler_angle_transformations(
                                self.lotn, self.aop, self.i,
                            )
                            .to_cols_array();
                        self.x1 = euler_angle_transformations[0];
                        self.x2 = euler_angle_transformations[1];
                        self.x3 = euler_angle_transformations[2];
                        self.y1 = euler_angle_transformations[3];
                        self.y2 = euler_angle_transformations[4];
                        self.y3 = euler_angle_transformations[5];
                        self.z1 = euler_angle_transformations[6];
                        self.z2 = euler_angle_transformations[7];
                        self.z3 = euler_angle_transformations[8];

                        //Display results
                        ui.heading("Resulting position (m)");
                        ui.monospace(format!("x1 = {}", self.x1));
                        ui.monospace(format!("x2 = {}", self.x2));
                        ui.monospace(format!("x3 = {}", self.x3));
                        ui.monospace(format!("y1 = {}", self.y1));
                        ui.monospace(format!("y2 = {}", self.y2));
                        ui.monospace(format!("y3 = {}", self.y3));
                        ui.monospace(format!("z1 = {}", self.z1));
                        ui.monospace(format!("z2 = {}", self.z2));
                        ui.monospace(format!("z3 = {}", self.z3));
                    });
                });
            }

            let companion_position_window = egui::Window::new("Companion position")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.companion_position_toggle {
                ui.vertical(|_ui| {
                    companion_position_window.show(ctx, |ui| {
                        //Parallax
                        ui.heading("Parallax (mas)");
                        let response_parallax =
                            ui.add(egui::TextEdit::singleline(&mut self.parallax_str));
                        match parse_text_input(
                            response_parallax,
                            self.parallax_str.clone(),
                            self.parallax,
                        ) {
                            Ok(val) => self.parallax = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} mas", self.parallax));
                        self.distance_parsec =
                            spv_rs::common::parallax_to_parsec(self.parallax);
                        ui.monospace(format!("{} parsec", self.distance_parsec));

                        //Semi major-axis
                        ui.heading("Semi-major axis (a)");
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("au")).clicked() {
                                self.au_arcsec_toggle = true;
                            }
                            if ui.add(egui::Button::new("arcseconds")).clicked() {
                                self.au_arcsec_toggle = false;
                            }
                        });
                        if self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} au", self.a_arcsec));
                            self.a = self.a_arcsec;
                        }
                        if !self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} arcseconds", self.a_arcsec));
                            self.a = spv_rs::common::a_to_au(
                                self.parallax,
                                self.a_arcsec,
                            );
                        }

                        //Eccentricity
                        ui.heading("Eccentricity (e)");
                        let response_e = ui.add(egui::TextEdit::singleline(&mut self.e_str));
                        match parse_text_input(response_e, self.e_str.clone(), self.e) {
                            Ok(val) => self.e = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}", self.e));

                        //Period
                        ui.heading("Period (P)");
                        let response_period =
                            ui.add(egui::TextEdit::singleline(&mut self.period_str));
                        match parse_text_input(
                            response_period,
                            self.period_str.clone(),
                            self.period,
                        ) {
                            Ok(val) => self.period = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.period));

                        //Time since periastron
                        ui.heading("Time since periapsis (t)");
                        let response_time_since_periapsis = ui.add(egui::TextEdit::singleline(
                            &mut self.time_since_periapsis_str,
                        ));
                        match parse_text_input(
                            response_time_since_periapsis,
                            self.time_since_periapsis_str.clone(),
                            self.time_since_periapsis,
                        ) {
                            Ok(val) => self.time_since_periapsis = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.time_since_periapsis));

                        //Calculations
                        let companion_position = spv_rs::position::companion_position(
                            self.a,
                            self.e,
                            self.period,
                            self.time_since_periapsis,
                        )
                        .to_array();
                        self.companion_position_x = companion_position[0];
                        self.companion_position_y = companion_position[1];

                        //Display results
                        ui.heading("Resulting Position (m)");
                        ui.monospace(format!("x = {} m", self.companion_position_x));
                        ui.monospace(format!("y = {} m", self.companion_position_y));
                    });
                });
            }

            let companion_velocity_window = egui::Window::new("Companion velocity")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.companion_velocity_toggle {
                ui.vertical(|_ui| {
                    companion_velocity_window.show(ctx, |ui| {
                        //Parallax
                        ui.heading("Parallax (mas)");
                        let response_parallax =
                            ui.add(egui::TextEdit::singleline(&mut self.parallax_str));
                        match parse_text_input(
                            response_parallax,
                            self.parallax_str.clone(),
                            self.parallax,
                        ) {
                            Ok(val) => self.parallax = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} mas", self.parallax));
                        self.distance_parsec =
                            spv_rs::common::parallax_to_parsec(self.parallax);
                        ui.monospace(format!("{} parsec", self.distance_parsec));

                        //Semi major-axis
                        ui.heading("Semi-major axis (a)");
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("au")).clicked() {
                                self.au_arcsec_toggle = true;
                            }
                            if ui.add(egui::Button::new("arcseconds")).clicked() {
                                self.au_arcsec_toggle = false;
                            }
                        });
                        if self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} au", self.a_arcsec));
                            self.a = self.a_arcsec;
                        }
                        if !self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} arcseconds", self.a_arcsec));
                            self.a = spv_rs::common::a_to_au(
                                self.parallax,
                                self.a_arcsec,
                            );
                        }

                        //Eccentricity
                        ui.heading("Eccentricity (e)");
                        let response_e = ui.add(egui::TextEdit::singleline(&mut self.e_str));
                        match parse_text_input(response_e, self.e_str.clone(), self.e) {
                            Ok(val) => self.e = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}", self.e));

                        //Period
                        ui.heading("Period (P)");
                        let response_period =
                            ui.add(egui::TextEdit::singleline(&mut self.period_str));
                        match parse_text_input(
                            response_period,
                            self.period_str.clone(),
                            self.period,
                        ) {
                            Ok(val) => self.period = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.period));

                        //Time since periastron
                        ui.heading("Time since periapsis (t)");
                        let response_time_since_periapsis = ui.add(egui::TextEdit::singleline(
                            &mut self.time_since_periapsis_str,
                        ));
                        match parse_text_input(
                            response_time_since_periapsis,
                            self.time_since_periapsis_str.clone(),
                            self.time_since_periapsis,
                        ) {
                            Ok(val) => self.time_since_periapsis = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.time_since_periapsis));

                        //Calculations
                        let companion_velocity = spv_rs::velocity::companion_velocity(
                            self.a,
                            self.e,
                            self.period,
                            self.time_since_periapsis,
                        )
                        .to_array();
                        self.companion_velocity_x = companion_velocity[0];
                        self.companion_velocity_y = companion_velocity[1];

                        //Display results
                        ui.heading("Resulting velocity (m/s)");
                        ui.monospace(format!("x = {} m/s", self.companion_velocity_x));
                        ui.monospace(format!("y = {} m/s", self.companion_velocity_y));
                    });
                });
            }

            let companion_relative_position_window =
                egui::Window::new("Companion relative position")
                    .auto_sized()
                    .collapsible(true)
                    .resizable(false);

            if self.companion_relative_position_toggle {
                ui.vertical(|_ui| {
                    companion_relative_position_window.show(ctx, |ui| {
                        //Parallax
                        ui.heading("Parallax (mas)");
                        let response_parallax =
                            ui.add(egui::TextEdit::singleline(&mut self.parallax_str));
                        match parse_text_input(
                            response_parallax,
                            self.parallax_str.clone(),
                            self.parallax,
                        ) {
                            Ok(val) => self.parallax = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} mas", self.parallax));
                        self.distance_parsec =
                            spv_rs::common::parallax_to_parsec(self.parallax);
                        ui.monospace(format!("{} parsec", self.distance_parsec));

                        //Semi major-axis
                        ui.heading("Semi-major axis (a)");
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("au")).clicked() {
                                self.au_arcsec_toggle = true;
                            }
                            if ui.add(egui::Button::new("arcseconds")).clicked() {
                                self.au_arcsec_toggle = false;
                            }
                        });
                        if self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} au", self.a_arcsec));
                            self.a = self.a_arcsec;
                        }
                        if !self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} arcseconds", self.a_arcsec));
                            self.a = spv_rs::common::a_to_au(
                                self.parallax,
                                self.a_arcsec,
                            );
                        }

                        //Eccentricity
                        ui.heading("Eccentricity (e)");
                        let response_e = ui.add(egui::TextEdit::singleline(&mut self.e_str));
                        match parse_text_input(response_e, self.e_str.clone(), self.e) {
                            Ok(val) => self.e = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}", self.e));

                        //Period
                        ui.heading("Period (P)");
                        let response_period =
                            ui.add(egui::TextEdit::singleline(&mut self.period_str));
                        match parse_text_input(
                            response_period,
                            self.period_str.clone(),
                            self.period,
                        ) {
                            Ok(val) => self.period = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.period));

                        //Time since periastron
                        ui.heading("Time since periapsis (t)");
                        let response_time_since_periapsis = ui.add(egui::TextEdit::singleline(
                            &mut self.time_since_periapsis_str,
                        ));
                        match parse_text_input(
                            response_time_since_periapsis,
                            self.time_since_periapsis_str.clone(),
                            self.time_since_periapsis,
                        ) {
                            Ok(val) => self.time_since_periapsis = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.time_since_periapsis));

                        //Longitude of the node
                        ui.heading("Longitude of the node (Ω)");
                        let response_lotn = ui.add(egui::TextEdit::singleline(&mut self.lotn_str));
                        match parse_text_input(response_lotn, self.lotn_str.clone(), self.lotn) {
                            Ok(val) => self.lotn = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.lotn));

                        //Argument of periastron
                        ui.heading("Argument of periastron (ω)");
                        let response_aop = ui.add(egui::TextEdit::singleline(&mut self.aop_str));
                        match parse_text_input(response_aop, self.aop_str.clone(), self.aop) {
                            Ok(val) => self.aop = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.aop));

                        //Inclination
                        ui.heading("Inclination (i)");
                        let response_i = ui.add(egui::TextEdit::singleline(&mut self.i_str));
                        match parse_text_input(response_i, self.i_str.clone(), self.i) {
                            Ok(val) => self.i = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.i));

                        //Calculations
                        let companion_relative_position =
                            spv_rs::position::companion_relative_position(
                                self.a,
                                self.e,
                                self.period,
                                self.time_since_periapsis,
                                self.lotn,
                                self.aop,
                                self.i,
                            )
                            .to_array();
                        self.companion_relative_position_x = companion_relative_position[0];
                        self.companion_relative_position_y = companion_relative_position[1];
                        self.companion_relative_position_z = companion_relative_position[2];

                        //Display results
                        ui.heading("Resulting position (m)");
                        ui.monospace(format!("x = {} m", self.companion_relative_position_x));
                        ui.monospace(format!("y = {} m", self.companion_relative_position_y));
                        ui.monospace(format!("z = {} m", self.companion_relative_position_y));
                    });
                });
            }

            let companion_relative_velocity_window =
                egui::Window::new("Companion relative velocity")
                    .auto_sized()
                    .collapsible(true)
                    .resizable(false);

            if self.companion_relative_velocity_toggle {
                ui.vertical(|_ui| {
                    companion_relative_velocity_window.show(ctx, |ui| {
                        //Parallax
                        ui.heading("Parallax (mas)");
                        let response_parallax =
                            ui.add(egui::TextEdit::singleline(&mut self.parallax_str));
                        match parse_text_input(
                            response_parallax,
                            self.parallax_str.clone(),
                            self.parallax,
                        ) {
                            Ok(val) => self.parallax = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} mas", self.parallax));
                        self.distance_parsec =
                            spv_rs::common::parallax_to_parsec(self.parallax);
                        ui.monospace(format!("{} parsec", self.distance_parsec));

                        //Semi major-axis
                        ui.heading("Semi-major axis (a)");
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("au")).clicked() {
                                self.au_arcsec_toggle = true;
                            }
                            if ui.add(egui::Button::new("arcseconds")).clicked() {
                                self.au_arcsec_toggle = false;
                            }
                        });
                        if self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} au", self.a_arcsec));
                            self.a = self.a_arcsec;
                        }
                        if !self.au_arcsec_toggle {
                            let response_a_arcsec =
                                ui.add(egui::TextEdit::singleline(&mut self.a_str));
                            match parse_text_input(
                                response_a_arcsec,
                                self.a_str.clone(),
                                self.a_arcsec,
                            ) {
                                Ok(val) => self.a_arcsec = val,
                                Err(ex) => {
                                    println!("ERROR -> {}", ex);
                                }
                            }
                            ui.monospace(format!("{} arcseconds", self.a_arcsec));
                            self.a = spv_rs::common::a_to_au(
                                self.parallax,
                                self.a_arcsec,
                            );
                        }

                        //Eccentricity
                        ui.heading("Eccentricity (e)");
                        let response_e = ui.add(egui::TextEdit::singleline(&mut self.e_str));
                        match parse_text_input(response_e, self.e_str.clone(), self.e) {
                            Ok(val) => self.e = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{}", self.e));

                        //Period
                        ui.heading("Period (P)");
                        let response_period =
                            ui.add(egui::TextEdit::singleline(&mut self.period_str));
                        match parse_text_input(
                            response_period,
                            self.period_str.clone(),
                            self.period,
                        ) {
                            Ok(val) => self.period = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.period));

                        //Time since periastron
                        ui.heading("Time since periapsis (t)");
                        let response_time_since_periapsis = ui.add(egui::TextEdit::singleline(
                            &mut self.time_since_periapsis_str,
                        ));
                        match parse_text_input(
                            response_time_since_periapsis,
                            self.time_since_periapsis_str.clone(),
                            self.time_since_periapsis,
                        ) {
                            Ok(val) => self.time_since_periapsis = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} years", self.time_since_periapsis));

                        //Longitude of the node
                        ui.heading("Longitude of the node (Ω)");
                        let response_lotn = ui.add(egui::TextEdit::singleline(&mut self.lotn_str));
                        match parse_text_input(response_lotn, self.lotn_str.clone(), self.lotn) {
                            Ok(val) => self.lotn = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.lotn));

                        //Argument of periastron
                        ui.heading("Argument of periastron (ω)");
                        let response_aop = ui.add(egui::TextEdit::singleline(&mut self.aop_str));
                        match parse_text_input(response_aop, self.aop_str.clone(), self.aop) {
                            Ok(val) => self.aop = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.aop));

                        //Inclination
                        ui.heading("Inclination (i)");
                        let response_i = ui.add(egui::TextEdit::singleline(&mut self.i_str));
                        match parse_text_input(response_i, self.i_str.clone(), self.i) {
                            Ok(val) => self.i = val,
                            Err(ex) => {
                                println!("ERROR -> {}", ex);
                            }
                        }
                        ui.monospace(format!("{} degrees", self.i));

                        //Calculations
                        let companion_relative_velocity =
                            spv_rs::velocity::companion_relative_velocity(
                                self.a,
                                self.e,
                                self.period,
                                self.time_since_periapsis,
                                self.lotn,
                                self.aop,
                                self.i,
                            )
                            .to_array();
                        self.companion_relative_velocity_x = companion_relative_velocity[0];
                        self.companion_relative_velocity_y = companion_relative_velocity[1];
                        self.companion_relative_velocity_z = companion_relative_velocity[2];

                        //Display results
                        ui.heading("Resulting velocity (m/s)");
                        ui.monospace(format!("x = {} m/s", self.companion_relative_velocity_x));
                        ui.monospace(format!("y = {} m/s", self.companion_relative_velocity_y));
                        ui.monospace(format!("z = {} m/s", self.companion_relative_velocity_z));
                    });
                });
            }

            let export_window = egui::Window::new("Export file")
                .auto_sized()
                .collapsible(true)
                .resizable(false);

            if self.export_toggle {
                export_window.show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Export file");

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
                                    self.x1,
                                    self.x2,
                                    self.x3,
                                    self.y1,
                                    self.y2,
                                    self.y3,
                                    self.z1,
                                    self.z2,
                                    self.z3,
                                    self.companion_position_x,
                                    self.companion_position_y,
                                    self.companion_velocity_x,
                                    self.companion_velocity_y,
                                    self.companion_relative_position_x,
                                    self.companion_relative_position_y,
                                    self.companion_relative_position_z,
                                    self.companion_relative_velocity_x,
                                    self.companion_relative_velocity_y,
                                    self.companion_relative_velocity_z,
                                );
                            }
                        });
                    });
                });
            }
        });
    }
}
