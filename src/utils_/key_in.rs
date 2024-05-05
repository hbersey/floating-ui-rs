macro_rules! key_in {
    ($name:ident, $in:ty {$($member:ident),*} => $type:ty) => {
        assert_impl_all!($type: Into<wasm_bindgen::JsValue>);

        paste::paste! {
            #[derive(Clone)]
            pub struct $name {
                $(pub [<$member:snake>]: $type),*
            }

            impl TryInto<wasm_bindgen::JsValue> for $name {
                type Error = crate::Error;

                fn try_into(self) -> crate::Result<wasm_bindgen::JsValue> {
                    let obj = js_sys::Object::new();

                    $(
                        js_sys::Reflect::set(
                            &obj,
                            &stringify!([<$member:snake>]).into(),
                            &self.[<$member:snake>].into())
                                .map_err(crate::Error::map_reflect_set(stringify!([<$member:snake>]).to_string()))?;
                    )*

                    Ok(obj.into())
                }
            }

            impl TryFrom<wasm_bindgen::JsValue> for $name {
                type Error = crate::Error;

                fn try_from(value: wasm_bindgen::JsValue) -> crate::Result<Self> {
                    let obj = js_sys::Object::from(value);

                    $(
                        let [<$member:snake>] = crate::utils_::object_utils::get_f64(&obj, stringify!([<$member:snake>]))?;
                    )*

                    Ok(Self {
                        $([<$member:snake>]),*
                    })
                }
            }

            crate::utils_::index!($name, $in {$($member),*} => $type);

            pub struct [< Partial $name> ] {
                $(pub [<$member:snake>]: Option<$type>),*
            }

            crate::utils_::index!([< Partial $name> ], $in {$($member),*} => Option<$type>);

        }
    };
}

pub(crate) use key_in;
