//! Conversion of data structures to and from MDB_val
//!
//! Since MDB_val is valid through whole transaction, it is kind of safe
//! to keep plain data, i.e. to keep raw pointers and transmute them back
//! and forward into corresponding data structures to avoid any unnecessary
//! copying.
//!
//! `MdbValue` is a simple wrapper with bounded lifetime which should help
//! keep it sane, i.e. provide compile errors when data retrieved outlives
//! transaction.
//!
//! It would be extremely helpful to create `compile-fail` tests to ensure
//! this, but unfortunately there is no way yet.


use std::{self, mem, slice};

use core::MdbValue;

pub trait AsByteSlice {
    fn as_byte_slice<'a>(&'a self) -> &'a [u8];
}

/// `FromMdbValue` is supposed to reconstruct a value from
/// memory slice. It allows to use zero copy where it is
/// required.
pub trait FromMdbValue {
    fn from_mdb_value(value: &MdbValue) -> Self;
}

impl FromMdbValue for String {
    fn from_mdb_value(value: &MdbValue) -> String {
        unsafe {
            let ptr = mem::transmute(value.get_ref());
            let data: Vec<u8> = slice::from_raw_parts(ptr, value.get_size()).to_vec();
            String::from_utf8(data).unwrap()
        }
    }
}

impl FromMdbValue for Vec<u8> {
    fn from_mdb_value(value: &MdbValue) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(mem::transmute(value.get_ref()),
                                value.get_size(),
                                value.get_size())
        }
    }
}

impl FromMdbValue for () {
    fn from_mdb_value(_: &MdbValue) {
    }
}

impl<'b> FromMdbValue for &'b str {
    fn from_mdb_value(value: &MdbValue) -> &'b str {
        unsafe {
            std::mem::transmute(slice::from_raw_parts(value.get_ref(), value.get_size()))
        }
    }
}

impl<'b> FromMdbValue for &'b [u8] {
    fn from_mdb_value(value: &MdbValue) -> &'b [u8] {
        unsafe {
            std::mem::transmute(slice::from_raw_parts(value.get_ref(), value.get_size()))
        }
    }
}

macro_rules! mdb_for_primitive {
    ($t:ty) => (
        impl FromMdbValue for $t {
            fn from_mdb_value(value: &MdbValue) -> $t {
                unsafe {
                    let t: *mut $t = mem::transmute(value.get_ref());
                    *t
                }
            }
        }

        )
}

mdb_for_primitive!(u8);
mdb_for_primitive!(i8);
mdb_for_primitive!(u16);
mdb_for_primitive!(i16);
mdb_for_primitive!(u32);
mdb_for_primitive!(i32);
mdb_for_primitive!(u64);
mdb_for_primitive!(i64);
mdb_for_primitive!(f32);
mdb_for_primitive!(f64);
mdb_for_primitive!(bool);
