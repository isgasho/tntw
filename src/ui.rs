//! Systems and structs for the users interface

use bevy::{prelude::*, render::pass::ClearColor};
use bevy_input::keyboard::*;
use bevy_input::mouse::*;

use crate::{Unit, UnitState};


pub const ICON_SCALE: f32 = 1.2;

pub struct SelectionMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub selected: Handle<ColorMaterial>,
}

pub struct UiStateMaterials {
    pub idle: Handle<ColorMaterial>,
    pub moving: Handle<ColorMaterial>,
    pub moving_fast: Handle<ColorMaterial>,
}

pub fn unit_display_system(
    selection_materials: Res<SelectionMaterials>,
    icon_materials: Res<UiStateMaterials>,
    mut unit_query: Query<(&Unit, &mut Handle<ColorMaterial>, &Children)>,
    icon_query: Query<&mut Handle<ColorMaterial>>,
) {
    for (unit, mut material, children) in &mut unit_query.iter() {
        let mut state_icon = icon_query.get_mut::<Handle<ColorMaterial>>(children[0]).unwrap();
        *state_icon = match unit.state() {
            UnitState::MovingSlow => icon_materials.moving,
            UnitState::MovingFast => icon_materials.moving_fast,
            _ => icon_materials.idle,
        };
        *material = if unit.is_selected() {
            selection_materials.selected
        } else {
            selection_materials.normal
        };
    }
}

impl FromResources for SelectionMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().expect("Colour resource");
        SelectionMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            selected: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}