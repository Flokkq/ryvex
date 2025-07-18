#[macro_export]
macro_rules! define_keymaps {
    (
        $( $mode:ident { $( $seq:literal => $cmd:expr ),* $(,)? } )*
    ) => {
        pub struct KeyMaps { $( pub $mode: $crate::keymap::KeyNode, )* }

        impl KeyMaps {
            pub fn new() -> Self {
                let mut km = Self {
                    $( $mode: $crate::keymap::KeyNode::default(), )*
                };
                $(
                    {
                        let root = &mut km.$mode;
                        $( root.bind_str($seq, $cmd); )*
                    }
                )*
                km
            }
        }
    };
}
