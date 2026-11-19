// Injected hook adapter for Game::native_input_key
fn hook_native_input_key(
    game: &crate::game::Game<'_>,
    key: &str,
    action: mod_api::KeyAction,
) {
    let pressed = match action {
        mod_api::KeyAction::Press => true,
        mod_api::KeyAction::Release => false,
        _ => return, // Ignore repeat events
    };

    let (pos, yaw, pitch) = game.player.as_ref().map_or(([0.0; 3], 0.0, 0.0), |p| {
        ([p.position.x, p.position.y, p.position.z], game.yaw, game.pitch)
    });
    crate::game::camera_utils::on_native_key(key, pressed, pos, yaw, pitch);
}
