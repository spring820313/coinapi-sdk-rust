extern crate libc;

#[repr(C)]
pub struct bigint_t
{
    pub sign: libc::c_schar,
    pub size: libc::size_t,
    pub data: *mut libc::c_uint,
}