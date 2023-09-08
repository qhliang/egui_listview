use crate::dashboard::Item;
use eframe::egui::Context;
use eframe::{egui, Frame};

pub struct ViewGroupDetail {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) desc: String,
}

impl eframe::App for ViewGroupDetail {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Group Detail:");
                ui.separator();
                ui.label(format!("ID: {}", self.id));
                ui.label(format!("name: {}", self.name));
                ui.label(format!("desc: {}", self.desc));
            });
        });
    }
}

impl ViewGroupDetail {
    pub fn new(id: i64, name: String, desc: String) -> Self {
        ViewGroupDetail { id, name, desc }
    }

    pub fn new_by_item(item: Item) -> Self {
        ViewGroupDetail {
            id: item.id,
            name: item.name,
            desc: item.desc,
        }
    }
}
