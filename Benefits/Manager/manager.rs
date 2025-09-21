use crate::player::Player;

/// Sets the player's has_manager property to true if not already true.
pub fn grant_manager(player: &mut Player) {
    if !player.has_manager {
        player.has_manager = true;
    }
}
