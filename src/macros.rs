#[macro_export]
macro_rules! rattle {
    ($ty:ident, $fn_name:ident, ($width:literal, $height:literal), $interval_ms:literal, [$($frame:literal),+ $(,)?]) => {
        $crate::rattle!(@impl $ty, $fn_name, $width, $height, $interval_ms, [$($frame),+]);
    };

    ($ty:ident, $fn_name:ident, $width:literal, $interval_ms:literal, [$($frame:literal),+ $(,)?]) => {
        $crate::rattle!(@impl $ty, $fn_name, $width, 1, $interval_ms, [$($frame),+]);
    };

    (@impl $ty:ident, $fn_name:ident, $width:literal, $height:literal, $interval_ms:literal, [$($frame:literal),+]) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $ty;

        impl $crate::Rattle for $ty {
            const SIZE: $crate::Size = $crate::Size::new($width, $height);
            const INTERVAL: core::time::Duration =
                core::time::Duration::from_millis($interval_ms);
            const FRAMES: &'static [&'static [&'static str]] = &[
                $(&[$frame]),+
            ];
        }

        pub fn $fn_name() -> $crate::Rattler<$ty> {
            $crate::Rattler::new()
        }
    };
}
