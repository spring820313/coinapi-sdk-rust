extern crate libc;

use coinapi::map_t::map_t;
use coinapi::value::value_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emitter_t
{
    pub listeners: *mut map_t,
}

pub type obj_dup_t = unsafe extern "C" fn(obj: *mut obj_t);
pub type obj_destroy_t = unsafe extern "C" fn(obj: *mut obj_t) -> libc::c_int;
pub type obj_copy_t = unsafe extern "C" fn(obj: *mut obj_t, other: *mut obj_t) -> libc::c_int;
pub type obj_set_prop_t = unsafe extern "C" fn(obj: *mut obj_t, prop: *const libc::c_char, value: value_t) -> libc::c_int;
pub type obj_get_prop_t = unsafe extern "C" fn(obj: *mut obj_t, prop: *const libc::c_char) -> value_t;

pub type obj_set_t = unsafe extern "C" fn(obj: *mut obj_t, value: value_t) -> libc::c_int;
pub type obj_get_t = unsafe extern "C" fn(obj: *mut obj_t) -> value_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct obj_t
{
    pub refcount: libc::c_ushort,
    pub magic: libc::c_ushort,
    pub emitter: *mut emitter_t,

    pub dup: obj_dup_t,
    pub copy: obj_copy_t,
    pub destroy: obj_destroy_t,
    pub set_prop: obj_set_prop_t,
    pub get_prop: obj_get_prop_t,
}