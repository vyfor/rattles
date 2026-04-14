#[macro_export]
macro_rules! rattle {
    ($ty:ident, $fn_name:ident, ($width:literal, $height:literal), $interval_ms:literal, [$($frame:literal),+ $(,)?]) => {
        $crate::rattle!(@impl $ty, $fn_name, $width, $height, $interval_ms, @inline [$($frame),+]);
    };

    ($ty:ident, $fn_name:ident, $width:literal, $interval_ms:literal, [$($frame:literal),+ $(,)?]) => {
        $crate::rattle!(@impl $ty, $fn_name, $width, 1, $interval_ms, @inline [$($frame),+]);
    };

    ($ty:ident, $fn_name:ident, ($width:literal, $height:literal), $interval_ms:literal, $frames:path) => {
        $crate::rattle!(@impl $ty, $fn_name, $width, $height, $interval_ms, @path $frames);
    };

    ($ty:ident, $fn_name:ident, $width:literal, $interval_ms:literal, $frames:path) => {
        $crate::rattle!(@impl $ty, $fn_name, $width, 1, $interval_ms, @path $frames);
    };

    (@impl $ty:ident, $fn_name:ident, $width:literal, $height:literal, $interval_ms:literal, @inline [$($frame:literal),+]) => {
        $crate::rattle!(@emit $ty, $fn_name, $width, $height, $interval_ms, &[$(&[$frame]),+], [$($frame),+]);
    };

    (@impl $ty:ident, $fn_name:ident, $width:literal, $height:literal, $interval_ms:literal, @path $frames:path) => {
        $crate::rattle!(@emit $ty, $fn_name, $width, $height, $interval_ms, $frames, $frames);
    };

    (@emit $ty:ident, $fn_name:ident, $width:literal, $height:literal, $interval_ms:literal, $frames:expr, $docs:tt) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $ty;

        impl $crate::Rattle for $ty {
            const SIZE: $crate::Size = $crate::Size::new($width, $height);
            const INTERVAL: core::time::Duration =
                core::time::Duration::from_millis($interval_ms);
            const FRAMES: &'static [&'static [&'static str]] = $frames;
        }

        #[doc = concat!("```rust\n", stringify!($docs), "\n```")]
        pub fn $fn_name() -> $crate::Rattler<$ty> {
            $crate::Rattler::new()
        }
    };
}
