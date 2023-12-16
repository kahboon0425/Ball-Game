// ECS = Entity Component System
// Entity = General object or item (e.g., a player, an enemy, a tree, a bullet...)
// Component = Data structures that contain information about aspects of an entity (e.g., a "Position" component might contain x,y,z coordinates)
// System = are where the logic and behavior of the game are implemented
//        = operate on entities that have specific components (e.g., a "MovementSystem" might update the position of all entities that have both a "Position" component and a "Velocity" component)

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // player sprite size
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;


// Commands: Used to create or modify entities in the game.
// Query: Used to access and modify components of entities.
// Transform: Represents the position, rotation, and scale of an entity.
// SpriteBundle, Camera2dBundle: Bundles of components for creating sprites and cameras.
// Vec3: A 3D vector used for positions and directions in the game world.
// KeyCode: Represents keyboard keys.
// Res and ResMut: Used to access shared resources in a read-only or mutable way.

fn main() {
    // Default plugins - allow us to get rendering Windows, UI, audio, and other funtionality
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(spawn_player, spawn_enemies, spawn_stars))
        .add_systems(Update,(spawn_camera, player_movement, confine_player_movement, enemy_movement, update_enemy_direction, confine_enemy_movement, enemy_hit_player, player_hit_star))
        // start the game loop
        .run();
}

// Component
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}
//Entity
// use "Commands" to spawn entities (creating a new object), despawn entities, add components to entities, remove components from entities 
// query window to get information about the width and height
// access to the asset server through a resource in order to load in our PNG file
// BEVY create an entity with a window and primary window component for us as well as a resource holding the asset server

// 1. create player entity
// 2. sets the player's position to the center od the game window
// 3. loads a sprite image for the player
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // a Resource<T> is a unique and globally accessible struct
    // Only one Resource of each type <T> can exist at an given time
    // We can use Resources in our systems as system parameters using Res<T> (Read-only), ResMut<T> (Mutable)
    // Res<T> and ResMut<T> are used to access shared resources in a read-only or mutable way.
    asset_server: Res<AssetServer>,
){
    // Getting a reference to our window query
    // The get_single() method 
    // only one entity will exist with both window component & primary window component
    // Since you are querying for the Window component with the PrimaryWindow marker, get_single() is used to get the primary game window.
    let window = window_query.get_single().unwrap();

    commands.spawn((
        // Bundles - can quickly add/remove sets of components to or from an entity
        SpriteBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}


// create a camera in the game, also centered in the window
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );
    };

}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{},
        ));
    }
}


// handles player's movement
pub fn player_movement(
    // resource to keyboard input
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    // get_single_mut give result type with Result<T,E>
    // if let is a syntax in Rust used for pattern matching. 
    // Ok(mut transform): This pattern matches if the result of player_query.get_single_mut() is Ok, meaning it successfully found the Transform component of the player entity.
    if let Ok(mut transform) = player_query.get_single_mut(){
        // Vec3 = 3-dimensional vector 
        // Vec3::ZERO =  (0, 0, 0) = no movement
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0,1.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0,-1.0,0.0);
        }

        // direction.normalize() normalizes the vector
        // Normalization is a process that adjusts the vector so that its length (magnitude) is exactly 1, but it keeps pointing in the same direction.
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // delta_seconds returns the time elapsed since the last frame/update in seconds.
        // Multiplying by time.delta_seconds() ensures that your movement is frame-rate independent. That means the entity will move at the same speed regardless of how fast the game loop is running.
        // "update the entity's position by moving it in the specified direction, at a certain speed, for the amount of time that has passed since the last frame." 
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// ensure the player doesnt move outside the game window
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();

        // The division by 2.0 is to get the radius (half the size) of the player if it's a square or circular sprite
        let half_player_size = PLAYER_SIZE / 2.0;
        // The leftmost point where the player can go without half of it going off-screen.
        let x_min = 0.0 + half_player_size;
        // The rightmost point where the player can go without going off the right edge of the screen.
        let x_max = window.width() - half_player_size;
        // The lowest point the player can go without going off the bottom edge of the screen.
        let y_min = 0.0 + half_player_size;
        // The highest point the player can go without going off the top edge of the screen
        let y_max = window.height() - half_player_size;

        // gets the current position of the player in the game window.
        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        // Bound the players y position
        if translation.y < y_min {
            translation.y = y_min;
        }else if translation.y > y_max {
            translation.y = y_max;
        }

        // applies the adjusted position back to the player
        player_transform.translation = translation;
    }

}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>){
        for (mut transform, enemy) in enemy_query.iter_mut() {
            let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
            transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        }
    }

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // audio: Res<Audio>,
    // asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();
    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut(){
        // let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            // direction_changed = true;

        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            // direction_changed = true;
        }

        // Play SFX
        // if direction_changed {
        //     // Play Sound Effect
        //     let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
        //     let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

        //     // Randomly play one of the two sound effects
        //     let sound_effect = if random::<f32>() > 0.5 {
        //         sound_effect_1
        //     } else {
        //         sound_effect_2
        //     };
            // audio.play(sound_effect);
        // }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query <&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut(){
        let mut translation = transform.translation;
        
        // Bound the enemy x position
        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        // Bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    // asset_server: Res<AssetServer>,
    // audio: Res<Audio>,
){
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut(){
        for enemy_transform in enemy_query.iter(){
            let distance: f32  = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game over!");
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    // asset_server: Res<AssetServer>,
){
    if let Ok(player_transform) = player_query.get_single(){
        for(star_entity, star_transform) in star_query.iter(){
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star!");
                commands.entity(star_entity).despawn();
            }
        }
    }
}

