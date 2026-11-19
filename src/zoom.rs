// Injected hook adapter for Game::native_zoom
fn hook_native_zoom(zoom: &mut Option<mod_api::Zoom>) {
    if let Some(desc) = crate::game::camera_utils::get_zoom_descriptor() {
        if zoom.is_none() {
            *zoom = Some(mod_api::Zoom {
                factor: desc.factor,
                smooth: desc.smooth,
                turn: desc.turn,
            });
        }
    }
}
