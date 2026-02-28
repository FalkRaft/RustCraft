use bevy::{prelude::*, window::PrimaryWindow};

pub struct BevyWindowPlugin;

impl Default for BevyWindowPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for BevyWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            |mut window_query: Query<&mut Window, With<PrimaryWindow>>| {
                if let Ok(mut window) = window_query.single_mut() {
                    // Set minimum window size
                    window.resize_constraints.min_width = 800.0;
                    window.resize_constraints.min_height = 600.0;
                }
            },
        );
    }
}
