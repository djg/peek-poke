#![no_std]

#[cfg(feature = "derive")]
pub use peek_poke_derive::*;

use core::{
    marker::PhantomData,
    mem::{size_of, uninitialized},
    ptr::copy_nonoverlapping,
};

#[inline(always)]
fn poke_into(bytes: *mut u8, v: &[u8]) -> *mut u8 {
    unsafe {
        copy_nonoverlapping(v.as_ptr(), bytes, v.len());
        bytes.add(v.len())
    }
}

#[inline(always)]
fn peek_from(v: &mut [u8], bytes: *const u8) -> *const u8 {
    unsafe {
        copy_nonoverlapping(bytes, v.as_mut_ptr(), v.len());
        bytes.add(v.len())
    }
}

#[cfg(feature = "extras")]
mod euclid;

pub trait PeekPoke {
    fn max_size() -> usize;
    fn poke_into(&self, bytes: *mut u8) -> *mut u8;
    fn peek_from(&mut self, bytes: *const u8) -> *const u8;
}

macro_rules! impl_for_integer {
    ($ty:ty) => {
        impl PeekPoke for $ty {
            #[inline(always)]
            fn max_size() -> usize {
                size_of::<Self>()
            }
            #[inline(always)]
            fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
                poke_into(bytes, &self.to_ne_bytes())
            }
            #[inline(always)]
            fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
                let mut int_bytes: [u8; size_of::<$ty>()] = unsafe { uninitialized() };
                let ptr = peek_from(&mut int_bytes, bytes);
                *self = <$ty>::from_ne_bytes(int_bytes);
                ptr
            }
        }
    };
}

impl_for_integer!(i8);
impl_for_integer!(i16);
impl_for_integer!(i32);
impl_for_integer!(i64);
impl_for_integer!(isize);

impl_for_integer!(u8);
impl_for_integer!(u16);
impl_for_integer!(u32);
impl_for_integer!(u64);
impl_for_integer!(usize);

impl PeekPoke for bool {
    #[inline(always)]
    fn max_size() -> usize {
        <u8>::max_size()
    }
    #[inline]
    fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        (*self as u8).poke_into(bytes)
    }
    #[inline]
    fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let mut int_bool = 0u8;
        let ptr = int_bool.peek_from(bytes);
        *self = int_bool != 0;
        ptr
    }
}

macro_rules! impl_for_float {
    ($fty:ty as $ity:ty) => {
        impl PeekPoke for $fty {
            #[inline(always)]
            fn max_size() -> usize {
                <$ity>::max_size()
            }
            #[inline]
            fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
                self.to_bits().poke_into(bytes)
            }
            #[inline(always)]
            fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
                let mut tmp: $ity = 0;
                let ptr = tmp.peek_from(bytes);
                *self = <$fty>::from_bits(tmp);
                ptr
            }
        }
    };
}

impl_for_float!(f32 as u32);
impl_for_float!(f64 as u64);

impl<T> PeekPoke for PhantomData<T> {
    #[inline(always)]
    fn max_size() -> usize {
        0
    }
    #[inline(always)]
    fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        bytes
    }
    #[inline(always)]
    fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        *self = PhantomData;
        bytes
    }
}

impl<T> PeekPoke for Option<T>
where
    T: PeekPoke,
{
    #[inline(always)]
    fn max_size() -> usize {
        <u8>::max_size() + <T>::max_size()
    }
    #[inline]
    fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        match self {
            None => 0u8.poke_into(bytes),
            Some(ref v) => {
                let bytes = 1u8.poke_into(bytes);
                let bytes = v.poke_into(bytes);
                bytes
            }
        }
    }
    #[inline]
    fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let mut variant = 0u8;
        let bytes = variant.peek_from(bytes);
        match variant {
            0 => {
                *self = None;
                bytes
            }
            1 => {
                let mut __0: T = unsafe { uninitialized() };
                let bytes = __0.peek_from(bytes);
                *self = Some(__0);
                bytes
            }
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_for_arrays {
    ($($len:tt)+) => {
        $(impl<T: PeekPoke> PeekPoke for [T; $len] {
            fn max_size() -> usize {
                $len * <T>::max_size()
            }
            fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
                self.iter().fold(bytes, |bytes, e| e.poke_into(bytes))
            }
            fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
                self.iter_mut().fold(bytes, |bytes, e| e.peek_from(bytes))
            }
        })+
    }
}

impl_for_arrays! {
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

impl PeekPoke for () {
    fn max_size() -> usize {
        0
    }
    fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        bytes
    }
    fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        *self = ();
        bytes
    }
}

macro_rules! impl_for_tuple {
    ($($n:tt: $ty:ident),+) => {
        impl<$($ty: PeekPoke),+> PeekPoke for ($($ty,)+) {
            #[inline(always)]
            fn max_size() -> usize {
                0 $(+ <$ty>::max_size())+
            }
            fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
                $(let bytes = self.$n.poke_into(bytes);)+
                bytes
            }
            fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
                $(let bytes = self.$n.peek_from(bytes);)+
                bytes
            }
        }
    }
}

impl_for_tuple!(0: A);
impl_for_tuple!(0: A, 1: B);
impl_for_tuple!(0: A, 1: B, 2: C);
impl_for_tuple!(0: A, 1: B, 2: C, 3: D);
impl_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
