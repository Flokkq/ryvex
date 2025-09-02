#[macro_export]
macro_rules! define_keymaps {
    (
        $(
            $mode:ident { $($mode_body:tt)* }
        )*
    ) => {
        pub struct KeyMaps {
            $( pub $mode: $crate::keymap::KeyNode, )*
        }

        impl Default for KeyMaps {
            fn default() -> Self { Self::new() }
        }

        impl KeyMaps {
            pub fn new() -> Self {
                let mut km = Self {
                    $( $mode: $crate::keymap::KeyNode::default(), )*
                };

                $(
                    {
                        let root = &mut km.$mode;
                        let mut __prefix = String::new();

                        fn __normalize(s: &str) -> String {
                            ::ryvex_target::key::AsciiKeyCode::parse_human_str(s)
                                .unwrap_or_else(|e| panic!("Error parsing key-sequence literal `{}`: {}", s, e))
                                .into_iter()
                                .map(|kc| kc.to_char())
                                .collect()
                        }

                        fn __extend(prefix: &str, seg: &str) -> String {
                            let mut out = String::with_capacity(prefix.len() + seg.len());
                            out.push_str(prefix);
                            out.push_str(seg);
                            out
                        }

                        define_keymaps!(@entries root, __prefix, __normalize, __extend, $($mode_body)*);
                    }
                )*

                km
            }
        }
    };

    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,) => {};
    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path) => {};
    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path, , $($rest:tt)*) => {
        define_keymaps!(@entries $root, $prefix, $normalize, $extend, $($rest)*);
    };

    // Nested first
    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,
        $seq:literal => { $($child:tt)* }, $($rest:tt)*
    ) => {
        {
            let __seg  = $normalize($seq);
            let __next = $extend(&$prefix, &__seg);
            define_keymaps!(@entries $root, __next, $normalize, $extend, $($child)*);
        }
        define_keymaps!(@entries $root, $prefix, $normalize, $extend, $($rest)*);
    };

    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,
        $seq:literal => { $($child:tt)* } $(,)?
    ) => {
        let __seg  = $normalize($seq);
        let __next = $extend(&$prefix, &__seg);
        define_keymaps!(@entries $root, __next, $normalize, $extend, $($child)*);
    };

    // Leaves
    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,
        $seq:literal => $cmd:expr, $($rest:tt)*
    ) => {
        {
            let __seg  = $normalize($seq);
            let __full = $extend(&$prefix, &__seg);
            $root.bind_str(&__full, $cmd);
        }
        define_keymaps!(@entries $root, $prefix, $normalize, $extend, $($rest)*);
    };

    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,
        $seq:literal => $cmd:expr $(,)?
    ) => {
        let __seg  = $normalize($seq);
        let __full = $extend(&$prefix, &__seg);
        $root.bind_str(&__full, $cmd);
    };

    // Fallbacks
    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,
        _ => $cmd:expr, $($rest:tt)*
    ) => {
        $root.bind_fallback_at(&$prefix, $cmd);
        define_keymaps!(@entries $root, $prefix, $normalize, $extend, $($rest)*);
    };

    (@entries $root:ident, $prefix:ident, $normalize:path, $extend:path,
        _ => $cmd:expr $(,)?
    ) => {
        $root.bind_fallback_at(&$prefix, $cmd);
    };
}
