use bevy::prelude::*;

#[allow(dead_code)]
pub fn setup_button(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn(ImageNode {
            rect: Some(Rect { ..default() }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(Button {
                ..Default::default()
            });
        })
        .with_children(|button| {
            button.spawn(Text::new("Button 1"));
        });
}

#[allow(dead_code)]
pub fn reactivity(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *color = BackgroundColor(Color::srgb(1.0, 1.0, 0.0)),
            Interaction::Hovered => *color = BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            Interaction::None => *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        }
    }
}
