use eframe::{run_native, NativeOptions, App};
use egui::{Response, Ui};
use enigma::{ReflectorType, Rotor, RotorType, Plugboard, Enigma};

struct EnigmaGui {
    input: String,
    output: String,
    reflector_type: ReflectorType,
    rotor_type: [RotorType; 3],
    rotor_ring: [usize; 3],
    rotor_key: [char; 3],
    current_plug_pair: [char; 2],
    plugs: Vec<[char; 2]>,
    enigma: Enigma,
}

impl EnigmaGui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut rotors: Vec<Rotor> = Vec::with_capacity(3);
        rotors.push(Rotor::new(RotorType::I, 'A', 1).unwrap());
        rotors.push(Rotor::new(RotorType::II, 'A', 1).unwrap());
        rotors.push(Rotor::new(RotorType::III, 'A', 1).unwrap());

        let plugs: Vec<[char; 2]> = Vec::new();
        let plugboard = Plugboard::new(&plugs).unwrap();

        EnigmaGui {
            input: String::new(),
            output: String::new(),
            reflector_type: ReflectorType::B,
            rotor_type: [RotorType::I, RotorType::II, RotorType::III],
            rotor_ring: [1, 1, 1],
            rotor_key: ['A', 'A', 'A'],
            current_plug_pair: ['A', 'Z'],
            plugs: Vec::new(),
            enigma: Enigma::new(ReflectorType::B, rotors, plugboard),
        }
    }

    fn apply_settings(&mut self) {
        let mut rotors: Vec<Rotor> = Vec::with_capacity(3);

        for i in 0..3 {
            let rotor = Rotor::new(self.rotor_type[i], self.rotor_key[i], self.rotor_ring[i]).unwrap();
            rotors.push(rotor);
        }

        let plugboard = Plugboard::new(&self.plugs).unwrap();

        self.enigma = Enigma::new(self.reflector_type, rotors, plugboard)
    }

    fn is_plug_available(&self, c: char) -> bool {
        for p in &self.plugs {
            if p.contains(&c) {
                return false;
            }
        }

        true
    }

    fn next_available_plug(&self, reverse: bool) -> Option<char> {
        if reverse {
            for c in ('A'..='Z').rev() {
                if self.is_plug_available(c) {
                    return Some(c);
                }
            }
        }
        else {
            for c in 'A'..='Z' {
                if self.is_plug_available(c) {
                    return Some(c);
                }
            }
        }

        None
    }

    fn remove_plug_pair(&mut self, pair: [char; 2]) {
        let mut index: Option<usize> = None;

        for i in 0..self.plugs.len() {
            if pair == self.plugs[i] {
                index = Some(i);
            }
        }

        if let Some(i) = index {
            self.plugs.remove(i);
        }
    }

    fn add_reflector(&mut self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Reflector");

                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_source("reflector-combobox")
                        .selected_text(format!("{:?}", self.reflector_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::Beta, "Beta");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::Gamma, "Gamma");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::A, "A");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::B, "B");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::C, "C");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::ThinB, "ThinB");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::ThinC, "ThinC");
                            ui.selectable_value(&mut self.reflector_type, ReflectorType::ETW, "ETW");
                        });
                });
            });
        }).response
    }

    fn add_rotor(&mut self, ui: &mut Ui, rotor_index: usize) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_source(format!("rotor-combobox{rotor_index}"))
                        .selected_text(format!("{:?}", self.rotor_type[rotor_index]))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::I, "I");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::II, "II");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::III, "III");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::IV, "IV");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::V, "V");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::VI, "VI");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::VII, "VII");
                            ui.selectable_value(&mut self.rotor_type[rotor_index], RotorType::VIII, "VIII");
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Ring:");
                    ui.add(egui::Slider::new(&mut self.rotor_ring[rotor_index], 1..=26));
                });

                ui.horizontal(|ui| {
                    ui.label("Key:");
                    egui::ComboBox::from_id_source(format!("key-combobox{rotor_index}"))
                        .selected_text(self.rotor_key[rotor_index].to_string())
                        .show_ui(ui, |ui| {
                            for c in 'A'..='Z' {
                                if ui.selectable_label(false, c.to_string()).clicked() {
                                    self.rotor_key[rotor_index] = c;
                                }
                            }
                        });
                });
            });
        }).response
    }

    fn add_rotors(&mut self, ui: &mut Ui) -> Response {
        ui.group(|ui| { 
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Rotors");
                });

                ui.horizontal(|ui| {
                    self.add_rotor(ui, 0);
                    self.add_rotor(ui, 1);
                    self.add_rotor(ui, 2);
                });
            });
        }).response
    }

    fn update_selected_plugs(&mut self) {
        // Update the current characters in the plug selection comboboxes to an available letter.
        if let Some(c) = self.next_available_plug(false) {
            self.current_plug_pair[0] = c;
        }

        if let Some(c) = self.next_available_plug(true) {
            self.current_plug_pair[1] = c;
        }
    }

    fn add_plugboard_combobox(&mut self, ui: &mut Ui, index: usize) -> Response {
        egui::ComboBox::from_id_source(format!("plugboard-combobox{index}"))
            .selected_text(self.current_plug_pair[index].to_string())
            .width(25.0)
            .show_ui(ui, |ui| {
                for c in 'A'..='Z' {
                    if ui.add_enabled(self.is_plug_available(c), egui::SelectableLabel::new(false, c.to_string())).clicked() {
                        self.current_plug_pair[index] = c;
                    }
                }
            }).response
    }

    fn add_plugboard(&mut self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Plugboard");
            });

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        self.add_plugboard_combobox(ui, 0);
                        self.add_plugboard_combobox(ui, 1);
                    });

                    if ui.button("Add Pair").clicked() {
                        // We can't have more than 13 pairs.
                        if self.plugs.len() < 13 {
                            self.plugs.push(self.current_plug_pair);
                        }

                        self.update_selected_plugs();
                    }
                });

                let mut clicked_pairs: Vec<[char; 2]> = Vec::new();

                for p in &self.plugs {
                    if ui.button(format!("{0} {1}", p[0], p[1])).clicked() {
                        clicked_pairs.push(*p);
                    }
                }

                // Remove any clicked on pairs from our vec.
                if !clicked_pairs.is_empty() {
                    for p in clicked_pairs {
                        self.remove_plug_pair(p);
                    }

                    self.update_selected_plugs();
                }
            });
        }).response
    }
}

impl App for EnigmaGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Settings");
                });

                ui.horizontal(|ui| {
                    self.add_reflector(ui);
                    self.add_rotors(ui);
                });

                ui.add_space(10.0);

                self.add_plugboard(ui);

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Apply Settings").clicked() {
                        self.apply_settings();
                    }
                });
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("Input");

                if ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::singleline(&mut self.input)).lost_focus() {
                    self.output = self.enigma.encrypt(&self.input.to_uppercase()).unwrap();
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("Output");
                ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.output.as_str()));
            });
        });
    }
}

fn main() {
    let window_options = NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(egui::Vec2::new(810 as f32, 600 as f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        run_and_return: true,
    };

    run_native("Enigma", window_options, Box::new(|cc| Box::new(EnigmaGui::new(cc))));
}