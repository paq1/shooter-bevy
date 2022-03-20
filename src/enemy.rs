use bevy::prelude::*;
use bevy::core::FixedTimestep;
use rand::{Rng, thread_rng};
use crate::{ActiveEnemies, AppBuilder, Enemy, Materials, Plugin, SystemSet, WinSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn.system())
        );
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemy: ResMut<ActiveEnemies>,
    win_size: Res<WinSize>,
    materials: Res<Materials>
) {
    if active_enemy.0 < 1 {
        // compute random position
        let mut rng = thread_rng();
        let w_spawn = win_size.w / 2. - 100.;
        let h_spawn = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_spawn..w_spawn) as f32;
        let y = rng.gen_range(-h_spawn..h_spawn) as f32;

        // spawn enemy
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy);

        active_enemy.0 += 1;
    }
}