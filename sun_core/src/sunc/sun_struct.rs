use crate::container::SunValue;
use crate::meta::OwnSunMeta;
use crate::utils::SunPointer;
use crate::{
    container::Function,
    meta::SunMeta,
    utils::{log::error_output, SunError, SunObject},
};
use libc::{c_char, c_double, c_void, size_t};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem::transmute;

#[repr(C)]
#[derive(Clone)]
pub struct ExportLibC {
    pub metas: *const SunMetaHashMapC,
    pub meta_len: size_t,
    pub values: *const SunPointerHashMapC,
    pub value_len: size_t,
}

/// HashMap<String, SunObject>
#[repr(C)]
#[derive(Clone)]
pub struct SunMetaHashMapC {
    pub key: *const c_char,
    pub object: *const SunObjectC,
}

// HashMap<String, SunPointer>
#[repr(C)]
#[derive(Clone)]
pub struct SunPointerHashMapC {
    pub key: *const c_char,
    pub pointer: *const SunPointerC,
}

// SunObject
#[repr(C)]
#[derive(Clone)]
pub struct SunObjectC {
    pub meta: *const SunMetaC,
}

// SunMeta
#[repr(C)]
#[derive(Clone)]
pub struct SunMetaC {
    pub name: *const c_char,
    pub methods: *const MethodsHashMapC,
    pub method_len: size_t,
}

// HashMap<String, Function>
#[repr(C)]
#[derive(Clone)]
pub struct MethodsHashMapC {
    pub key: *const c_char,
    pub method: *const FunctionC,
}

// Function的枚举
#[repr(C)]
#[derive(Clone)]
pub enum FunctionType {
    RustFunction,
    SysFunction,
}

// Function的数据
#[repr(C)]
#[derive(Copy, Clone)]
pub union FunctionData {
    pub rust_function: *const c_void,
    pub sys_function: *const c_void,
}

// Function
#[repr(C)]
#[derive(Clone)]
pub struct FunctionC {
    pub _type: FunctionType,
    pub data: FunctionData,
}

// SunValue 的枚举
#[repr(C)]
#[derive(Clone)]
pub enum SunValueType {
    SunNil,
    SunBoolean,
    SunNumber,
    SunString,
    SunTable,
    SunFunction,
    SunClass,
}

// SunValue 的数据
#[repr(C)]
#[derive(Clone, Copy)]
pub union SunValueData {
    pub boolean: bool,
    pub number: c_double,
    pub string: *const c_char,
    pub table: *const TableC,
    pub function: *const FunctionC,
    pub class: *const ClassC,
}

// SunValue
#[repr(C)]
#[derive(Clone)]
pub struct SunValueC {
    pub _type: SunValueType,
    pub data: SunValueData,
}

// SunPointer
#[repr(C)]
#[derive(Clone)]
pub struct SunPointerC {
    pub data: *const SunValueC,
}

// Table
#[repr(C)]
#[derive(Clone)]
pub struct TableC {
    pub array: *const SunPointerC,
    pub array_len: size_t,
    pub dict: *const SunPointerHashMapC,
    pub dict_len: size_t,
}

// Class
#[repr(C)]
#[derive(Clone)]
pub struct ClassC {
    pub name: *const c_char,
    pub attributes: *const SunPointerHashMapC,
    pub attr_len: size_t,
}

impl From<SunObjectC> for SunObject {
    fn from(value: SunObjectC) -> Self {
        let meta = unsafe { (&*(value.meta)).clone().into() };
        SunObject { meta }
    }
}

