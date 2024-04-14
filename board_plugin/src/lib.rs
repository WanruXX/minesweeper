mod bounds;
pub mod components;
mod events;
pub mod resources;
pub mod states;
mod systems;

#[cfg(feature = "inspect")]
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::{PrimaryWindow, Window};
use bevy_round_ui::prelude::RoundUiPlugin;

use board::Board;
use bounds::Bounds2;
use button_style::ButtonStyle;
use components::*;
use events::*;
use resources::*;
use states::AppState;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoundUiPlugin)
            .add_event::<TileTriggerEvent>()
            .add_event::<TileMarkEvent>()
            .init_resource::<ButtonStyle>()
            .insert_state(AppState::InGame)
            .add_systems(OnEnter(AppState::InGame), Self::create_board)
            .add_systems(
                Update,
                (
                    systems::input::handle_mouse_input,
                    systems::uncover::left_click_handler,
                    systems::uncover::uncover_tiles,
                    systems::mark::mark_tiles,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), systems::uncover::clear_tiles)
            .add_systems(
                OnEnter(AppState::Out),
                systems::exit_handler::setup_exit_window,
            )
            .add_systems(
                Update,
                (
                    systems::exit_handler::handle_button_interactions,
                    systems::exit_handler::handle_button_actions,
                )
                    .run_if(in_state(AppState::Out)),
            );
        #[cfg(feature = "inspect")]
        {
            app.register_type::<Coordinate>()
                .register_type::<Bomb>()
                .register_type::<BombNeighbor>()
                .register_type::<Uncover>();
        }
        #[cfg(feature = "inspect")]
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window, With<PrimaryWindow>>,
        board_assets: Res<BoardAssets>,
    ) {
        let window = windows
            .into_iter()
            .next()
            .expect("no primary window found!");
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let mut tile_map = tile_map::TileMap::create(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "inspect")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        #[cfg(feature = "inspect")]
        log::info!("board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        let board_entity = commands
            .spawn(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position.into()),
                ..Default::default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                parent
                    .spawn(Self::board_base_bundle(&board_assets, board_size))
                    .insert(Name::new("Background"));
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                );
            })
            .id();

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.truncate(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            marked_tiles: Vec::new(),
            entity: board_entity,
        });
    }

    fn adaptative_tile_size(
        window: &Window,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_heigth = window.height() / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }

    fn board_base_bundle(board_assets: &BoardAssets, board_size: Vec2) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: board_assets.board_material.color,
                custom_size: Some(board_size),
                ..Default::default()
            },
            texture: board_assets.board_material.texture.clone(),
            transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
            ..Default::default()
        }
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &tile_map::TileMap,
        tile_size: f32,
        padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinate, Entity>,
    ) {
        let tile_size_nopadding = tile_size - padding;
        let tile_size_nopadding_vec2 = Some(Vec2::splat(tile_size_nopadding));
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinate = Coordinate {
                    x: x as u16,
                    y: y as u16,
                };

                let mut cmd = parent.spawn(Self::tile_bundle(
                    board_assets,
                    tile_size_nopadding_vec2,
                    tile_size,
                    coordinate,
                ));

                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coordinate)
                    .with_children(|parent| {
                        let entity = parent
                            .spawn(Self::tile_cover_bundle(
                                board_assets,
                                tile_size_nopadding_vec2,
                            ))
                            .insert(Name::new("Tile Cover"))
                            .id();
                        covered_tiles.insert(coordinate, entity);
                    });

                match tile {
                    crate::resources::tile::Tile::Bomb => {
                        cmd.insert(Bomb).with_children(|parent| {
                            parent.spawn(Self::bomb_bundle(board_assets, tile_size_nopadding_vec2));
                        });
                    }
                    crate::resources::tile::Tile::BombNeighbor(count) => {
                        let bomb_neighbor = BombNeighbor { count: *count };
                        cmd.insert(bomb_neighbor).with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                board_assets,
                                *count,
                                tile_size_nopadding,
                            ));
                        });
                    }
                    _ => (),
                };
            }
        }
    }

    fn tile_bundle(
        board_assets: &BoardAssets,
        size: Option<Vec2>,
        tile_size: f32,
        coord: Coordinate,
    ) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size: size,
                color: board_assets.tile_material.color,
                ..Default::default()
            },
            texture: board_assets.tile_material.texture.clone(),
            transform: Transform::from_xyz(
                (coord.x as f32 * tile_size) + (tile_size / 2.),
                (coord.y as f32 * tile_size) + (tile_size / 2.),
                1.,
            ),
            ..Default::default()
        }
    }

    fn tile_cover_bundle(board_assets: &BoardAssets, size: Option<Vec2>) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size: size,
                color: board_assets.covered_tile_material.color,
                ..Default::default()
            },
            texture: board_assets.covered_tile_material.texture.clone(),
            transform: Transform::from_xyz(0., 0., 2.),
            ..Default::default()
        }
    }

    fn bomb_bundle(board_assets: &BoardAssets, size: Option<Vec2>) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: board_assets.bomb_material.color,
                custom_size: size,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            texture: board_assets.bomb_material.texture.clone(),
            ..Default::default()
        }
    }

    fn bomb_count_text_bundle(board_assets: &BoardAssets, count: u8, size: f32) -> Text2dBundle {
        let color = board_assets.bomb_counter_color(count);
        Text2dBundle {
            text: Text::from_sections(vec![TextSection {
                value: count.to_string(),
                style: TextStyle {
                    color,
                    font: board_assets.bomb_counter_font.clone(),
                    font_size: size,
                },
            }])
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }
}
