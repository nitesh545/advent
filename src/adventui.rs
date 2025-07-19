// bevy::prelude::*;
//
//#[allow(dead_code)]
//pub fn setup_button(mut commands: Commands, _asset_server: Res<AssetServer>) {
//    commands
//        .spawn(ImageNode {
//            rect: Some(Rect { ..default() }),
//            ..Default::default()
//        })
//        .with_children(|parent| {
//            parent.spawn(Button {
//                ..Default::default()
//            });
//        })
//        .with_children(|button| {
//            button.spawn(Text::new("Button 1"));
//        });
//}
//
//#[allow(dead_code)]
//pub fn reactivity(
//    mut interaction_query: Query<
//        (&Interaction, &mut BackgroundColor),
//        (Changed<Interaction>, With<Button>),
//    >,
//) {
//    for (interaction, mut color) in interaction_query.iter_mut() {
//        match *interaction {
//            Interaction::Pressed => *color = BackgroundColor(Color::srgb(1.0, 1.0, 0.0)),
//            Interaction::Hovered => *color = BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
//            Interaction::None => *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
//        }
//    }
//}

// use bevy::prelude::*;
//
// pub struct ProgressBarPlugin;
//
// impl Plugin for ProgressBarPlugin {
//     fn build(&self, app: &mut App) {
//         app.register_type::<ProgressBarValues>()
//             .add_systems(Update, progress_bar_system);
//     }
// }
//
// pub struct AddProgressBar {
//     pub background_image: Handle<Image>,
//     pub foreground_image: Handle<Image>,
//     pub right_to_left: bool,
//     pub values: ProgressBarValues,
//     pub entity: Entity,
// }
//
// impl Default for AddProgressBar {
//     fn default() -> Self {
//         Self {
//             background_image: Handle::default(),
//             foreground_image: Handle::default(),
//             right_to_left: false,
//             values: ProgressBarValues {
//                 min: 0.,
//                 max: 100.,
//                 step: 1.,
//                 value: 100.,
//             },
//             style: Default::default(),
//             entity: Entity::PLACEHOLDER,
//         }
//     }
// }
//
// impl Command for AddProgressBar {
//     fn apply(self, world: &mut World) {
//         let percent = self.values.as_percent();
//         let entity = if self.entity == Entity::PLACEHOLDER {
//             world.spawn_empty().id()
//         } else {
//             self.entity
//         };
//
//         world
//             .entity_mut(entity)
//             .insert((
//                 self.values,
//                 Node {
//                     ..default()
//                 },
//             ))
//             .with_children(|builder| {
//                 builder.spawn((
//                     Name::new("progress_bar_bg"),
//                     ImageNode {
//
//                         style: Style {
//                             width: Val::Percent(100.),
//                             height: Val::Percent(100.),
//                             ..default()
//                         },
//                         image: self.background_image.into(),
//                         ..default()
//                     },
//                 ));
//
//                 let right = if self.right_to_left {
//                     Val::Px(0.)
//                 } else {
//                     Val::Auto
//                 };
//
//                 let justify_content = if self.right_to_left {
//                     JustifyContent::FlexEnd
//                 } else {
//                     JustifyContent::FlexStart
//                 };
//
//                 builder
//                     .spawn((
//                         ProgressBarForeground,
//                         Name::new("progress_bar_fg"),
//                         Node {
//                             style: Style {
//                                 width: Val::Percent(percent),
//                                 height: Val::Percent(100.),
//                                 right,
//                                 justify_content,
//                                 position_type: PositionType::Absolute,
//                                 overflow: Overflow::clip(),
//                                 ..default()
//                             },
//                             ..default()
//                         },
//                     ))
//                     .with_children(|builder| {
//                         builder.spawn(ImageBundle {
//                             image: self.foreground_image.into(),
//                             ..default()
//                         });
//                     });
//             });
//     }
// }
//
// #[derive(Component)]
// pub struct ProgressBarForeground;
//
// #[derive(Component, Debug, Reflect)]
// pub struct ProgressBarValues {
//     pub min: f32,
//     pub max: f32,
//     pub step: f32,
//     value: f32,
// }
//
// impl ProgressBarValues {
//     pub fn as_percent(&self) -> f32 {
//         100. * ((self.value - self.min) / self.step).round() * self.step / (self.max - self.min)
//     }
//
//     pub fn value(&self) -> f32 {
//         self.value
//     }
//
//     pub fn set_value(&mut self, value: f32) {
//         self.value = ((value.clamp(self.min, self.max) - self.min) / self.step).round() * self.step
//             + self.min;
//     }
// }
//
// fn progress_bar_system(
//     mut query: Query<(&Children, &mut ProgressBarValues), Changed<ProgressBarValues>>,
//     mut foreground_query: Query<&mut Style, With<ProgressBarForeground>>,
// ) {
//     for (children, mut bar) in query.iter_mut() {
//         let value = bar.value();
//         bar.set_value(value);
//         let width: Val = Val::Percent(bar.as_percent());
//         for child in children.iter() {
//             if let Ok(mut style) = foreground_query.get_mut(*child) {
//                 style.width = width;
//             }
//         }
//     }
// }

use bevy::prelude::*;

#[allow(clippy::needless_update)]
pub fn _setup_progress_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            width: Val::Percent(50.),
            height: Val::Percent(50.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ImageNode {
                color: Color::srgb(255.0, 0.0, 0.0),
                image: asset_server.load("SpaceBackground5.png"),
                rect: Some(Rect::new(40., 40., 40., 40.)),
                ..Default::default()
            });
        })
        .with_children(|image| {
            image.spawn(Button {
                ..Default::default()
            });
        })
        .with_children(|button| {
            button.spawn(Text::new("Progress Bar"));
        });
    println!("progress bar added");
}
