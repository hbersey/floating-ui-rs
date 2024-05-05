macro_rules! extends {
    ($child:ty => $parent:ty, $($member:ident),*) => {
        paste::paste! {
            impl From<$child> for $parent {
                fn from(child: $child) -> Self {
                    Self {
                        $(
                            [<$member:snake>]: child.[<$member:snake>],
                        )*
                    }
                }
            }
        }
    };
}

pub(crate) use extends;
