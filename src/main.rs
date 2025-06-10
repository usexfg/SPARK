use eframe::egui;
use egui_file_dialog::FileDialog;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

mod proof;

#[derive(Serialize, Deserialize, Default, Clone)]
struct ProofData {
    amount: String,
    term: String,
    tx_hash: String,
    block_hash: String,
    recipient: String,
    merkle_root: String,
    proof: Option<String>,
}

struct SparkApp {
    amount: String,
    term: String,
    tx_hash: String,
    block_hash: String,
    recipient: String,
    merkle_root: String,
    proof_output: String,
    proof: Option<Vec<u8>>,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    theme_dark: bool,
}

impl Default for SparkApp {
    fn default() -> Self {
        Self {
            amount: String::new(),
            term: String::new(),
            tx_hash: String::new(),
            block_hash: String::new(),
            recipient: String::new(),
            merkle_root: String::new(),
            proof_output: String::new(),
            proof: None,
            file_dialog: FileDialog::new(),
            picked_file: None,
            theme_dark: true,
        }
    }
}

impl eframe::App for SparkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Theming toggle
        egui::TopBottomPanel::top("theme_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.selectable_label(self.theme_dark, "Dark").clicked() {
                    self.theme_dark = true;
                    ctx.set_visuals(egui::Visuals::dark());
                }
                if ui.selectable_label(!self.theme_dark, "Light").clicked() {
                    self.theme_dark = false;
                    ctx.set_visuals(egui::Visuals::light());
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💥 SPARK 💥 zkProofGen (Rust)");

            ui.label("Deposit Amount:");
            ui.text_edit_singleline(&mut self.amount);

            ui.label("Term (in days):");
            ui.text_edit_singleline(&mut self.term);

            ui.label("Transaction Hash (hex):");
            ui.text_edit_singleline(&mut self.tx_hash);

            ui.label("Block Hash (hex):");
            ui.text_edit_singleline(&mut self.block_hash);

            ui.label("Recipient (hex):");
            ui.text_edit_singleline(&mut self.recipient);

            ui.label("Merkle Root (hex):");
            ui.text_edit_singleline(&mut self.merkle_root);

            if ui.button("Generate Proof").clicked() {
                if let (Ok(amount), Ok(term), Ok(tx_bytes), Ok(block_bytes), Ok(recipient_bytes), Ok(merkle_bytes)) = (
                    self.amount.parse::<u64>(),
                    self.term.parse::<u32>(),
                    hex::decode(&self.tx_hash),
                    hex::decode(&self.block_hash),
                    hex::decode(&self.recipient),
                    hex::decode(&self.merkle_root),
                ) {
                    match proof::generate_proof(amount, term, &tx_bytes, &block_bytes, &recipient_bytes, &merkle_bytes) {
                        Ok(proof_bytes) => {
                            self.proof = Some(proof_bytes.clone());
                            self.proof_output = format!(
                                "Proof generated: {}",
                                hex::encode(&proof_bytes)
                            );
                        }
                        Err(e) => {
                            self.proof_output = format!("Proof generation failed: {}", e);
                        }
                    }
                } else {
                    self.proof_output = "Invalid input for proof generation.".to_string();
                }
            }

            if ui.button("Verify Proof").clicked() {
                if let (Some(proof_bytes), Ok(amount), Ok(term), Ok(tx_bytes), Ok(block_bytes), Ok(recipient_bytes), Ok(merkle_bytes)) = (
                    &self.proof,
                    self.amount.parse::<u64>(),
                    self.term.parse::<u32>(),
                    hex::decode(&self.tx_hash),
                    hex::decode(&self.block_hash),
                    hex::decode(&self.recipient),
                    hex::decode(&self.merkle_root),
                ) {
                    match proof::verify_proof(proof_bytes, amount, term, &tx_bytes, &block_bytes, &recipient_bytes, &merkle_bytes) {
                        Ok(true) => self.proof_output.push_str("\nProof is valid."),
                        Ok(false) => self.proof_output.push_str("\nProof is invalid."),
                        Err(e) => self.proof_output.push_str(&format!("\nVerification error: {}", e)),
                    }
                } else {
                    self.proof_output.push_str("\nNo proof to verify or invalid input.");
                }
            }

            ui.horizontal(|ui| {
                if ui.button("Save Proof").clicked() {
                    self.file_dialog.save_file_dialog();
                }
                if ui.button("Load Proof").clicked() {
                    self.file_dialog.open_file_dialog();
                }
            });

            // File dialog update
            self.file_dialog.update(ctx);

            // Load file if requested
            if let Some(path) = self.file_dialog.file_to_open() {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(proof_data) = serde_json::from_str::<ProofData>(&data) {
                        self.amount = proof_data.amount;
                        self.term = proof_data.term;
                        self.tx_hash = proof_data.tx_hash;
                        self.block_hash = proof_data.block_hash;
                        self.recipient = proof_data.recipient;
                        self.merkle_root = proof_data.merkle_root;
                        self.proof = proof_data.proof.and_then(|s| hex::decode(&s).ok());
                        self.proof_output = format!("Loaded proof from file: {}", path.display());
                    } else {
                        self.proof_output = "Failed to parse proof file.".to_string();
                    }
                } else {
                    self.proof_output = "Failed to read proof file.".to_string();
                }
            }
            // Save file if requested
            if let Some(path) = self.file_dialog.file_to_save() {
                let proof_data = ProofData {
                    amount: self.amount.clone(),
                    term: self.term.clone(),
                    tx_hash: self.tx_hash.clone(),
                    block_hash: self.block_hash.clone(),
                    recipient: self.recipient.clone(),
                    merkle_root: self.merkle_root.clone(),
                    proof: self.proof.as_ref().map(|b| hex::encode(b)),
                };
                if let Ok(json) = serde_json::to_string_pretty(&proof_data) {
                    if let Err(e) = fs::write(&path, json) {
                        self.proof_output = format!("Failed to save proof: {}", e);
                    } else {
                        self.proof_output = format!("Proof saved to: {}", path.display());
                    }
                } else {
                    self.proof_output = "Failed to serialize proof.".to_string();
                }
            }

            ui.separator();
            ui.label("Proof Output:");
            ui.text_edit_multiline(&mut self.proof_output);
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "SPARK (Rust)",
        native_options,
        Box::new(|_cc| Box::new(SparkApp::default())),
    );
} 