extern crate libc;

use coinapi::array_t::array_t;
use coinapi::value::value_t;
use coinapi::str_t::str_t;

#[repr(C)]
pub struct map_t
{
    pub array: *mut array_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_value_t
{
    pub key: *mut str_t,
    pub value: value_t,
}