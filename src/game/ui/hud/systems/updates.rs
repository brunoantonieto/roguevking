use crate::game::monster::components::Monster;
use bevy::prelude::*;

use crate::game::score::resources::Score;
use crate::game::ui::hud::components::{MonsterText, ScoreText};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}

pub fn update_monster_text(
    mut text_query: Query<&mut Text, With<MonsterText>>,
    monster_query: Query<Entity, With<Monster>>,
) {
    let count = monster_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", count.to_string());
    }
}
