use crate::game::actor::player::components::Player;
use crate::game::items::weapons::components::Weapon;

pub fn attack(    
    player: &mut Player,
    weapon: &mut Weapon,
) {
    if weapon.progress == 0.0 { // Start the attack if not already attacking
        weapon.progress = 0.01; // Small non-zero value to signify the start of an attack
        player.is_attacking = true;
    }
}