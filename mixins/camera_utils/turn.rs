// Injected hook adapter for Game::turn_player
// Intercept mouse turns when Freecam or Freelock is active
fn hook_turn_player(game: &mut crate::game::Game<'_>, xo: f64, yo: f64) {
    if crate::game::camera_utils::is_freelock_or_freecam() {
        crate::game::camera_utils::on_turn_input(xo, yo);
    }
}
