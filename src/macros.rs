#[macro_export]
macro_rules! rattle {
    ($ty:ident, $fn_name:ident, $width:literal, $interval_ms:literal, [$($frame:literal),+ $(,)?]) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $ty;

        impl $crate::Rattle for $ty {
            const SIZE: $crate::Size = $crate::Size::new($width, 1);
            const INTERVAL: std::time::Duration = std::time::Duration::from_millis($interval_ms);
            const FRAMES: &'static [&'static [&'static str]] = &[
                $(&[$frame]),+
            ];
        }

        pub fn $fn_name() -> $crate::Rattler<$ty> {
            $crate::Rattler::new()
        }
    };
}
