use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn despawn_recursive<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
