extern crate libc;

use std::ptr;
use coinapi::str_t::str_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyPath
{
    pub path1 : libc::c_int,
    pub path2 : libc::c_int,
    pub path3 : libc::c_int,
    pub path4 : libc::c_int,
    pub path5 : libc::c_int,
    pub hd1 : libc::c_int,
    pub hd2 : libc::c_int,
    pub hd3 : libc::c_int,
    pub hd4 : libc::c_int,
    pub hd5 : libc::c_int,
    pub symbol: *mut str_t,
}

impl KeyPath {
    pub fn new() -> KeyPath {
        KeyPath{
            path1: -1,
            path2: -1,
            path3: -1,
            path4: -1,
            path5: -1,
            hd1: 0,
            hd2: 0,
            hd3: 0,
            hd4: 0,
            hd5: 0,
            symbol: ptr::null_mut(),
        }
    }
}