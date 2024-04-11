use crate::button_style::ButtonStyle;
use crate::AppState;
use crate::ExitWindow;
use crate::RoundButton;
use crate::Board;
use bevy::{app::AppExit, prelude::*};
use bevy_round_ui::{autosize::*, prelude::*};

use bevy::input::ButtonInput;

#[derive(Component, Debug)]
pub enum ButtonAction {
    Play,
    Quit,
}

fn spawn_button(
    parent: &mut ChildBuilder,
    button_style: &Res<ButtonStyle>,
    text: impl Into<String>,
    extras: impl Bundle,
) -> Entity {
    parent
        .spawn((
            RoundButton,
            RoundUiAutosizeNode,
            RoundUiAutosizeNodePadding,
            MaterialNodeBundle {
                material: button_style.default.clone(),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(button_style.width),
                    height: Val::Px(button_style.height),
                    margin: UiRect::bottom(Val::Px(10.)),
                    ..default()
                },
                ..default()
            },
            extras,
            Interaction::default(),
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 20.,
                    ..default()
                },
            ));
        })
        .id()
}

pub fn setup_exit_window(
    mut commands: Commands,
    button_style: Res<ButtonStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
) {
    // Define a material for the panel.
    // This material looks like it has a border, because we applied an equal offset to all sides.
    let panel_width = 300.0;
    let panel_height = 250.0;
    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("5cb3af").unwrap(),
        border_color: Color::WHITE,
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
        offset: RoundUiOffset::all(6.0).into(),
    });

    // Spawn the screen layout, containing a centered panel with menu items
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(MaterialNodeBundle {
                material: panel_material,
                style: Style {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                // Spawn the title
                p.spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(30.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "OVER",
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 40.,
                            ..default()
                        },
                    ));
                });

                // Spawn the buttons
                spawn_button(p, &button_style, "New Game", ButtonAction::Play);
                spawn_button(p, &button_style, "Quit", ButtonAction::Quit);
            });
        })
        .insert(ExitWindow);
}

/// Updates button materials when their interaction changes
#[allow(clippy::type_complexity)]
pub fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut Handle<RoundUiMaterial>),
        (Changed<Interaction>, With<RoundButton>),
    >,
    button_style: Res<ButtonStyle>,
) {
    for (interaction, mut material) in &mut interaction_query {
        *material = match *interaction {
            Interaction::Pressed => button_style.press.clone(),
            Interaction::Hovered => button_style.hover.clone(),
            Interaction::None => button_style.default.clone(),
        };
    }
}

/// Handle button click events
pub fn handle_button_actions(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
    exit_window: Query<Entity, With<ExitWindow>>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Button pressed: {action:?}");
            match action {
                ButtonAction::Play => {
                    commands
                        .entity(exit_window.iter().next().unwrap())
                        .despawn_recursive();
                    commands.entity(board.entity).despawn_recursive();
                    commands.remove_resource::<Board>();
                    mouse_input.clear();
                    next_state.set(AppState::InGame);
                    
                }
                ButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}