impl From<SunObject> for SunObjectC {
    fn from(value: SunObject) -> Self {
        let name = CString::new(value.get_name());
        let name = match name {
            Ok(name) => name,
            Err(e) => error_output(SunError::ParaError(e.to_string())),
        };
        let method_len = value.get_meta().get_all().len();
        let methods: Vec<MethodsHashMapC> = value
            .get_meta()
            .get_all()
            .iter()
            .map(|(k, v)| {
                let k = CString::new(k.to_string());
                let k = match k {
                    Ok(k) => k,
                    Err(e) => error_output(SunError::ParaError(e.to_string())),
                };
                let v = match v {
                    Function::RustFunction(f) => FunctionC {
                        _type: FunctionType::RustFunction,
                        data: FunctionData {
                            rust_function: unsafe { transmute::<_, *mut c_void>(f) },
                        },
                    },
                    Function::SysFunction(f) => FunctionC {
                        _type: FunctionType::SysFunction,
                        data: FunctionData {
                            sys_function: unsafe { transmute::<_, *mut c_void>(f) },
                        },
                    },
                };
                MethodsHashMapC {
                    key: k.as_ptr(),
                    method: &v as *const FunctionC,
                }
            })
            .collect();
        let methods = methods.as_ptr();
        let meta = SunMetaC {
            name: name.as_ptr(),
            methods,
            method_len,
        };
        let meta = &meta as *const SunMetaC;
        SunObjectC { meta }
    }
}

impl From<SunMetaC> for SunMeta {
    fn from(value: SunMetaC) -> Self {
        let mut methods = HashMap::new();
        for j in 0..value.method_len {
            let key = unsafe { (*(value.methods.offset(j as isize))).key };
            let method_c = unsafe { &*(*(value.methods.offset(j as isize))).method };
            methods.insert(
                unsafe { (*key).to_string() },
                Function::from(method_c.clone()),
            );
        }
        let name = unsafe { CStr::from_ptr(value.name).to_string_lossy().into_owned() };
        let name: &'static str = Box::leak(name.into_boxed_str());
        let sun_meta = SunMeta { name, methods };
        sun_meta
    }
}

impl From<SunPointer> for SunPointerC {
    fn from(value: SunPointer) -> Self {
        let data = value.get();
        let data = match data {
            SunValue::Nil => SunValueC {
                _type: SunValueType::SunNil,
                data: SunValueData { boolean: false },
            },
            SunValue::Boolean(b) => SunValueC {
                _type: SunValueType::SunBoolean,
                data: SunValueData { boolean: b },
            },
            SunValue::Number(n) => SunValueC {
                _type: SunValueType::SunNumber,
                data: SunValueData { number: n },
            },
            SunValue::Class(c) => {
                let name = CString::new(c.get_name()).unwrap();
                let attr_len = c.get_all().len();
                let attr: Vec<SunPointerHashMapC> = c
                    .get_all()
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
                let class = ClassC {
                    name: name.as_ptr(),
                    attr_len,
                    attributes: attr.as_ptr(),
                };
                let class = &class as *const ClassC;
                SunValueC {
                    _type: SunValueType::SunClass,
                    data: SunValueData { class },
                }
            }
            SunValue::Function(f) => {
                let f = match f {
                    Function::RustFunction(f) => FunctionC {
                        _type: FunctionType::RustFunction,
                        data: FunctionData {
                            rust_function: unsafe { transmute::<_, *mut c_void>(f) },
                        },
                    },
                    Function::SysFunction(f) => FunctionC {
                        _type: FunctionType::SysFunction,
                        data: FunctionData {
                            sys_function: unsafe { transmute::<_, *mut c_void>(f) },
                        },
                    },
                };
                let f = &f as *const FunctionC;
                SunValueC {
                    _type: SunValueType::SunFunction,
                    data: SunValueData { function: f },
                }
            }
            SunValue::String(s) => {
                let s = CString::new(s).unwrap();
                SunValueC {
                    _type: SunValueType::SunString,
                    data: SunValueData { string: s.as_ptr() },
                }
            }
            SunValue::Table(t) => {
                let array_len = t.get_array().len();
                let dict_len = t.get_dict().len();
                let array: Vec<SunPointerC> = t
                    .get_array()
                    .into_iter()
                    .map(|v| SunPointerC::from(v))
                    .collect();
                let dict: Vec<SunPointerHashMapC> = t
                    .get_dict()
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
                let table = TableC {
                    array: array.as_ptr(),
                    array_len,
                    dict: dict.as_ptr(),
                    dict_len,
                };
                let table = &table as *const TableC;
                SunValueC {
                    _type: SunValueType::SunTable,
                    data: SunValueData { table },
                }
            }
        };
        let data = &data as *const SunValueC;
        SunPointerC { data }
    }
}
