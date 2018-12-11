extern crate libc;

#[repr(C)]
pub struct str_t
{
    pub refcount : libc::c_ushort,
    pub size : libc::c_uint,
    pub capacity: libc::c_uint,
    pub str: *mut libc::c_char,
}