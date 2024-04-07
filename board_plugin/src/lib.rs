pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use components::*;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::BoardPosition;
use resources::TileSize;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        #[cfg(feature = "inspect")]
        {
            app.register_type::<Coordinate>()
                .register_type::<Bomb>()
                .register_type::<BombNeighbor>()
                .register_type::<Uncover>();
        }
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    fn adaptative_tile_size(
        window: &Window,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_heigth = window.height() / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }

    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        let window = windows
            .into_iter()
            .next()
            .expect("no primary window found!");
        let options = match board_options {
            None => BoardOptions::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };
        // Tilemap generation
        let mut tile_map = TileMap::create(options.map_size.0, options.map_size.1);
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
        log::info!("board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        commands
            .spawn(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position.into()),
                ..Default::default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                let bomb_image = asset_server.load("sprites/bomb.png");
                let font = asset_server.load("fonts/pixeled.ttf");
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    bomb_image,
                    font,
                );
            });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        padding: f32,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
    ) {
        let tile_size_nopadding = tile_size - padding;
        let tile_size_nopadding_vec2 = Some(Vec2::splat(tile_size_nopadding));
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let tile_bundle = SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: tile_size_nopadding_vec2,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
                    ),
                    ..Default::default()
                };

                let mut cmd = parent.spawn(tile_bundle);

                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(Coordinate {
                        x: x as u16,
                        y: y as u16,
                    });

                match tile {
                    crate::resources::tile::Tile::Bomb => {
                        cmd.insert(Bomb).with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: tile_size_nopadding_vec2,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    crate::resources::tile::Tile::BombNeighbor(count) => {
                        let bomb_neighbor = BombNeighbor { count: *count };
                        cmd.insert(bomb_neighbor).with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *count,
                                font.clone(),
                                tile_size_nopadding,
                            ));
                        });
                    }
                    _ => (),
                };
            }
        }
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );
        Text2dBundle {
            text: Text::from_sections(vec![TextSection {
                value: text,
                style: TextStyle {
                    color,
                    font,
                    font_size: size,
                },
            }])
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }
}
