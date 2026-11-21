fn turn(xo: f64, yo: f64) -> ::core::ops::ControlFlow<()> {
    if camera_utils::turn(xo, yo) {
        ::core::ops::ControlFlow::Break(())
    } else {
        ::core::ops::ControlFlow::Continue(())
    }
}
