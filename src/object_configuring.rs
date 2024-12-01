//! Copyright 2024 - The Open-Agriculture Developers
//! SPDX-License-Identifier: GPL-3.0-or-later
//! Authors: Daan Steenbergen

use crate::allowed_object_relationships::get_allowed_child_refs;
use crate::allowed_object_relationships::AllowedChildRefs;
use crate::possible_events::PossibleEvents;
use crate::EditorProject;

use ag_iso_stack::object_pool::object::*;
use ag_iso_stack::object_pool::object_attributes::DataCodeType;
use ag_iso_stack::object_pool::object_attributes::Event;
use ag_iso_stack::object_pool::object_attributes::FormatType;
use ag_iso_stack::object_pool::object_attributes::HorizontalAlignment;
use ag_iso_stack::object_pool::object_attributes::MacroRef;
use ag_iso_stack::object_pool::object_attributes::PictureGraphicFormat;
use ag_iso_stack::object_pool::object_attributes::Point;
use ag_iso_stack::object_pool::object_attributes::VerticalAlignment;
use ag_iso_stack::object_pool::vt_version::VtVersion;
use ag_iso_stack::object_pool::NullableObjectId;
use ag_iso_stack::object_pool::ObjectId;
use ag_iso_stack::object_pool::ObjectPool;
use ag_iso_stack::object_pool::ObjectRef;
use ag_iso_stack::object_pool::ObjectType;
use eframe::egui;

pub trait ConfigurableObject {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    );
}

impl ConfigurableObject for Object {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        match self {
            Object::WorkingSet(o) => o.render_parameters(ui, design, navigation_selected),
            Object::DataMask(o) => o.render_parameters(ui, design, navigation_selected),
            Object::AlarmMask(o) => o.render_parameters(ui, design, navigation_selected),
            Object::Container(o) => o.render_parameters(ui, design, navigation_selected),
            Object::SoftKeyMask(o) => o.render_parameters(ui, design, navigation_selected),
            Object::Key(o) => o.render_parameters(ui, design, navigation_selected),
            Object::Button(o) => o.render_parameters(ui, design, navigation_selected),
            Object::InputBoolean(o) => o.render_parameters(ui, design, navigation_selected),
            Object::InputString(o) => o.render_parameters(ui, design, navigation_selected),
            Object::InputNumber(o) => o.render_parameters(ui, design, navigation_selected),
            Object::InputList(o) => o.render_parameters(ui, design, navigation_selected),
            Object::OutputString(o) => o.render_parameters(ui, design, navigation_selected),
            Object::OutputNumber(o) => o.render_parameters(ui, design, navigation_selected),
            Object::OutputList(o) => (),
            Object::OutputLine(o) => (),
            Object::OutputRectangle(o) => (),
            Object::OutputEllipse(o) => (),
            Object::OutputPolygon(o) => (),
            Object::OutputMeter(o) => (),
            Object::OutputLinearBarGraph(o) => (),
            Object::OutputArchedBarGraph(o) => (),
            Object::PictureGraphic(o) => o.render_parameters(ui, design, navigation_selected),
            Object::NumberVariable(o) => (),
            Object::StringVariable(o) => (),
            Object::FontAttributes(o) => (),
            Object::LineAttributes(o) => (),
            Object::FillAttributes(o) => (),
            Object::InputAttributes(o) => (),
            Object::ObjectPointer(o) => o.render_parameters(ui, design, navigation_selected),
            Object::Macro(o) => (),
            Object::AuxiliaryFunctionType1(o) => (),
            Object::AuxiliaryInputType1(o) => (),
            Object::AuxiliaryFunctionType2(o) => (),
            Object::AuxiliaryInputType2(o) => (),
            Object::AuxiliaryControlDesignatorType2(o) => (),
            Object::WindowMask(o) => (),
            Object::KeyGroup(o) => (),
            Object::GraphicsContext(o) => (),
            Object::ExtendedInputAttributes(o) => (),
            Object::ColourMap(o) => (),
            Object::ObjectLabelReferenceList(o) => (),
            Object::ExternalObjectDefinition(o) => (),
            Object::ExternalReferenceName(o) => (),
            Object::ExternalObjectPointer(o) => (),
            Object::Animation(o) => (),
            Object::ColourPalette(o) => (),
            Object::GraphicData(o) => (),
            Object::WorkingSetSpecialControls(o) => (),
            Object::ScaledGraphic(o) => (),
        }
    }
}

