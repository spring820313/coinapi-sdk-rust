extern crate libc;

use coinapi::array_t::array_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encrypted_data
{
    pub initialisationVector : *mut array_t,
    pub encryptedBytes : *mut array_t,
}