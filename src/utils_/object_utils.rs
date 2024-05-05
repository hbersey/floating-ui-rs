use crate::error::{Error, Result};
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

pub fn get_js_value(object: &Object, key: &str) -> Result<JsValue> {
    let property_key = JsValue::from_str(key);

    Reflect::get(&object, &property_key).map_err(Error::map_reflect_get(key.to_string()))
}

pub fn get_option_js_value(object: &Object, key: &str) -> Result<Option<JsValue>> {
    let property_key = JsValue::from_str(key);

    let value =
        Reflect::get(&object, &property_key).map_err(Error::map_reflect_get(key.to_string()))?;

    if value.is_undefined() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

pub fn try_get<T>(object: &Object, key: &str) -> Result<T>
where
    T: TryFrom<JsValue, Error = Error>,
{
    get_js_value(object, key)?.try_into()
}

pub fn try_get_option<T>(object: &Object, key: &str) -> Result<Option<T>>
where
    T: TryFrom<JsValue, Error = Error>,
{
    if let Some(value) = get_option_js_value(object, key)? {
        Ok(Some(value.try_into()?))
    } else {
        Ok(None)
    }
}

pub fn get_f64(object: &Object, key: &str) -> Result<f64> {
    let value = get_js_value(object, key)?;

    value
        .as_f64()
        .ok_or(Error::InvalidJsValue(key.to_string(), "f64"))
}

pub fn get_option_f64(object: &Object, key: &str) -> Result<Option<f64>> {
    if let Some(value) = get_option_js_value(object, key)? {
        Ok(Some(
            value
                .as_f64()
                .ok_or(Error::InvalidJsValue(key.to_string(), "f64"))?,
        ))
    } else {
        Ok(None)
    }
}

pub fn get_bool(object: &Object, key: &str) -> Result<bool> {
    let value = get_js_value(object, key)?;

    value
        .as_bool()
        .ok_or(Error::InvalidJsValue(key.to_string(), "bool"))
}

pub fn get_option_bool(object: &Object, key: &str) -> Result<Option<bool>> {
    if let Some(value) = get_option_js_value(object, key)? {
        Ok(Some(
            value
                .as_bool()
                .ok_or(Error::InvalidJsValue(key.to_string(), "bool"))?,
        ))
    } else {
        Ok(None)
    }
}

pub fn get_string(object: &Object, key: &str) -> Result<String> {
    let value = get_js_value(object, key)?;

    value
        .as_string()
        .ok_or(Error::InvalidJsValue(key.to_string(), "string"))
}

pub fn set_f64(object: &Object, key: &str, value: f64) -> Result<()> {
    let property_key = JsValue::from_str(key);
    let value = JsValue::from_f64(value);

    let res = Reflect::set(&object, &property_key, &value)
        .map_err(Error::map_reflect_set(key.to_string()))?;

    if !res {
        Err(Error::ReflectSet(
            key.to_string(),
            "Setting was unsuccessful.".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn set_option_f64(object: &Object, key: &str, value: Option<f64>) -> Result<()> {
    if let Some(value) = value {
        set_f64(object, key, value)
    } else {
        Ok(())
    }
}

pub fn set_bool(object: &Object, key: &str, value: bool) -> Result<()> {
    let property_key = JsValue::from_str(key);
    let value = JsValue::from_bool(value);

    let res = Reflect::set(&object, &property_key, &value)
        .map_err(Error::map_reflect_set(key.to_string()))?;

    if !res {
        Err(Error::ReflectSet(
            key.to_string(),
            "Setting was unsuccessful.".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn set_option_bool(object: &Object, key: &str, value: Option<bool>) -> Result<()> {
    if let Some(value) = value {
        set_bool(object, key, value)
    } else {
        Ok(())
    }
}

pub fn try_set_js_value(
    object: &Object,
    key: &str,
    value: impl TryInto<JsValue, Error = Error>,
) -> Result<()> {
    set_js_value(object, key, &value.try_into()?)
}

pub fn set_js_value(object: &Object, key: &str, value: &JsValue) -> Result<()> {
    let property_key = JsValue::from_str(key);

    let res = Reflect::set(&object, &property_key, value)
        .map_err(Error::map_reflect_set(key.to_string()))?;

    if !res {
        Err(Error::ReflectSet(
            key.to_string(),
            "Setting was unsuccessful.".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn try_set_option_js_value(
    object: &Object,
    key: &str,
    value: Option<impl TryInto<JsValue, Error = Error>>,
) -> Result<()> {
    if let Some(value) = value {
        set_js_value(object, key, &value.try_into()?)
    } else {
        Ok(())
    }
}

pub fn set_option_js_value(object: &Object, key: &str, value: Option<JsValue>) -> Result<()> {
    if let Some(value) = value {
        set_js_value(object, key, &value)
    } else {
        Ok(())
    }
}

pub fn set_string(object: &Object, key: &str, value: &str) -> Result<()> {
    let property_key = JsValue::from_str(key);
    let value = JsValue::from_str(value);

    let res = Reflect::set(&object, &property_key, &value)
        .map_err(Error::map_reflect_set(key.to_string()))?;

    if !res {
        Err(Error::ReflectSet(
            key.to_string(),
            "Setting was unsuccessful.".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn set_option_string(object: &Object, key: &str, value: Option<&str>) -> Result<()> {
    if let Some(value) = value {
        set_string(object, key, value)
    } else {
        Ok(())
    }
}
