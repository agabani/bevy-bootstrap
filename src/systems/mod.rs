use bevy::{app::AppExit, prelude::*};

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn despawn_recursive<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn on_pressed_change_visibility<I: Component, V: Component>(
    visibility: Visibility,
) -> impl Fn(Query<&mut Visibility, With<V>>, Query<&Interaction, (Changed<Interaction>, With<I>)>)
{
    move |mut visibilities, interactions| {
        for &i in &interactions {
            if i == Interaction::Pressed {
                for mut v in &mut visibilities {
                    *v = visibility;
                }
            }
        }
    }
}

pub(crate) fn on_pressed_despawn_descendants<I: Component, E: Component>(
    mut commands: Commands,
    interactions: Query<&Interaction, (Changed<Interaction>, With<I>)>,
    entities: Query<Entity, With<E>>,
) {
    for &interaction in &interactions {
        if interaction == Interaction::Pressed {
            for entity in &entities {
                commands.entity(entity).despawn_descendants();
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn on_pressed_exit_app<I: Component>(
    mut events: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<I>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            events.send(AppExit);
        }
    }
}
