use bevy::{diagnostic::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(crate) struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EntityCountDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    }
}
