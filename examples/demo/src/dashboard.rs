use crate::detail::ViewGroupDetail;
use crate::group_new::ViewGroupNew;
use crate::state::State;
use eframe::egui::{CentralPanel, Color32, Context, FontId, RichText, SidePanel};
use eframe::emath::Align;
use eframe::{egui, Frame};
use egui::{Layout, Ui};
use egui_listview::{ItemTrait, ListView};
use std::borrow::Cow;
use std::sync::RwLock;

static CURRENT_GROUP_ITEM: RwLock<Option<Item>> = RwLock::new(None);
static CURRENT_GROUP_ID: RwLock<Option<i64>> = RwLock::new(None);

pub struct Dashboard {
    pub(crate) items: Vec<Item>,
    pub(crate) state: Option<State>,
}

#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub desc: String,
}

impl ItemTrait for Item {
    type Data<'a> = ();

    fn id(&self, _data: Self::Data<'_>) -> egui::Id {
        egui::Id::new(self.id)
    }

    fn show(
        &self,
        selected: bool,
        _hover: bool,
        _ctx: &Context,
        ui: &mut Ui,
        _data: Self::Data<'_>,
    ) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label(format!("Name: {}", self.name));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(format!("ID: {}", self.id));
                });
            });
            ui.label(RichText::new(format!("ID: {:?}", self.id(_data))).color(Color32::GRAY));
        });

        if selected && *CURRENT_GROUP_ID.read().unwrap() != Some(self.id) {
            *CURRENT_GROUP_ID.write().unwrap() = Some(self.id);
            *CURRENT_GROUP_ITEM.write().unwrap() = Some(self.clone());
        }
    }

    fn hovered_text(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(&self.desc))
    }

    fn selected_item(&self, _data: Self::Data<'_>) {}

    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool {
        self.name.contains(text)
    }
}

impl eframe::App for Dashboard {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        SidePanel::left("order_group_list")
            .resizable(false)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.scope(|ui| {
                    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                        if ui.button("Add Group").clicked() {
                            self.state = Some(State::GroupNew(ViewGroupNew::new(
                                String::new(),
                                String::new(),
                            )));
                        }

                        ListView::new(self.items.iter(), ())
                            .title("Search".into())
                            .hold_text("something".into())
                            // .max_height(ui.available_height() - 150.0)
                            .show(ctx, ui);
                        if CURRENT_GROUP_ITEM.read().unwrap().is_some() {
                            let mut writer = CURRENT_GROUP_ITEM.write().unwrap();
                            if let Some(item) = std::mem::take(&mut *writer) {
                                self.state =
                                    Some(State::GroupDetail(ViewGroupDetail::new_by_item(item)));
                            }
                        }
                    });
                });
            });
        if let Some(state) = self.state.as_mut() {
            state.as_app().update(ctx, frame);
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("Nothing Selected")
                            .font(FontId::proportional(20.0))
                            .color(Color32::GRAY),
                    )
                });
            });
        }
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {
            items: {
                (0..100)
                    .map(|id| Item {
                        id,
                        name: format!("id-{id}-name"),
                        desc: format!("id-{id}-description"),
                    })
                    .collect()
            },
            state: None,
        }
    }
}
