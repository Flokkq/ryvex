#[macro_export]
macro_rules! define_keymaps {
    (
        $( $mode:ident { $( $seq:literal => $cmd:expr ),* $(,)? } )*
    ) => {
        pub struct KeyMaps {
            $( pub $mode: $crate::keymap::KeyNode, )*
        }

        impl KeyMaps {
            pub fn new() -> Self {
                let mut km = Self {
                    $( $mode: $crate::keymap::KeyNode::default(), )*
                };

                $(
                    {
                        let root = &mut km.$mode;
                        $(
                            let normalized: String = ::ryvex_target::key::AsciiKeyCode::parse_human_str($seq)
                                .unwrap_or_else(|e| {
                                    panic!("Error parsing key‐sequence literal `{}`: {}", $seq, e)
                                })
                                .into_iter()
                                .map(|kc| kc.to_char())
                                .collect();

                            root.bind_str(&normalized, $cmd);
                        )*
                    }
                )*

                km
            }
        }
    };
}