fn render_object_id(
    ui: &mut egui::Ui,
    id: &mut ObjectId,
    pool: &ObjectPool,
    navigation_selected: &mut NullableObjectId,
) {
    let mut temp_id = u16::from(*id);

    ui.horizontal(|ui| {
        ui.label("Object ID:");

        let resp = ui.add(
            egui::DragValue::new(&mut temp_id)
                .speed(1.0)
                .clamp_range(0..=65534),
        );

        let new_id = ObjectId::new(temp_id).unwrap();

        // Check if the new ID is already used by another object (excluding the current object)
        let conflict = pool.object_by_id(new_id).is_some() && new_id != *id;

        let conflict_storage = ui.id().with("conflict");
        let was_conflict = ui.data(|data| data.get_temp::<u16>(conflict_storage));

        if conflict || was_conflict.is_some_and(|id| id == temp_id) {
            ui.colored_label(egui::Color32::RED, "ID already in use!");

            // Save the conflict in storage so it is still displayed next frame
            ui.data_mut(|data| {
                data.insert_temp(conflict_storage, u16::from(*id));
            });
        } else if resp.changed() || was_conflict.is_some_and(|id| id != temp_id) {
            // Remove the conflict from storage if we are actively changing the ID,
            // or if the ID has changed (most likely another object is selected)
            ui.data_mut(|data| {
                data.remove_temp::<u16>(conflict_storage);
            });
        }

        // Update the ID when editing is finished and there's no conflict
        if !conflict && (resp.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter))) {
            *id = new_id;
            navigation_selected.0 = Some(*id);
        }
    });
}

fn render_object_id_selector(
    ui: &mut egui::Ui,
    idx: usize,
    pool: &ObjectPool,
    object_id: &mut ObjectId,
    allowed_child_objects: &[ObjectType],
) {
    egui::ComboBox::from_id_source(format!("object_id_selector_{}", idx))
        .selected_text(format!("{:?}", object_id.value()))
        .width(0.0)
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            for potential_child in pool.objects_by_types(allowed_child_objects) {
                ui.selectable_value(
                    object_id,
                    potential_child.id(),
                    format!(
                        "{:?}: {:?}",
                        u16::from(potential_child.id()),
                        potential_child.object_type()
                    ),
                );
            }
        });
}

fn render_nullable_object_id_selector(
    ui: &mut egui::Ui,
    idx: usize,
    pool: &ObjectPool,
    object_id: &mut NullableObjectId,
    allowed_child_objects: &[ObjectType],
) {
    egui::ComboBox::from_id_source(format!("nullable_object_id_selector_{}", idx))
        .selected_text(
            object_id
                .0
                .map_or("None".to_string(), |id| format!("{:?}", id.value())),
        )
        .width(0.0)
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.selectable_value(object_id, NullableObjectId::NULL, "None");
            for potential_child in pool.objects_by_types(allowed_child_objects) {
                ui.selectable_value(
                    object_id,
                    potential_child.id().into(),
                    format!(
                        "{:?}: {:?}",
                        u16::from(potential_child.id()),
                        potential_child.object_type()
                    ),
                );
            }
        });
}

fn render_index_modifiers<T>(ui: &mut egui::Ui, idx: usize, list: &mut Vec<T>) {
    if ui
        .add_enabled(idx > 0, egui::Button::new("\u{23F6}"))
        .on_hover_text("Move up")
        .clicked()
    {
        list.swap(idx, idx - 1);
    }
    if ui
        .add_enabled(idx < list.len() - 1, egui::Button::new("\u{23F7}"))
        .on_hover_text("Move down")
        .clicked()
    {
        list.swap(idx, idx + 1);
    }
    if ui.button("\u{1F5D9}").on_hover_text("Remove").clicked() {
        list.remove(idx);
    }
}

fn render_object_references_list(
    ui: &mut egui::Ui,
    pool: &ObjectPool,
    width: u16,
    height: u16,
    object_refs: &mut Vec<ObjectRef>,
    allowed_child_objects: &[ObjectType],
    navigation_selected: &mut NullableObjectId,
) {
    egui::Grid::new("object_ref_grid")
        .striped(true)
        .min_col_width(0.0)
        .show(ui, |ui| {
            let mut idx = 0;
            while idx < object_refs.len() {
                let obj_ref = &mut object_refs[idx];
                let obj = pool.object_by_id(obj_ref.id);

                ui.label(" - ");
                render_object_id_selector(ui, idx, pool, &mut obj_ref.id, allowed_child_objects);

                if let Some(obj) = obj {
                    let mut max_x = width as i16;
                    let mut max_y = height as i16;
                    if let Some(sized_obj) = obj.as_sized_object() {
                        max_x -= sized_obj.width() as i16;
                        max_y -= sized_obj.height() as i16;
                    }
                    if ui.link(format!("{:?}", obj.object_type())).clicked() {
                        *navigation_selected = obj.id().into();
                    }

                    ui.add(
                        egui::Slider::new(&mut obj_ref.offset.x, 0..=max_x)
                            .text("X")
                            .drag_value_speed(1.0),
                    );
                    ui.add(
                        egui::Slider::new(&mut obj_ref.offset.y, 0..=max_y)
                            .text("Y")
                            .drag_value_speed(1.0),
                    );
                } else {
                    ui.colored_label(egui::Color32::RED, "Missing object");
                }

                render_index_modifiers(ui, idx, object_refs);
                idx += 1;
                ui.end_row();
            }
        });

    let (new_object_id, _) = render_add_object_id(ui, pool, allowed_child_objects, false);
    if let Some(id) = new_object_id {
        object_refs.push(ObjectRef {
            id,
            offset: Point::default(),
        });
    }
}

