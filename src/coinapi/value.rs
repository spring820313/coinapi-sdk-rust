extern crate libc;
use coinapi::str_t::str_t;
use coinapi::obj_t::obj_t;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum value_type_t {
    kValueTypeInvalid = 0,
    kValueTypeInt8,
    kValueTypeUInt8,
    kValueTypeInt16,
    kValueTypeUInt16,
    kValueTypeInt32,
    kValueTypeUInt32,
    kValueTypeInt64,
    kValueTypeUInt64,
    kValueTypePointer,
    kValueTypeFloat32,
    kValueTypeFloat64,
    kValueTypeString,
    kValueTypeObject,
}

pub type destroy_t = unsafe fn(data: *mut libc::c_void);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pointer_t
{
    pub data: *mut libc::c_void,
    pub destroy: destroy_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union type_t {
    pub i8: libc::c_schar,
    pub u8: libc::c_char,
    pub i16: libc::c_short,
    pub u16: libc::c_ushort,
    pub i32: libc::c_int,
    pub u32: libc::c_uint,
    pub i64: libc::c_longlong,
    pub u64: libc::c_ulonglong,
    pub f32: libc::c_float,
    pub f64: libc::c_double,

    pub ptr: pointer_t,
    pub str: *mut str_t,
    pub obj: *mut obj_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_t
{
    pub vtype : libc::c_uint,
    pub value: type_t,
}