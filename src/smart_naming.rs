//! Copyright 2024 - The Open-Agriculture Developers
//! SPDX-License-Identifier: GPL-3.0-or-later
//! Authors: Daan Steenbergen

use ag_iso_stack::object_pool::{object::Object, ObjectPool, ObjectType};
use std::collections::HashMap;

/// Get a user-friendly name for an object type
pub fn get_object_type_name(object_type: ObjectType) -> &'static str {
    match object_type {
        ObjectType::WorkingSet => "Working Set",
        ObjectType::DataMask => "Data Mask",
        ObjectType::AlarmMask => "Alarm Screen",
        ObjectType::Container => "Container",
        ObjectType::SoftKeyMask => "Soft Key Mask",
        ObjectType::Key => "Key",
        ObjectType::Button => "Button",
        ObjectType::InputBoolean => "Checkbox",
        ObjectType::InputString => "Text Input",
        ObjectType::InputNumber => "Number Input",
        ObjectType::InputList => "List Input",
        ObjectType::OutputString => "Text Display",
        ObjectType::OutputNumber => "Number Display",
        ObjectType::OutputList => "List Display",
        ObjectType::OutputLine => "Line",
        ObjectType::OutputRectangle => "Rectangle",
        ObjectType::OutputEllipse => "Ellipse",
        ObjectType::OutputPolygon => "Polygon",
        ObjectType::OutputMeter => "Meter",
        ObjectType::OutputLinearBarGraph => "Linear Bar",
        ObjectType::OutputArchedBarGraph => "Arched Bar",
        ObjectType::PictureGraphic => "Picture",
        ObjectType::NumberVariable => "Number Variable",
        ObjectType::StringVariable => "String Variable",
        ObjectType::FontAttributes => "Font Style",
        ObjectType::LineAttributes => "Line Style",
        ObjectType::FillAttributes => "Fill Style",
        ObjectType::InputAttributes => "Input Style",
        ObjectType::ObjectPointer => "Object Reference",
        ObjectType::Macro => "Macro",
        ObjectType::AuxiliaryFunctionType1 => "Aux Function v1",
        ObjectType::AuxiliaryInputType1 => "Aux Input v1",
        ObjectType::AuxiliaryFunctionType2 => "Aux Function v2",
        ObjectType::AuxiliaryInputType2 => "Aux Input v2",
        ObjectType::AuxiliaryControlDesignatorType2 => "Aux Control v2",
        ObjectType::ColourMap => "Colour Map",
        ObjectType::GraphicsContext => "Graphics Context",
        ObjectType::ColourPalette => "Colour Palette",
        ObjectType::GraphicData => "Graphic Data",
        ObjectType::WorkingSetSpecialControls => "Special Controls",
        ObjectType::ScaledGraphic => "Scaled Graphic",
        ObjectType::WindowMask => "Window Mask",
        ObjectType::KeyGroup => "Key Group",
        ObjectType::ExtendedInputAttributes => "Extended Input Style",
        ObjectType::ObjectLabelReferenceList => "Label Reference List",
        ObjectType::ExternalObjectDefinition => "External Object Definition",
        ObjectType::ExternalReferenceName => "External Reference Name",
        ObjectType::ExternalObjectPointer => "External Object Pointer",
        ObjectType::Animation => "Animation",
    }
}

/// Generates a smart default name for an object based on its type and context
pub fn generate_smart_default_name(
    object_type: ObjectType,
    pool: &ObjectPool,
    existing_names: &HashMap<String, usize>,
) -> String {
    // Count existing objects of the same type
    let same_type_count = pool
        .objects()
        .iter()
        .filter(|obj| obj.object_type() == object_type)
        .count();

    // Generate base name based on object type
    let base_name = match object_type {
        ObjectType::DataMask => {
            if same_type_count == 0 {
                "Main Screen"
            } else {
                "Data Screen"
            }
        }
        _ => get_object_type_name(object_type),
    };

    // If this is the first of its type and has a special name, use it
    if same_type_count == 0 && !base_name.contains("Screen") {
        return base_name.to_string();
    }

    // Check if the base name already exists
    if existing_names.get(base_name).copied().unwrap_or(0) == 0 && same_type_count == 0 {
        return base_name.to_string();
    }

    // Generate numbered name
    let mut counter = same_type_count + 1;
    loop {
        let candidate = format!("{} {}", base_name, counter);
        if existing_names.get(&candidate).copied().unwrap_or(0) == 0 {
            return candidate;
        }
        counter += 1;
    }
}

/// Generates contextual names for specific object types based on their properties
pub fn generate_contextual_name(object: &Object, pool: &ObjectPool) -> Option<String> {
    match object {
        Object::Key(key) => {
            // Name keys based on their key code
            match key.key_code {
                0 => Some("ACK/Enter Key".to_string()),
                1 => Some("ESC Key".to_string()),
                2..=7 => Some(format!("Soft Key {}", key.key_code - 1)),
                _ => None,
            }
        }
        Object::Button(button) => {
            // Try to name buttons based on their key code
            match button.key_code {
                0 => Some("OK Button".to_string()),
                1 => Some("Cancel Button".to_string()),
                _ => None,
            }
        }
        Object::Container(container) => {
            // Name containers based on their size
            if container.height < 100 {
                Some("Header Container".to_string())
            } else if container.height > 300 {
                Some("Main Container".to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Suggests a name for an object when it's being created, considering its parent context
pub fn suggest_name_for_child(
    parent_object: &Object,
    child_type: ObjectType,
    pool: &ObjectPool,
) -> Option<String> {
    match (parent_object.object_type(), child_type) {
        (ObjectType::SoftKeyMask, ObjectType::Key) => {
            // Count existing keys in this soft key mask
            let key_count = parent_object
                .referenced_objects()
                .iter()
                .filter_map(|id| pool.object_by_id(*id))
                .filter(|obj| matches!(obj, Object::Key(_)))
                .count();
            Some(format!("F{} Key", key_count + 1))
        }
        (ObjectType::Container, ObjectType::Button) => Some("Container Button".to_string()),
        (ObjectType::Container, ObjectType::OutputString) => Some("Container Label".to_string()),
        (ObjectType::DataMask, ObjectType::Container) => {
            // Suggest container names based on position in data mask
            let container_count = parent_object
                .referenced_objects()
                .iter()
                .filter_map(|id| pool.object_by_id(*id))
                .filter(|obj| matches!(obj, Object::Container(_)))
                .count();
            
            match container_count {
                0 => Some("Header Container".to_string()),
                1 => Some("Main Container".to_string()),
                2 => Some("Footer Container".to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Validates a name and suggests corrections if needed
pub fn validate_and_suggest_name(name: &str, existing_names: &HashMap<String, usize>) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    
    if name.len() > 100 {
        return Err("Name is too long (max 100 characters)".to_string());
    }
    
    if existing_names.get(name).copied().unwrap_or(0) > 0 {
        // Suggest an alternative
        const MAX_OBJECTS: u32 = 65535; // ISOBUS maximum object count (16-bit IDs)
        let mut counter = 2;
        while counter <= MAX_OBJECTS {
            let suggestion = format!("{} {}", name, counter);
            if existing_names.get(&suggestion).copied().unwrap_or(0) == 0 {
                return Err(format!("Name '{}' already exists. Try '{}'", name, suggestion));
            }
            counter += 1;
        }
        return Err(format!("Name '{}' already exists and all numbered variations up to {} are taken", name, MAX_OBJECTS));
    }
    
    Ok(())
}