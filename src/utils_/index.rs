macro_rules! index {
    ($name:ident, $in:ty {$($member:ident),*} => $type:ty) => {
        paste::paste! {
            impl std::ops::Index<&$in> for $name {
                type Output = $type;

                fn index(&self, index: &$in) -> &Self::Output {
                    match *index {
                        $(<$in>::[<$member:camel>] => &self.[<$member:snake>],)*
                    }
                }
            }

            impl std::ops::IndexMut<&$in> for $name {
                fn index_mut(&mut self, index: &$in) -> &mut Self::Output {
                    match *index {
                        $(<$in>::[<$member:camel>] => &mut self.[<$member:snake>],)*
                    }
                }
            }
        }
    };
}

pub(crate) use index;