fn render_object_id_list(
    ui: &mut egui::Ui,
    pool: &ObjectPool,
    object_ids: &mut Vec<ObjectId>,
    allowed_child_objects: &[ObjectType],
    navigation_selected: &mut NullableObjectId,
) {
    egui::Grid::new("object_id_grid")
        .striped(true)
        .min_col_width(0.0)
        .show(ui, |ui| {
            let mut idx = 0;
            while idx < object_ids.len() {
                let obj: Option<&Object> = pool.object_by_id(object_ids[idx]);

                ui.label(" - ");
                render_object_id_selector(
                    ui,
                    idx,
                    pool,
                    &mut object_ids[idx],
                    allowed_child_objects,
                );

                if let Some(obj) = obj {
                    if ui.link(format!("{:?}", obj.object_type())).clicked() {
                        *navigation_selected = obj.id().into();
                    }
                } else {
                    ui.colored_label(egui::Color32::RED, "Missing object");
                }

                render_index_modifiers(ui, idx, object_ids);
                idx += 1;
                ui.end_row();
            }
        });
    let (new_object_id, _) = render_add_object_id(ui, pool, allowed_child_objects, false);
    if let Some(id) = new_object_id {
        object_ids.push(id);
    }
}

fn render_nullable_object_id_list(
    ui: &mut egui::Ui,
    pool: &ObjectPool,
    nullable_object_ids: &mut Vec<NullableObjectId>,
    allowed_child_objects: &[ObjectType],
    navigation_selected: &mut NullableObjectId,
) {
    egui::Grid::new("object_id_grid")
        .striped(true)
        .min_col_width(0.0)
        .show(ui, |ui| {
            let mut idx = 0;
            while idx < nullable_object_ids.len() {
                ui.label(" - ");
                render_nullable_object_id_selector(
                    ui,
                    idx,
                    pool,
                    &mut nullable_object_ids[idx],
                    allowed_child_objects,
                );
                if let Some(object_id) = &mut nullable_object_ids[idx].0 {
                    let obj: Option<&Object> = pool.object_by_id(*object_id);

                    if let Some(obj) = obj {
                        if ui.link(format!("{:?}", obj.object_type())).clicked() {
                            *navigation_selected = obj.id().into();
                        }
                    } else {
                        ui.colored_label(egui::Color32::RED, "Missing object");
                    }
                } else {
                    ui.label(""); // Empty cell
                }
                render_index_modifiers(ui, idx, nullable_object_ids);
                idx += 1;
                ui.end_row();
            }
        });

    let (new_object_id, success) = render_add_object_id(ui, pool, allowed_child_objects, true);
    if success {
        nullable_object_ids.push(NullableObjectId(new_object_id));
    }
}

fn render_add_object_id(
    ui: &mut egui::Ui,
    pool: &ObjectPool,
    allowed_child_objects: &[ObjectType],
    allow_none: bool,
) -> (Option<ObjectId>, bool) {
    let mut result = (None, false);
    ui.horizontal(|ui| {
        ui.label("Add object:");
        egui::ComboBox::from_id_source("New Object Type")
            .selected_text("Select existing object")
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                if allow_none {
                    if ui.selectable_label(false, "None").clicked() {
                        result = (None, true);
                    }
                }
                for potential_child in pool.objects_by_types(allowed_child_objects) {
                    if ui
                        .selectable_label(
                            false,
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        )
                        .clicked()
                    {
                        result = (Some(potential_child.id()), true);
                    }
                }
            });
    });
    result
}

fn render_macro_references(
    ui: &mut egui::Ui,
    pool: &ObjectPool,
    macro_refs: &mut Vec<MacroRef>,
    possible_events: &[Event],
    navigation_selected: &mut NullableObjectId,
) {
    egui::Grid::new("macro_grid")
        .striped(true)
        .min_col_width(0.0)
        .show(ui, |ui| {
            let mut idx = 0;
            while idx < macro_refs.len() {
                let macro_ref = &mut macro_refs[idx];

                if let Some(macro_obj) = pool
                    .objects_by_type(ObjectType::Macro)
                    .iter()
                    .find(|o| u16::from(o.id()) == macro_ref.macro_id as u16)
                {
                    ui.label(" - ");
                    ui.push_id(idx, |ui| {
                        egui::ComboBox::from_id_source("event_id")
                            .selected_text(format!("{:?}", macro_ref.event_id))
                            .width(0.0)
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                for event in possible_events {
                                    ui.selectable_value(
                                        &mut macro_ref.event_id,
                                        *event,
                                        format!("{:?}", event),
                                    );
                                }
                            });

                        if ui.link(" Macro ").clicked() {
                            *navigation_selected = macro_obj.id().into();
                        }

                        egui::ComboBox::from_id_source("macro_id")
                            .selected_text(format!("{:?}", macro_ref.macro_id))
                            .width(0.0)
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                for potential_macro in pool.objects_by_type(ObjectType::Macro) {
                                    ui.selectable_value(
                                        &mut macro_ref.macro_id,
                                        u16::from(potential_macro.id()) as u8,
                                        format!("{:?}", u16::from(potential_macro.id())),
                                    );
                                }
                            });
                    });
                } else {
                    ui.label(format!(
                        "- {:?}: Missing macro object {:?}",
                        macro_ref.event_id, macro_ref.macro_id
                    ));
                }

                render_index_modifiers(ui, idx, macro_refs);
                idx += 1;
                ui.end_row();
            }
        });

    render_add_macro_reference(ui, pool, macro_refs, possible_events);
}

