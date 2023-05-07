use crate::{
    sunc::sun_struct::*,
    utils::{log::error_output, SunError, SunObject, SunPointer},
};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

pub struct ExportLib {
    pub meta: HashMap<String, SunObject>,
    pub value: HashMap<String, SunPointer>,
}

impl ExportLib {
    pub fn new(meta: HashMap<String, SunObject>, value: HashMap<String, SunPointer>) -> Self {
        ExportLib { meta, value }
    }
}

pub fn to_c(lib: ExportLib) -> ExportLibC {
    let meta_len = lib.meta.len();
    let value_len = lib.value.len();
    let metas: Vec<SunMetaHashMapC> = lib
        .meta
        .into_iter()
        .map(|(k, v)| {
            let k = CString::new(k);
            let k = match k {
                Ok(k) => k,
                Err(e) => error_output(SunError::ParaError(e.to_string())),
            };
            let v = SunObjectC::from(v);
            SunMetaHashMapC {
                key: k.as_ptr(),
                object: &v as *const SunObjectC,
            }
        })
        .collect();
    let metas = metas.as_ptr();
    let values: Vec<SunPointerHashMapC> = lib
        .value
        .into_iter()
        .map(|(k, v)| {
            let k = CString::new(k);
            let k = match k {
                Ok(k) => k,
                Err(e) => error_output(SunError::ParaError(e.to_string())),
            };
            let v = SunPointerC::from(v);
            SunPointerHashMapC {
                key: k.as_ptr(),
                pointer: &v as *const SunPointerC,
            }
        })
        .collect();
    let values = values.as_ptr();
    ExportLibC {
        meta_len,
        metas,
        value_len,
        values,
    }
}

pub fn to_rust(clib: ExportLibC) -> ExportLib {
    let mut meta = HashMap::new();
    for i in 0..clib.meta_len {
        let meta_c = unsafe { &*clib.metas.offset(i as isize) };
        let name = unsafe { CStr::from_ptr(meta_c.key).to_string_lossy().into_owned() };
        let obj = unsafe { (*meta_c.object).clone().into() };
        meta.insert(name.to_string(), obj);
    }

    let mut value = HashMap::new();
    for i in 0..clib.value_len {
        let value_c = unsafe { &*clib.values.offset(i as isize) };
        let sun_value = unsafe { &*(*(*value_c).pointer).data };
        let sun_pointer = SunPointer::new(sun_value.clone().into());
        let name = unsafe { CStr::from_ptr(value_c.key).to_string_lossy().into_owned() };
        value.insert(name, sun_pointer);
    }

    ExportLib { meta, value }
}
