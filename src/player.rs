use bevy::prelude::*;
use crate::{Laser, Materials, Player, PlayerReadyFire, Speed, TIME_STEP, WinSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_stage(
                "game_setup_actors",
                SystemStage::single(
                    player_spawn.system()
                )
            )
            .add_system(player_movement.system())
            .add_system(player_fire.system())
            .add_system(laser_mouvement.system());
    }
}

fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>
) {
    // spawn a sprite
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyFire::default())
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&mut Transform, &mut PlayerReadyFire, With<Player>)>
) {
    if let Ok((mut transform, mut ready_fire, _)) = query.single_mut() {
        if ready_fire.0 && keyboard_input.pressed(KeyCode::Space) {

            let x = transform.translation.x;
            let y = transform.translation.y;

            let mut spawn_lasers = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.laser_materials.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y, 10.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(Speed::default());
            };

            let x_offset = 144. / 4. - 5.0;
            spawn_lasers(x_offset);
            spawn_lasers(-x_offset);

            ready_fire.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) {
            ready_fire.0 = true
        }
    }
}

fn laser_mouvement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>
) {
    for (laser_entity, speed, mut transform, _) in query.iter_mut() {
        // on fait avancer me laser
        transform.translation.y += speed.0 * TIME_STEP;

        if transform.translation.y > win_size.h {
            commands.entity(laser_entity).despawn();
        }
    }
}
