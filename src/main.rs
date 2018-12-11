mod coinapi;

extern crate libc;
use std::ffi::CStr;
use coinapi::str_t::str_t;
use coinapi::keypath::KeyPath;
use coinapi::netparams::NetParams;
use coinapi::btwallet::btwallet;
use coinapi::btc_vout::btc_vout;
use coinapi::value::value_t;
use coinapi::value::value_type_t;
use coinapi::value::type_t;
use coinapi::map_t::key_value_t;
use coinapi::btc_transaction_params::btc_transaction_params;
use std::ffi::CString;
use coinapi::def::CoinType;
use coinapi::def::NetworkType;
use coinapi::map_t::map_t;
use std::ptr;
use std::mem;
use std::collections::HashMap;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}

#[macro_export]
macro_rules! c_str {
    ($lit:expr) => {
        // Currently, there is no working way to concatenate a byte string
        // literal out of bytestring or string literals. Otherwise, we could
        // use from_static_bytes and accept byte strings as well.
        // See https://github.com/rust-lang/rfcs/pull/566
        unsafe {
            std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr()
                                     as *const std::os::raw::c_char)
        }
    }
}

fn btc_vout_destroy(data: *mut libc::c_void){
    let p = data as *const btc_vout as *mut btc_vout;
    if p != ptr::null_mut() {
        unsafe {
            coinapi::btc_vout_free(p);
            libc::free(data)
        };
    }

}

