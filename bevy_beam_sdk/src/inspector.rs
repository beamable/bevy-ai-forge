use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext, EguiContexts};
use bevy_inspector_egui::bevy_inspector::Filter;
use bevy_inspector_egui::egui::RichText;

use crate::config::BeamableConfigResource;
use crate::slot::prelude::BeamSlot;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_egui);
        app.add_systems(
            Update,
            bevy_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        );
    }
}

fn bevy_inspector(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();
    egui::SidePanel::right("Bevy State")
        .frame(
            egui::Frame::canvas(&egui_context.get_mut().style())
                .inner_margin(egui::Margin::same(10.0)),
        )
        .show_animated(egui_context.get_mut(), true, |ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new("Beamable")
                    .color(egui::Color32::from_rgb(32, 148, 243))
                    .strong()
                    .size(25.0),
            );
            ui.add_space(10.0);
            ui.label("BeamableSDK Status");
            ui.add_enabled_ui(false, |ui|
            bevy_inspector_egui::bevy_inspector::ui_for_state::<crate::state::BeamableInitStatus>(
                world, ui,
            ));
            ui.separator();
            ui.collapsing("Configuration", |ui| {
                ui.vertical_centered_justified(|ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<BeamableConfigResource>(
                        world, ui,
                    );
                });
            });
            ui.separator();
            egui::ScrollArea::both().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_entities_filtered(
                        world,
                        ui,
                        false,
                        &Filter::<With<BeamSlot>>::all(),
                    );
                });
            });
        });
}

fn configure_egui(mut contexts: EguiContexts) {
    #[cfg(windows)]
    {
        if let Some((regular, semibold)) = get_fonts() {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "regular".to_owned(),
                egui::FontData::from_owned(regular).into(),
            );
            fonts.font_data.insert(
                "semibold".to_owned(),
                egui::FontData::from_owned(semibold).into(),
            );

            // Put my font first (highest priority) for proportional text:
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "regular".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Name("semibold".into()))
                .or_default()
                .insert(0, "semibold".to_owned());

            // Put my font as last fallback for monospace:
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("regular".to_owned());

            // Tell egui to use these fonts:
            contexts.ctx_mut().set_fonts(fonts);
        }
    }
    contexts.ctx_mut().style_mut(|style| {
        for font_id in style.text_styles.values_mut() {
            font_id.size *= 1.2;
        }
    });
}

#[cfg(windows)]
fn get_fonts() -> Option<(Vec<u8>, Vec<u8>)> {
    use std::fs;

    let app_data = std::env::var("APPDATA").ok()?;
    let font_path = std::path::Path::new(&app_data);

    let regular = fs::read(font_path.join("../Local/Microsoft/Windows/Fonts/aptos.ttf")).ok()?;
    let semibold =
        fs::read(font_path.join("../Local/Microsoft/Windows/Fonts/aptos-bold.ttf")).ok()?;

    Some((regular, semibold))
}
