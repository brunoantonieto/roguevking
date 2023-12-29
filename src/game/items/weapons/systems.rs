use crate::game::player::components::Player;
// use crate::game::items::weapons::components::Weapon;
use crate::game::items::weapons::sword::components::Sword;

pub fn attack(    
    player: &mut Player,
    // mut weapon: &mut Weapon
    sword: &mut Sword,
) {
    if sword.progress == 0.0 { // Start the attack if not already attacking
        sword.progress = 0.01; // Small non-zero value to signify the start of an attack
        player.is_attacking = true;
    }
}