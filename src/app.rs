use base64;
use chrono::prelude::*;
use eframe::{egui, epi};
use std::str;
use urlencoding::decode;
use urlencoding::encode;
use uuid::Uuid;
use sqlformat::{QueryParams,FormatOptions,Indent};

#[derive(PartialEq)]
enum ToolType {
    UrlEncoding,
    Base64,
    UUID,
    UnixTime,
    JsonFormat,
    SqlFormat,
}

#[derive(PartialEq)]
enum Direction {
    Decode,
    Encode,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    current_tool: ToolType,
    current_input: String,
    current_output: String,
    current_direction: Direction,
    //
    //    // this how you opt-out of serialization of a member
    //    #[cfg_attr(feature = "persistence", serde(skip))]
    //    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            current_tool: ToolType::UrlEncoding,
            current_input: String::new(),
            current_output: String::new(),
            current_direction: Direction::Encode,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Dev Toolbox"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            current_tool,
            current_input,
            current_output,
            current_direction,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Toolbox");
            if ui
                .selectable_value(current_tool, ToolType::UrlEncoding, "URL Encoding/Decoding")
                .clicked()
            {
                current_input.clear();
                current_output.clear();
            }
            if ui
                .selectable_value(current_tool, ToolType::Base64, "Base64 Encoding/Decoding")
                .clicked()
            {
                current_input.clear();
                current_output.clear();
            }
            if ui
                .selectable_value(current_tool, ToolType::UUID, "UUID Generator")
                .clicked()
            {
                current_input.clear();
                current_output.clear();
            }
            if ui
                .selectable_value(current_tool, ToolType::UnixTime, "Unix Time Converter")
                .clicked()
            {
                current_input.clear();
                current_output.clear();
            }
            if ui
                .selectable_value(current_tool, ToolType::JsonFormat, "Json Formatter")
                .clicked()
            {
                current_input.clear();
                current_output.clear();
            }
            if ui
                .selectable_value(current_tool, ToolType::SqlFormat, "SQL Formatter")
                .clicked()
            {
                current_input.clear();
                current_output.clear();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            match current_tool {
                ToolType::UrlEncoding => {
                    ui.heading("URL Encoding/Decoding");
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui
                            .radio_value(current_direction, Direction::Encode, "Encode")
                            .clicked()
                        {
                            current_input.clear();
                            current_output.clear();
                        }
                        if ui
                            .radio_value(current_direction, Direction::Decode, "Decode")
                            .clicked()
                        {
                            current_input.clear();
                            current_output.clear();
                        }
                        if ui.button("clear").clicked() {
                            current_input.clear();
                            current_output.clear();
                        }
                    });
                    ui.label("Input:");
                    let input = ui
                        .add(egui::TextEdit::multiline(current_input).desired_width(f32::INFINITY));
                    if input.changed() {
                        match current_direction {
                            Direction::Encode => {
                                let result = encode(current_input).into_owned();
                                *current_output = result;
                            }
                            Direction::Decode => {
                                let result = decode(current_input).expect("UTF-8").into_owned();
                                *current_output = result;
                            }
                        }
                    }
                    ui.separator();
                    ui.label("Output:");
                    ui.add(egui::TextEdit::multiline(current_output).desired_width(f32::INFINITY));
                }
                ToolType::Base64 => {
                    ui.heading("Base64 Encoding/Decoding");
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui
                            .radio_value(current_direction, Direction::Encode, "Encode")
                            .clicked()
                        {
                            current_input.clear();
                            current_output.clear();
                        }
                        if ui
                            .radio_value(current_direction, Direction::Decode, "Decode")
                            .clicked()
                        {
                            current_input.clear();
                            current_output.clear();
                        }
                        if ui.button("clear").clicked() {
                            current_input.clear();
                            current_output.clear();
                        }
                    });
                    ui.label("Input:");
                    let input = ui
                        .add(egui::TextEdit::multiline(current_input).desired_width(f32::INFINITY));
                    if input.changed() {
                        match current_direction {
                            Direction::Encode => {
                                let result = base64::encode(current_input);
                                *current_output = result;
                            }
                            Direction::Decode => {
                                let result = base64::decode(current_input);
                                let contents = result.unwrap_or_default();
                                *current_output =
                                    str::from_utf8(&contents).unwrap_or_default().to_owned();
                            }
                        }
                    }
                    ui.separator();
                    ui.label("Output:");
                    ui.add(egui::TextEdit::multiline(current_output).desired_width(f32::INFINITY));
                }
                ToolType::UUID => {
                    ui.heading("UUID Generator");
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("generate").clicked() {
                            *current_output = Uuid::new_v4().to_hyphenated().to_string();
                        }
                    });
                    ui.label("Output:");
                    ui.add(egui::TextEdit::multiline(current_output).desired_width(f32::INFINITY));
                }
                ToolType::UnixTime => {
                    ui.heading("Unix Time Converter");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("UTC Now:");
                        ui.label(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Epoch input:");
                        ui.text_edit_singleline(current_input);
                    });

                    let timestamp = current_input.parse::<i64>();
                    let mut utc_output;
                    let mut local_output;
                    match timestamp {
                        Ok(ts) => {
                            let naive = NaiveDateTime::from_timestamp(ts, 0);
                            let datetime_utc: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                            let datetime_local: DateTime<Local> = DateTime::from(datetime_utc);
                            utc_output = datetime_utc.format("%Y-%m-%d %H:%M:%S").to_string();
                            local_output = datetime_local.format("%Y-%m-%d %H:%M:%S").to_string();
                        }
                        Err(_) => {
                            utc_output = "N/A".to_string();
                            local_output = "N/A".to_string();
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.label("Formatted UTC Time:");
                        ui.label(&mut utc_output);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Formatted Local Time:");
                        ui.label(&mut local_output);
                    });
                }
                ToolType::JsonFormat => {
                    ui.heading("Json Formatter");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Input:");
                        let input = ui.add(
                            egui::TextEdit::multiline(current_input).desired_width(f32::INFINITY),
                        );
                        if input.changed() {
                            let result = jsonxf::pretty_print(current_input);
                            let contents = result.unwrap_or("Invalid".to_string());
                            *current_output = contents;
                        }
                        ui.separator();
                        ui.label("Output:");
                        ui.add(
                            egui::TextEdit::multiline(current_output).desired_width(f32::INFINITY),
                        );
                    });
                }
                ToolType::SqlFormat => {
                    ui.heading("SQL Formatter");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Input:");
                        let input = ui.add(
                            egui::TextEdit::multiline(current_input).desired_width(f32::INFINITY),
                        );
                        if input.changed() {
                            let opts = FormatOptions {
                                indent: Indent::Spaces(4),
                                uppercase: true,
                                lines_between_queries: 1,
                            };
                            let result = sqlformat::format(current_input, &QueryParams::None, opts);
                            *current_output = result;
                        }
                        ui.separator();
                        ui.label("Output:");
                        ui.add(
                            egui::TextEdit::multiline(current_output).desired_width(f32::INFINITY),
                        );
                    });
                }
            }
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