fn render_add_macro_reference(
    ui: &mut egui::Ui,
    pool: &ObjectPool,
    macro_refs: &mut Vec<MacroRef>,
    possible_events: &[Event],
) {
    ui.horizontal(|ui| {
        ui.label("Add macro:");
        ui.horizontal(|ui| {
            let mut selected_event = ui.data_mut(|data| {
                data.get_temp(egui::Id::new("selected_event"))
                    .unwrap_or(Event::Reserved)
            });
            egui::ComboBox::from_id_source("New Event Type")
                .selected_text(if selected_event == Event::Reserved {
                    "Select event".to_string()
                } else {
                    format!("{:?}", selected_event)
                })
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for event in possible_events {
                        if ui
                            .selectable_value(&mut selected_event, *event, format!("{:?}", event))
                            .changed()
                        {
                            ui.data_mut(|data| {
                                data.insert_temp(egui::Id::new("selected_event"), selected_event);
                            });
                        }
                    }
                });

            if selected_event != Event::Reserved {
                egui::ComboBox::from_id_source("New Macro")
                    .selected_text("Select macro")
                    .width(0.0)
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        for potential_macro in pool.objects_by_type(ObjectType::Macro) {
                            if ui
                                .selectable_label(
                                    false,
                                    format!("{:?}", u16::from(potential_macro.id())),
                                )
                                .clicked()
                            {
                                macro_refs.push(MacroRef {
                                    event_id: selected_event,
                                    macro_id: u16::from(potential_macro.id()) as u8,
                                });
                            }
                        }
                    });
            }
        });
    });
}

