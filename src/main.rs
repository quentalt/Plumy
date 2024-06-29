use eframe::{egui, App, Frame};
use serde::{Serialize, Deserialize};
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::{Read, Write, Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note {
    id: u32,
    title: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Folder {
    id: u32,
    name: String,
    notes: Vec<Note>,
}

#[derive(Serialize, Deserialize, Default)]
struct NoteApp {
    folders: Vec<Folder>,
    selected_folder: Option<usize>,
    selected_note: Option<usize>,
    search_query: String,
    new_tag: String, // Ajout du champ new_tag
}

impl NoteApp {
    fn new() -> Self {
        Self {
            new_tag: String::new(), // Initialisation de new_tag
            ..Default::default()
        }
    }
}

impl App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Top panel for search bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
            });
        });

        // Side panel for folders
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Folders");
            for (i, folder) in self.folders.iter().enumerate() {
                if ui.button(&folder.name).clicked() {
                    self.selected_folder = Some(i);
                    self.selected_note = None;
                }
            }
            if ui.button("Add Folder").clicked() {
                self.folders.push(Folder {
                    id: self.folders.len() as u32,
                    name: format!("Folder {}", self.folders.len() + 1),
                    notes: vec![],
                });
            }
        });

        // Central panel for notes
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(folder_index) = self.selected_folder {
                let folder = &mut self.folders[folder_index];
                ui.vertical(|ui| {
                    ui.heading(&folder.name);
                    for (i, note) in folder.notes.iter().enumerate() {
                        if ui.button(&note.title).clicked() {
                            self.selected_note = Some(i);
                        }
                    }
                    if ui.button("Add Note").clicked() {
                        folder.notes.push(Note {
                            id: folder.notes.len() as u32,
                            title: format!("Note {}", folder.notes.len() + 1),
                            content: String::new(),
                            tags: vec![],
                        });
                    }
                });

                ui.separator();

                if let Some(note_index) = self.selected_note {
                    let note = &mut folder.notes[note_index];
                    ui.vertical(|ui| {
                        ui.heading(&note.title);
                        ui.horizontal(|ui| {
                            ui.label("Content:");
                            ui.text_edit_multiline(&mut note.content);
                        });
                        ui.separator();
                        ui.label("Tags:");
                        for tag in &note.tags {
                            ui.label(tag);
                        }
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.new_tag);
                            if ui.button("Add Tag").clicked() {
                                if !self.new_tag.is_empty() {
                                    note.tags.push(self.new_tag.clone());
                                    self.new_tag.clear(); // Réinitialiser new_tag après l'ajout
                                }
                            }
                        });
                    });
                }
            }
        });
    }
}

fn main() {
    let app = NoteApp::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Note App",
        native_options,
        Box::new(|_cc| Box::new(app)),
    ).expect("Failed to run app");
}
