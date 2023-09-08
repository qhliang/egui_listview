use egui::{Id, Label, RichText, ScrollArea, Sense, TextEdit};
use std::borrow::Cow;

use crate::r#trait::ItemTrait;

pub struct ListView<'a, W: ItemTrait + 'a, L: Iterator<Item = &'a W>> {
    pub hold_text: Option<Cow<'a, str>>,
    pub items: L,
    pub data: W::Data<'a>,
}

impl<'a, W: ItemTrait + 'a, L: Iterator<Item = &'a W>> ListView<'a, W, L> {
    pub fn new(items: L, data: W::Data<'a>, hold_text: Option<impl Into<Cow<'a, str>>>) -> Self {
        Self {
            hold_text: hold_text.map(|x| x.into()),
            items,
            data,
        }
    }
}

impl<'a, W: ItemTrait + 'a, L: Iterator<Item = &'a W>> ListView<'a, W, L> {
    pub fn show(
        self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
    ) -> egui::InnerResponse<Option<&'a W>> {
        let mut selected_item = None;

        let mut resp = ui.vertical(|ui| {
            let ListView {
                hold_text,
                items,
                data,
            } = self;

            let resp = ui.group(|ui| {
                let root_id = ui.auto_id_with("ListView");
                let search_id = root_id.with("search");
                let selected_id = root_id.with("selected");
                let hovered_id = root_id.with("hovered");

                let mut search: String = ui.data_mut(|d| d.get_temp(search_id)).unwrap_or_default();
                let mut selected: Option<Id> =
                    ui.data_mut(|d| d.get_temp(selected_id)).unwrap_or_default();
                let old_selected = selected;
                let mut hovered: Option<Id> =
                    ui.data_mut(|d| d.get_temp(hovered_id)).unwrap_or_default();

                ui.horizontal_top(|ui| {
                    ui.add(Label::new(RichText::new(W::TITLE).strong()));
                    let mut search_text = TextEdit::singleline(&mut search);
                    if let Some(text) = hold_text {
                        search_text = search_text.hint_text(text);
                    }
                    ui.add(search_text)
                });

                ui.separator();

                ScrollArea::vertical()
                    .id_source(root_id.with("list"))
                    .hscroll(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Grid::new("list view container")
                            .num_columns(1)
                            .striped(W::STRIPED)
                            .show(ui, |ui| {
                                for item in items {
                                    let id = item.id(data);
                                    let checked = selected == Some(id);
                                    let hover = hovered == Some(id);

                                    if checked {
                                        selected_item = Some(item);
                                    }

                                    if search.is_empty() || item.on_search(&search, data) {
                                        let mut child_frame = egui::Frame::default()
                                            .inner_margin(W::INNER_MARGIN)
                                            .outer_margin(W::OUTER_MARGIN)
                                            .rounding(W::ROUNDING);
                                        if checked {
                                            item.style_clicked(&mut child_frame);
                                        } else if hover {
                                            item.style_hovered(&mut child_frame);
                                        } else {
                                            item.style_normal(&mut child_frame);
                                        }
                                        let mut interact_area = child_frame
                                            .show(ui, |ui| {
                                                item.show(checked, hover, ctx, ui, data);
                                                ui.interact(
                                                    ui.max_rect(),
                                                    item.id(data),
                                                    Sense::click(),
                                                )
                                            })
                                            .inner;
                                        if let Some(tips) = item.hovered_text() {
                                            interact_area = interact_area.on_hover_text(tips);
                                        }

                                        if interact_area.hovered() && hovered != Some(id) {
                                            hovered = Some(id);
                                        }

                                        if interact_area.clicked() && !checked {
                                            selected = Some(id);
                                            selected_item = Some(item);
                                        }

                                        ui.end_row();
                                    }
                                }
                            });
                    });

                if let Some(item) = selected_item {
                    item.selected_item(data);
                }

                ui.data_mut(|d| {
                    d.insert_temp(search_id, search);
                    d.insert_temp(selected_id, selected);
                    d.insert_temp(hovered_id, hovered);
                });

                old_selected != selected
            });

            resp.inner
        });

        if resp.inner {
            resp.response.mark_changed();
        }

        egui::InnerResponse::new(selected_item, resp.response)
    }
}
