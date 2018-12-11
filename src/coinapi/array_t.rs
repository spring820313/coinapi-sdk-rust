extern crate libc;

use coinapi::value::value_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_t
{
    pub data: *mut value_t,
    pub size : libc::c_uint,
    pub capacity: libc::c_uint,
}