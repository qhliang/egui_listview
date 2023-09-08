use eframe::egui::Context;
use eframe::{egui, Frame};

pub struct ViewGroupNew {
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) msg: Option<String>,
}

impl eframe::App for ViewGroupNew {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default()
            // .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("New Group");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Group Name");
                        ui.text_edit_singleline(&mut self.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Group Description");
                        ui.text_edit_singleline(&mut self.desc);
                    });
                    ui.separator();
                    if ui.button("Add").clicked() {
                        self.name = self.name.trim().to_string();

                        if self.name.is_empty() {
                            self.msg = Some("invalid name".into());
                        } else {
                            self.msg = Some(format!("group {} added", self.name));
                        }
                    }
                    if let Some(msg) = &self.msg {
                        ui.label(msg);
                    }
                });
            });
    }
}

impl ViewGroupNew {
    pub fn new(name: String, desc: String) -> Self {
        ViewGroupNew {
            name,
            desc,
            msg: None,
        }
    }
}
