#![allow(unused)]

mod player;
mod enemy;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const TIME_STEP: f32 = 1. / 60.;

// region resources
struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
    enemy_materials: Handle<ColorMaterial>,
    explosion_materials: Handle<TextureAtlas>
}
struct WinSize {
    w: f32,
    h: f32
}
struct ActiveEnemies(u32);
// endregion resources

// region component
struct Player;
struct PlayerReadyFire(bool);
struct Laser;

struct Enemy;
struct Explosion;
struct ExoplosionToSpawn(Vec3);

struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self (500.)
    }
}
impl Default for PlayerReadyFire {
    fn default() -> Self {
        Self(true)
    }
}
// endregion component

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04,0.04)))
        .insert_resource(WindowDescriptor {
            title: "Shooter".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(laser_hit_enemy.system())
        .add_system(explosion_to_spawn.system())
        .add_system(animate_explosion.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>
) {
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());

    // create the main resources
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser_materials: materials.add(asset_server.load(LASER_SPRITE).into()),
        enemy_materials: materials.add(asset_server.load(ENEMY_SPRITE).into()),
        explosion_materials: texture_atlases.add(texture_atlas)
    });
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height()
    });

    // position window
    window.set_position(IVec2::new(0, 0));
}


fn laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>
) {
    for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
            let laser_scale = Vec2::from(laser_tf.scale);
            let enemy_scale = Vec2::from(enemy_tf.scale);
            let collision = collide(
                laser_tf.translation,
                laser_sprite.size *  laser_scale,
                enemy_tf.translation,
                enemy_sprite.size * enemy_scale
            );

            if let Some(_) = collision {
                // remove enemy
                commands
                    .entity(enemy_entity).despawn();
                active_enemies.0 -= 1;

                // remove laser
                commands
                    .entity(laser_entity).despawn();

                // spawn explosion
                commands
                    .spawn()
                    .insert(ExoplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn(
    mut commands: Commands,
    query: Query<(Entity, &ExoplosionToSpawn)>,
    materials: Res<Materials>
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: materials.explosion_materials.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(Timer::from_seconds(0.05, true));

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        With<Explosion>
    )>
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle, _) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;

            if sprite.index == texture_atlas.textures.len() as u32 {
                commands.entity(entity).despawn()
            }
        }
    }
}
