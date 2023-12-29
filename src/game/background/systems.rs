use bevy::prelude::*;

use bevy::window::PrimaryWindow;

use super::components::*;


pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/World_A22.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(45.0, 45.0),
        1,
        1,
        None,
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let window =window_query.get_single().unwrap();
    let tile_size = 45.0;
    let rows = (window.height() / tile_size) as i32;
    let cols = (window.width() / tile_size) as i32;

    for row in 0..rows+1 {
        for col in 0..cols+1 {
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_xyz(
                        col as f32 * tile_size,
                        row as f32 * tile_size,
                        -0.5,
                    ),
                    ..default()
                })
                .insert(Background {});
        }
    }


}
