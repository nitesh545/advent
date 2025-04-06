use bevy::prelude::*;

pub fn setup_button(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn(ImageBundle {
            node: Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                min_height: Val::Percent(60.0),
                min_width: Val::Percent(100.0),
                ..default()
            },
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

pub fn reactivity(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *color = BackgroundColor(Color::rgb(1.0, 1.0, 0.0)),
            Interaction::Hovered => *color = BackgroundColor(Color::rgb(1.0, 0.0, 0.0)),
            Interaction::None => *color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
        }
    }
}
