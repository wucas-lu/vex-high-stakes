//! Exports useful macros for the robot.

/**
    Sets the given motors' velocities to the specified velocity in RPM.
*/
#[macro_export]
macro_rules! set_velocities {
    ($v:expr, $( $m:expr ),* $(,)?) => {{
        $(
            $m.set_velocity($v)?;
        )*
    }};
}
