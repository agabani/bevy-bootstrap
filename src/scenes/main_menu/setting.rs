use bevy::prelude::*;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(super) struct Screen;

pub(super) fn template(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        Name::new("Settings"),
        super::Screen,
        Screen,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::BLUE.into(),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
    ));
}