impl ConfigurableObject for WorkingSet {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.checkbox(&mut self.selectable, "Selectable");
        ui.horizontal(|ui| {
            let masks = design
                .get_pool()
                .objects_by_types(&[ObjectType::DataMask, ObjectType::AlarmMask]);
            egui::ComboBox::from_label("Active Mask")
                .selected_text(format!("{:?}", u16::from(self.active_mask)))
                .show_ui(ui, |ui| {
                    for object in masks {
                        ui.selectable_value(
                            &mut self.active_mask,
                            object.id(),
                            format!("{:?}", u16::from(object.id())),
                        );
                    }
                });
            if ui.link("(view)").clicked() {
                *navigation_selected = self.active_mask.into();
            }
        });
        ui.separator();
        ui.label("Objects:");
        render_object_references_list(
            ui,
            design.get_pool(),
            design.mask_size,
            design.mask_size,
            &mut self.object_refs,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for DataMask {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Soft Key Mask")
                .selected_text(
                    self.soft_key_mask
                        .0
                        .map_or("None".to_string(), |id| format!("{:?}", u16::from(id))),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.soft_key_mask,
                        NullableObjectId(None),
                        "None".to_string(),
                    );
                    for object in design.get_pool().objects_by_type(ObjectType::SoftKeyMask) {
                        ui.selectable_value(
                            &mut self.soft_key_mask,
                            NullableObjectId(Some(object.id())),
                            format!("{:?}", u16::from(object.id())),
                        );
                    }
                });
            if let Some(mask) = self.soft_key_mask.0 {
                if ui.link("(view)").clicked() {
                    *navigation_selected = mask.into();
                }
            }
        });
        ui.separator();
        ui.label("Objects:");
        render_object_references_list(
            ui,
            design.get_pool(),
            design.mask_size,
            design.mask_size,
            &mut self.object_refs,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for AlarmMask {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Soft Key Mask")
                .selected_text(
                    self.soft_key_mask
                        .0
                        .map_or("None".to_string(), |id| format!("{:?}", u16::from(id))),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.soft_key_mask,
                        NullableObjectId(None),
                        "None".to_string(),
                    );
                    for object in design.get_pool().objects_by_type(ObjectType::SoftKeyMask) {
                        ui.selectable_value(
                            &mut self.soft_key_mask,
                            NullableObjectId(Some(object.id())),
                            format!("{:?}", u16::from(object.id())),
                        );
                    }
                });
            if let Some(mask) = self.soft_key_mask.0 {
                if ui.link("(view)").clicked() {
                    *navigation_selected = mask.into();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Priority:");
            ui.radio_value(&mut self.priority, 2, "Low");
            ui.radio_value(&mut self.priority, 1, "Medium");
            ui.radio_value(&mut self.priority, 0, "High");
        });
        ui.horizontal(|ui| {
            ui.label("Acoustic signal:");
            ui.radio_value(&mut self.acoustic_signal, 3, "None");
            ui.radio_value(&mut self.acoustic_signal, 2, "Lowest");
            ui.radio_value(&mut self.acoustic_signal, 1, "Medium");
            ui.radio_value(&mut self.acoustic_signal, 0, "Highest");
        });
        ui.separator();
        ui.label("Objects:");
        render_object_references_list(
            ui,
            design.get_pool(),
            design.mask_size,
            design.mask_size,
            &mut self.object_refs,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for Container {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.checkbox(&mut self.hidden, "Hidden");
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.separator();
        ui.label("Objects:");
        render_object_references_list(
            ui,
            design.get_pool(),
            self.width,
            self.height,
            &mut self.object_refs,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for SoftKeyMask {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.separator();
        ui.label("Objects:");
        render_object_id_list(
            ui,
            design.get_pool(),
            &mut self.objects,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for Key {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            ui.label("Key code:");
            ui.radio_value(&mut self.key_code, 0, "ACK");
            ui.add(egui::DragValue::new(&mut self.key_code).speed(1));
        });
        ui.separator();
        ui.label("Objects:");
        render_object_references_list(
            ui,
            design.get_pool(),
            design.mask_size,
            design.mask_size,
            &mut self.object_refs,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for Button {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.checkbox(&mut self.options.no_border, "No Border");
        ui.checkbox(
            &mut self.options.transparent_background,
            "Transparent Background",
        );
        ui.checkbox(&mut self.options.suppress_border, "Suppress Border");
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.separator();
        ui.label("Objects:");
        render_object_references_list(
            ui,
            design.get_pool(),
            self.width,
            self.height,
            &mut self.object_refs,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for InputBoolean {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        egui::ComboBox::from_id_source("foreground_colour")
            .selected_text(format!("{:?}", u16::from(self.foreground_colour)))
            .width(0.0)
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                for potential_child in design
                    .get_pool()
                    .objects_by_type(ObjectType::FontAttributes)
                {
                    ui.selectable_value(
                        &mut self.foreground_colour,
                        potential_child.id(),
                        format!(
                            "{:?}: {:?}",
                            u16::from(potential_child.id()),
                            potential_child.object_type()
                        ),
                    );
                }
            });
        ui.horizontal(|ui| {
            ui.label("Variable reference:");
            egui::ComboBox::from_id_source("variable_reference")
                .selected_text(format!("{:?}", u16::from(self.variable_reference)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(
                        &mut self.variable_reference,
                        NullableObjectId::NULL,
                        "None",
                    );
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::NumberVariable)
                    {
                        ui.selectable_value(
                            &mut self.variable_reference,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
        });
        if self.variable_reference.0.is_none() {
            ui.label("Initial value:");
            egui::ComboBox::from_id_source("initial_value")
                .selected_text(format!("{:?}", self.value))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(&mut self.value, false, "False");
                    ui.selectable_value(&mut self.value, true, "True");
                });
        }
        ui.checkbox(&mut self.enabled, "Enabled");
        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for InputString {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            ui.label("Font attributes:");
            egui::ComboBox::from_id_source("font_attributes")
                .selected_text(format!("{:?}", u16::from(self.font_attributes)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::FontAttributes)
                    {
                        ui.selectable_value(
                            &mut self.font_attributes,
                            potential_child.id(),
                            format!("{:?}", u16::from(potential_child.id())),
                        );
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Input attributes:");
            egui::ComboBox::from_id_source("input_attributes")
                .selected_text(format!("{:?}", u16::from(self.input_attributes)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(&mut self.input_attributes, NullableObjectId::NULL, "None");
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::InputAttributes)
                    {
                        ui.selectable_value(
                            &mut self.input_attributes,
                            potential_child.id().into(),
                            format!("{:?}", u16::from(potential_child.id())),
                        );
                    }
                });
        });
        ui.checkbox(&mut self.options.transparent, "Transparent Background");
        ui.checkbox(&mut self.options.auto_wrap, "Auto Wrap");
        // TODO: check if we have VT version 4 or later
        // if self.options.auto_wrap {
        //     ui.checkbox(&mut self.options.wrap_on_hyphen, "Wrap on Hyphen");
        // }
        ui.horizontal(|ui| {
            ui.label("Variable reference:");
            egui::ComboBox::from_id_source("variable_reference")
                .selected_text(format!("{:?}", u16::from(self.variable_reference)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(
                        &mut self.variable_reference,
                        NullableObjectId::NULL,
                        "None",
                    );
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::StringVariable)
                    {
                        ui.selectable_value(
                            &mut self.variable_reference,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Horizontal Justification:");
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Left,
                "Left",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Middle,
                "Middle",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Right,
                "Right",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Reserved,
                "Reserved",
            );
        });
        // TODO: check if we have VT version 4 or later
        // ui.horizontal(|ui| {
        //     ui.label("Vertical Justification:");
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Top,
        //         "Top",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Middle,
        //         "Middle",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Bottom,
        //         "Bottom",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Reserved,
        //         "Reserved",
        //     );
        // });
        if self.variable_reference.0.is_none() {
            ui.label("Initial value:");
            ui.text_edit_singleline(&mut self.value);
        }
        ui.checkbox(&mut self.enabled, "Enabled");
        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for InputNumber {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            ui.label("Font attributes:");
            egui::ComboBox::from_id_source("font_attributes")
                .selected_text(format!("{:?}", u16::from(self.font_attributes)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::FontAttributes)
                    {
                        ui.selectable_value(
                            &mut self.font_attributes,
                            potential_child.id(),
                            format!("{:?}", u16::from(potential_child.id())),
                        );
                    }
                });
        });
        ui.checkbox(&mut self.options.transparent, "Transparent Background");
        ui.checkbox(
            &mut self.options.display_leading_zeros,
            "Display Leading Zeros",
        );
        ui.checkbox(
            &mut self.options.display_zero_as_blank,
            "Display Zero as Blank",
        );
        // TODO: check if we have VT version 4 or later
        // ui.checkbox(&mut self.options.truncate, "Truncate");
        ui.horizontal(|ui| {
            ui.label("Variable reference:");
            egui::ComboBox::from_id_source("variable_reference")
                .selected_text(format!("{:?}", u16::from(self.variable_reference)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(
                        &mut self.variable_reference,
                        NullableObjectId::NULL,
                        "None",
                    );
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::NumberVariable)
                    {
                        ui.selectable_value(
                            &mut self.variable_reference,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
        });
        if self.variable_reference.0.is_none() {
            ui.label("Initial value:");
            ui.add(egui::DragValue::new(&mut self.value).speed(1.0));
        }
        ui.add(
            egui::DragValue::new(&mut self.min_value)
                .speed(1.0)
                .prefix("Min: "),
        );
        ui.add(
            egui::DragValue::new(&mut self.max_value)
                .speed(1.0)
                .prefix("Max: "),
        );
        ui.add(
            egui::DragValue::new(&mut self.offset)
                .speed(1.0)
                .prefix("Offset: "),
        );
        ui.add(egui::DragValue::new(&mut self.scale).prefix("Scale: "));
        ui.add(
            egui::DragValue::new(&mut self.nr_of_decimals)
                .speed(1.0)
                .prefix("Number of Decimals: "),
        );
        ui.horizontal(|ui| {
            ui.label("Format:");
            ui.radio_value(&mut self.format, FormatType::Decimal, "Decimal");
            ui.radio_value(&mut self.format, FormatType::Exponential, "Exponential");
        });

        ui.horizontal(|ui| {
            ui.label("Horizontal Justification:");
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Left,
                "Left",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Middle,
                "Middle",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Right,
                "Right",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Reserved,
                "Reserved",
            );
        });
        // TODO: check if we have VT version 4 or later
        // ui.horizontal(|ui| {
        //     ui.label("Vertical Justification:");
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Top,
        //         "Top",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Middle,
        //         "Middle",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Bottom,
        //         "Bottom",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Reserved,
        //         "Reserved",
        //     );
        // });

        ui.checkbox(&mut self.options2.enabled, "Enabled");
        // TODO: check if we have VT version 4 or later
        // ui.checkbox(&mut self.options2.real_time_editing, "Real Time Editing");

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for InputList {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            ui.label("Variable reference:");
            egui::ComboBox::from_id_source("variable_reference")
                .selected_text(format!("{:?}", u16::from(self.variable_reference)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(
                        &mut self.variable_reference,
                        NullableObjectId::NULL,
                        "None",
                    );
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::NumberVariable)
                    {
                        ui.selectable_value(
                            &mut self.variable_reference,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
        });
        if self.variable_reference.0.is_none() {
            ui.label("Initial value:");
            ui.add(egui::DragValue::new(&mut self.value).speed(1.0));
        }

        ui.checkbox(&mut self.options.enabled, "Enabled");
        // TODO: check if we have VT version 4 or later
        // ui.checkbox(&mut self.options.real_time_editing, "Real Time Editing");

        ui.separator();
        ui.label("List items:");
        render_nullable_object_id_list(
            ui,
            design.get_pool(),
            &mut self.list_items,
            &Self::get_allowed_child_refs(VtVersion::Version3),
            navigation_selected,
        );

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for OutputString {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            ui.label("Font attributes:");
            egui::ComboBox::from_id_source("font_attributes")
                .selected_text(format!("{:?}", u16::from(self.font_attributes)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::FontAttributes)
                    {
                        ui.selectable_value(
                            &mut self.font_attributes,
                            potential_child.id(),
                            format!("{:?}", u16::from(potential_child.id())),
                        );
                    }
                });
        });
        ui.checkbox(&mut self.options.transparent, "Transparent Background");
        ui.checkbox(&mut self.options.auto_wrap, "Auto Wrap");
        // TODO: check if we have VT version 4 or later
        // if self.options.auto_wrap {
        //     ui.checkbox(&mut self.options.wrap_on_hyphen, "Wrap on Hyphen");
        // }
        ui.horizontal(|ui| {
            ui.label("Variable reference:");
            egui::ComboBox::from_id_source("variable_reference")
                .selected_text(format!("{:?}", u16::from(self.variable_reference)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(
                        &mut self.variable_reference,
                        NullableObjectId::NULL,
                        "None",
                    );
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::StringVariable)
                    {
                        ui.selectable_value(
                            &mut self.variable_reference,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Horizontal Justification:");
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Left,
                "Left",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Middle,
                "Middle",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Right,
                "Right",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Reserved,
                "Reserved",
            );
        });
        // TODO: check if we have VT version 4 or later
        // ui.horizontal(|ui| {
        //     ui.label("Vertical Justification:");
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Top,
        //         "Top",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Middle,
        //         "Middle",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Bottom,
        //         "Bottom",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Reserved,
        //         "Reserved",
        //     );
        // });
        if self.variable_reference.0.is_none() {
            ui.label("Initial value:");
            ui.text_edit_singleline(&mut self.value);
        }
        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for OutputNumber {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.height, 0..=design.mask_size)
                .text("Height")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut self.background_colour, 0..=255)
                .text("Background Colour")
                .drag_value_speed(1.0),
        );
        ui.horizontal(|ui| {
            ui.label("Font attributes:");
            egui::ComboBox::from_id_source("font_attributes")
                .selected_text(format!("{:?}", u16::from(self.font_attributes)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::FontAttributes)
                    {
                        ui.selectable_value(
                            &mut self.font_attributes,
                            potential_child.id(),
                            format!("{:?}", u16::from(potential_child.id())),
                        );
                    }
                });
        });
        ui.checkbox(&mut self.options.transparent, "Transparent Background");
        ui.checkbox(
            &mut self.options.display_leading_zeros,
            "Display Leading Zeros",
        );
        ui.checkbox(
            &mut self.options.display_zero_as_blank,
            "Display Zero as Blank",
        );
        // TODO: check if we have VT version 4 or later
        // ui.checkbox(&mut self.options.truncate, "Truncate");
        ui.horizontal(|ui| {
            ui.label("Variable reference:");
            egui::ComboBox::from_id_source("variable_reference")
                .selected_text(format!("{:?}", u16::from(self.variable_reference)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(
                        &mut self.variable_reference,
                        NullableObjectId::NULL,
                        "None",
                    );
                    for potential_child in design
                        .get_pool()
                        .objects_by_type(ObjectType::NumberVariable)
                    {
                        ui.selectable_value(
                            &mut self.variable_reference,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
        });
        if self.variable_reference.0.is_none() {
            ui.label("Initial value:");
            ui.add(egui::DragValue::new(&mut self.value).speed(1.0));
        }
        ui.add(
            egui::DragValue::new(&mut self.offset)
                .speed(1.0)
                .prefix("Offset: "),
        );
        ui.add(egui::DragValue::new(&mut self.scale).prefix("Scale: "));
        ui.add(
            egui::DragValue::new(&mut self.nr_of_decimals)
                .speed(1.0)
                .prefix("Number of Decimals: "),
        );
        ui.horizontal(|ui| {
            ui.label("Format:");
            ui.radio_value(&mut self.format, FormatType::Decimal, "Decimal");
            ui.radio_value(&mut self.format, FormatType::Exponential, "Exponential");
        });

        ui.horizontal(|ui| {
            ui.label("Horizontal Justification:");
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Left,
                "Left",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Middle,
                "Middle",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Right,
                "Right",
            );
            ui.radio_value(
                &mut self.justification.horizontal,
                HorizontalAlignment::Reserved,
                "Reserved",
            );
        });
        // TODO: check if we have VT version 4 or later
        // ui.horizontal(|ui| {
        //     ui.label("Vertical Justification:");
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Top,
        //         "Top",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Middle,
        //         "Middle",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Bottom,
        //         "Bottom",
        //     );
        //     ui.radio_value(
        //         &mut self.justification.vertical,
        //         VerticalAlignment::Reserved,
        //         "Reserved",
        //     );
        // });

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for PictureGraphic {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.add(
            egui::Slider::new(&mut self.width, 0..=design.mask_size)
                .text("Width")
                .drag_value_speed(1.0),
        );
        ui.label(format!("Actual Image Width: {}", self.actual_width));
        ui.label(format!("Actual Image Height: {}", self.actual_height));
        ui.horizontal(|ui| {
            ui.label("Format:");
            if ui
                .radio(
                    self.format == PictureGraphicFormat::Monochrome,
                    "Monochrome",
                )
                .clicked()
            {
                match self.format {
                    PictureGraphicFormat::FourBit => {
                        self.data = self
                            .data_as_raw_encoded()
                            .windows(4)
                            .step_by(4)
                            .flat_map(|chunk| {
                                let mut byte = 0;
                                for (i, bit) in chunk.iter().enumerate() {
                                    for j in 0..2 {
                                        if *bit & (1 << j) != 0 {
                                            byte |= 1 << (i * 2 + j);
                                        }
                                    }
                                }
                                vec![byte]
                            })
                            .collect();
                    }
                    PictureGraphicFormat::EightBit => {
                        self.data = self
                            .data_as_raw_encoded()
                            .windows(8)
                            .step_by(8)
                            .flat_map(|chunk| {
                                let mut byte = 0;
                                for (i, bit) in chunk.iter().enumerate() {
                                    if *bit != 0 {
                                        byte |= 1 << i;
                                    }
                                }
                                vec![byte]
                            })
                            .collect();
                    }
                    _ => {}
                }
                self.format = PictureGraphicFormat::Monochrome;
                self.options.data_code_type = DataCodeType::Raw;
            }
            if ui
                .radio(self.format == PictureGraphicFormat::FourBit, "4-bit colour")
                .clicked()
            {
                match self.format {
                    PictureGraphicFormat::Monochrome => {
                        self.data = self
                            .data_as_raw_encoded()
                            .iter()
                            .flat_map(|value| {
                                let mut result = vec![];
                                for idx in 0..8 {
                                    let bit_color = value << idx & 0x01;
                                    if idx % 2 == 0 {
                                        result.push(bit_color);
                                    } else if let Some(last) = result.last_mut() {
                                        *last |= bit_color >> 4;
                                    }
                                }
                                result
                            })
                            .collect();
                    }
                    PictureGraphicFormat::EightBit => {
                        self.data = self
                            .data_as_raw_encoded()
                            .windows(2)
                            .step_by(2)
                            .flat_map(|values| {
                                let high = (values[0] & 0x0F) << 4;
                                let low = values[1] & 0x0F;
                                vec![high | low]
                            })
                            .collect();
                    }
                    _ => {}
                }
                self.format = PictureGraphicFormat::FourBit;
                self.options.data_code_type = DataCodeType::Raw;
            }
            if ui
                .radio(
                    self.format == PictureGraphicFormat::EightBit,
                    "8-bit colour",
                )
                .clicked()
            {
                match self.format {
                    PictureGraphicFormat::Monochrome => {
                        self.data = self
                            .data_as_raw_encoded()
                            .iter()
                            .flat_map(|value| {
                                let mut result = vec![];
                                for bit in 0..8 {
                                    result.push(value >> bit & 0x01);
                                }
                                result
                            })
                            .collect();
                    }
                    PictureGraphicFormat::FourBit => {
                        self.data = self
                            .data_as_raw_encoded()
                            .iter()
                            .flat_map(|value| {
                                let high = (value >> 4) & 0x0F;
                                let low = value & 0x0F;
                                vec![high, low]
                            })
                            .collect();
                    }
                    _ => {}
                }
                self.format = PictureGraphicFormat::EightBit;
                self.options.data_code_type = DataCodeType::Raw;
            }
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.options.transparent, "Transparent Pixels");
            if self.options.transparent {
                ui.add(
                    egui::Slider::new(&mut self.transparency_colour, 0..=255)
                        .text("Transparent Colour")
                        .drag_value_speed(1.0),
                );
            }
        });
        ui.checkbox(&mut self.options.flashing, "Flashing");

        // if let Some(dialog) =
        //     ui.data(|data| data.get_temp::<Arc<Mutex<FileDialog>>>(Id::new("file_dialog")))
        // {
        //     let mut dialog = dialog.lock().unwrap();
        //     if dialog.show(ui.ctx()).selected() {
        //         if let Some(path) = dialog.path() {
        //             let image = image::io::Reader::open(path).unwrap().decode().unwrap();
        //             self.actual_width = image.width() as u16;
        //             self.actual_height = image.height() as u16;
        //             self.options.data_code_type = DataCodeType::Raw;
        //             self.format = PictureGraphicFormat::EightBit;
        //             self.data = image
        //                 .to_rgb8()
        //                 .pixels()
        //                 .map(|pixel| {
        //                     let color = Colour::new_by_rgb(pixel[0], pixel[1], pixel[2]);
        //                     if let Some(index) = design.pool.color_to_index(color) {
        //                         index
        //                     } else {
        //                         0 // Default to black?
        //                     }
        //                 })
        //                 .collect();
        //         }
        //     }
        // } else if ui
        //     .button("Load Image")
        //     .on_hover_text("Load a new image")
        //     .clicked()
        // {
        //     let dialog = Arc::new(Mutex::new(FileDialog::open_file(None)));
        //     ui.close_menu();
        //     dialog.lock().unwrap().open();
        //     ui.data_mut(|data| {
        //         data.insert_temp(Id::new("file_dialog"), dialog);
        //     });
        // }

        ui.separator();
        ui.label("Macros:");
        render_macro_references(
            ui,
            design.get_pool(),
            &mut self.macro_refs,
            &Self::get_possible_events(),
            navigation_selected,
        );
    }
}

impl ConfigurableObject for ObjectPointer {
    fn render_parameters(
        &mut self,
        ui: &mut egui::Ui,
        design: &EditorProject,
        navigation_selected: &mut NullableObjectId,
    ) {
        render_object_id(ui, &mut self.id, design.get_pool(), navigation_selected);
        ui.horizontal(|ui| {
            ui.label("Object reference:");
            egui::ComboBox::from_id_source("object_reference")
                .selected_text(format!("{:?}", u16::from(self.value)))
                .width(0.0)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.selectable_value(&mut self.value, NullableObjectId::NULL, "None");
                    let object_types: Vec<ObjectType> = design
                        .get_pool()
                        .parent_objects(self.id)
                        .iter()
                        .flat_map(|parent_obj| {
                            get_allowed_child_refs(parent_obj.object_type(), VtVersion::Version3)
                                .into_iter()
                        })
                        .collect();
                    for potential_child in design.get_pool().objects_by_types(&object_types) {
                        ui.selectable_value(
                            &mut self.value,
                            potential_child.id().into(),
                            format!(
                                "{:?}: {:?}",
                                u16::from(potential_child.id()),
                                potential_child.object_type()
                            ),
                        );
                    }
                });
            if let Some(id) = self.value.into() {
                if let Some(object) = design.get_pool().object_by_id(id) {
                    if ui.link(format!("{:?}", object.object_type())).clicked() {
                        *navigation_selected = id.into();
                    }
                } else {
                    ui.colored_label(egui::Color32::RED, "Missing object in pool");
                }
            }
        });
    }
}