//TODO
fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);

    let h : *mut libc::c_void = unsafe {coinapi::init()};
    let addr = h as usize;
    println!("addr: 0x{:X}",addr);

    let words: *mut libc::c_char = unsafe {coinapi::createAllCoinMnemonicCode(h)};
    let buf_words = unsafe { CStr::from_ptr(words).to_bytes() };
    let str_words = String::from_utf8(buf_words.to_vec()).unwrap();
    println!("words: {}", str_words);

    let ret: libc::c_int = unsafe {coinapi::checkMnemonicCode(h, words)};
    println!("result: {}", ret);

    let cwords = unsafe {
        assert!(!words.is_null());
        CStr::from_ptr(words).to_str().unwrap()
    };
    let len = cwords.len();
    let str_tmp: *mut str_t = unsafe {coinapi::str_create(words, len as u32, 0)};
    let tmp = unsafe {coinapi::str_unref(str_tmp)};

    let data: Vec<u8> = b"BTC".to_vec();
    let btc = CString::new(data).unwrap();
    let str_btc: *mut str_t = unsafe {coinapi::str_create(btc.as_ptr(), 3, 0)};

    let kp : KeyPath = KeyPath {
        path1: 44,
        path2: 0,
        path3: 0,
        path4: 0,
        path5: 0,
        hd1: 1,
        hd2: 1,
        hd3: 1,
        hd4: 0,
        hd5: 0,
        symbol: str_btc,
    };

    let np : &mut NetParams = &mut NetParams {
        symbol: str_btc,
        coinType: CoinType::kCoinTypeBTC,
        nettype: NetworkType::kNetworkTypeTest,
        keyPath: kp,
        version: 2,
        HDprivate: 0x04358394,
        HDpublic: 0x043587CF,
        P2KH: 0x6f,
        P2SH: 0xC4,
        keyprefixes: 239,
        ApiVersion: 2,
        N: 32768,
        R: 8,
        P: 1,
    };

    let pwd_data: Vec<u8> = b"12345".to_vec();
    let pwd = CString::new(pwd_data).unwrap();
    let bw: &mut btwallet = &mut btwallet::new();
    let ret = unsafe { coinapi::createWallet(h, words, pwd.as_ptr(), np, bw) };

    let from: &CStr = c_str!("n2w5TLN2hgYuumewuWrCTEsizhUAvxhSge");
    let mut length : usize = unsafe {libc::strlen(from.as_ptr() as *const libc::c_char)};
    let fromAddress = unsafe {coinapi::str_create(from.as_ptr(), length as u32, 0)};

    let to: &CStr = c_str!("2N5SZ9bMxhEfDZ3bWXHnHW4RngczVE9nb16");
    length = unsafe {libc::strlen(to.as_ptr() as *const libc::c_char)};
    let toAddress = unsafe {coinapi::str_create(to.as_ptr(), length as u32, 0)};

    let tmp_pwd = pwd.as_c_str();
    length = unsafe {libc::strlen(tmp_pwd.as_ptr() as *const libc::c_char)};
    let password = unsafe {coinapi::str_create(tmp_pwd.as_ptr(), length as u32, 0)};

    let pri: &CStr = c_str!("cVraRiXNUghfAgsms7ZtwXkXCd5KoauvBs64hzsP4J7UMVyJXgiW");
    length = unsafe {libc::strlen(pri.as_ptr() as *const libc::c_char)};
    let priKey = unsafe {coinapi::str_create(pri.as_ptr(), length as u32, 0)};

    let tmp_value = c_str!("100000000");
    let sendValue = unsafe {coinapi::bigint_from_string(tmp_value.as_ptr() as *mut i8, 10)};

    let tmp_fee = c_str!("100000");
    let feePerKb = unsafe {coinapi::bigint_from_string(tmp_fee.as_ptr() as *mut i8, 10)};

    let tmp_hash: &CStr = c_str!("842596976630aa2664fa2a58b9c5b618ca1c34c23ccdfc7b6de281e393b3f7c4");
    length = unsafe {libc::strlen(tmp_hash.as_ptr() as *const libc::c_char)};
    let hash = unsafe {coinapi::str_create(tmp_hash.as_ptr(), length as u32, 0)};

    let tmp_value2 = c_str!("1000000000");
    let value2 = unsafe {coinapi::bigint_from_string(tmp_value2.as_ptr() as *mut i8, 10)};

    let bv : &mut btc_vout = &mut btc_vout {
        hash: hash,
        value: value2,
        n: 0,
        coinBase: 0,
    };

    let btcvoutFormList = unsafe {coinapi::array_create()};
    let t = type_t {
        i8: 1,
    };

    let vt = &mut value_t {
        vtype: 0u32,
        value: t,
    };

    unsafe { coinapi::value_set_pointer(vt, bv as *const btc_vout as *mut libc::c_void, btc_vout_destroy)};
    unsafe { coinapi::array_append(btcvoutFormList, *vt)};

    let btp: &mut btc_transaction_params = &mut btc_transaction_params {
        seed: ptr::null_mut(),
        fromAddress: fromAddress,
        sendAddress: toAddress,
        sendValue: sendValue,
        feePerKb: feePerKb,
        password: password,
        priKey: priKey,
        recipientsPayFees: 0,
        btcvoutFormList: btcvoutFormList,
        txType: -1,
        data: ptr::null_mut(),
    };

    let ret_map: *mut map_t = unsafe { coinapi::createSignTransaction(h, btp as *mut btc_transaction_params as *mut libc::c_void, np) };
    let size = unsafe { (*(*ret_map).array).size};
    let mut p = unsafe { (*(*ret_map).array).data};

    let mut ret_reviews = HashMap::new();

    for i in 0..size {
        let v = p;
        let vp = unsafe { coinapi::value_pointer(*v) };
        let kv = vp as *mut key_value_t;

        let mut key: &str = "";
        if unsafe { (*kv).key != ptr::null_mut()} {
            key = unsafe {
                CStr::from_ptr((*(*kv).key).str).to_str().unwrap()
            };
        }

        let value = unsafe { coinapi::value_str((*kv).value) };
        let vau = unsafe {
            CStr::from_ptr((*value).str).to_str().unwrap()
        };

        ret_reviews.insert(key,    vau);

        let ret = mem::size_of::<value_t>();
        p = unsafe {p.offset(1 as isize )};
    }

    for (k, v) in &ret_reviews {
        println!("{}: \"{}\"", k, v);
    }

    unsafe { coinapi::cleanup(h)};
}

