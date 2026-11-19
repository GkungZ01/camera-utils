// Injected hook adapter for Game::native_zoom
fn hook_native_zoom(zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    if let Some(desc) = crate::game::camera_utils::get_zoom_descriptor() {
        if zoom.is_none() {
            *zoom = Some(crate::game::native_hooks::NativeZoom {
                factor: desc.factor,
                smooth: desc.smooth,
                turn: desc.turn,
            });
        }
    }
}
