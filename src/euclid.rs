use crate::{Peek, Poke};
use euclid::{
    TypedPoint2D, TypedRect, TypedSideOffsets2D, TypedSize2D, TypedTransform3D, TypedVector2D,
};

unsafe impl<T: Poke, U> Poke for TypedPoint2D<T, U> {
    #[inline(always)]
    fn max_size() -> usize {
        2 * <T>::max_size()
    }
    #[inline(always)]
    unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        let bytes = self.x.poke_into(bytes);
        let bytes = self.y.poke_into(bytes);
        bytes
    }
}
impl<T: Peek, U> Peek for TypedPoint2D<T, U> {
    #[inline(always)]
    unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let bytes = self.x.peek_from(bytes);
        let bytes = self.y.peek_from(bytes);
        bytes
    }
}

unsafe impl<T: Poke, U> Poke for TypedRect<T, U> {
    #[inline(always)]
    fn max_size() -> usize {
        TypedPoint2D::<T, U>::max_size() + TypedSize2D::<T, U>::max_size()
    }
    #[inline(always)]
    unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        let bytes = self.origin.poke_into(bytes);
        let bytes = self.size.poke_into(bytes);
        bytes
    }
}
impl<T: Peek, U> Peek for TypedRect<T, U> {
    #[inline(always)]
    unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let bytes = self.origin.peek_from(bytes);
        let bytes = self.size.peek_from(bytes);
        bytes
    }
}

unsafe impl<T: Poke, U> Poke for TypedSideOffsets2D<T, U> {
    #[inline(always)]
    fn max_size() -> usize {
        4 * <T>::max_size()
    }
    #[inline(always)]
    unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        let bytes = self.top.poke_into(bytes);
        let bytes = self.right.poke_into(bytes);
        let bytes = self.bottom.poke_into(bytes);
        let bytes = self.left.poke_into(bytes);
        bytes
    }
}
impl<T: Peek, U> Peek for TypedSideOffsets2D<T, U> {
    #[inline(always)]
    unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let bytes = self.top.peek_from(bytes);
        let bytes = self.right.peek_from(bytes);
        let bytes = self.bottom.peek_from(bytes);
        let bytes = self.left.peek_from(bytes);
        bytes
    }
}

unsafe impl<T: Poke, U> Poke for TypedSize2D<T, U> {
    #[inline(always)]
    fn max_size() -> usize {
        2 * <T>::max_size()
    }
    #[inline(always)]
    unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        let bytes = self.width.poke_into(bytes);
        let bytes = self.height.poke_into(bytes);
        bytes
    }
}
impl<T: Peek, U> Peek for TypedSize2D<T, U> {
    #[inline(always)]
    unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let bytes = self.width.peek_from(bytes);
        let bytes = self.height.peek_from(bytes);
        bytes
    }
}

unsafe impl<T: Poke, S, D> Poke for TypedTransform3D<T, S, D> {
    #[inline(always)]
    fn max_size() -> usize {
        16 * <T>::max_size()
    }
    #[inline(always)]
    unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        let bytes = self.m11.poke_into(bytes);
        let bytes = self.m12.poke_into(bytes);
        let bytes = self.m13.poke_into(bytes);
        let bytes = self.m14.poke_into(bytes);
        let bytes = self.m21.poke_into(bytes);
        let bytes = self.m22.poke_into(bytes);
        let bytes = self.m23.poke_into(bytes);
        let bytes = self.m24.poke_into(bytes);
        let bytes = self.m31.poke_into(bytes);
        let bytes = self.m32.poke_into(bytes);
        let bytes = self.m33.poke_into(bytes);
        let bytes = self.m34.poke_into(bytes);
        let bytes = self.m41.poke_into(bytes);
        let bytes = self.m42.poke_into(bytes);
        let bytes = self.m43.poke_into(bytes);
        let bytes = self.m44.poke_into(bytes);
        bytes
    }
}
impl<T: Peek, S, D> Peek for TypedTransform3D<T, S, D> {
    #[inline(always)]
    unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let bytes = self.m11.peek_from(bytes);
        let bytes = self.m12.peek_from(bytes);
        let bytes = self.m13.peek_from(bytes);
        let bytes = self.m14.peek_from(bytes);
        let bytes = self.m21.peek_from(bytes);
        let bytes = self.m22.peek_from(bytes);
        let bytes = self.m23.peek_from(bytes);
        let bytes = self.m24.peek_from(bytes);
        let bytes = self.m31.peek_from(bytes);
        let bytes = self.m32.peek_from(bytes);
        let bytes = self.m33.peek_from(bytes);
        let bytes = self.m34.peek_from(bytes);
        let bytes = self.m41.peek_from(bytes);
        let bytes = self.m42.peek_from(bytes);
        let bytes = self.m43.peek_from(bytes);
        let bytes = self.m44.peek_from(bytes);
        bytes
    }
}

unsafe impl<T: Poke, U> Poke for TypedVector2D<T, U> {
    #[inline(always)]
    fn max_size() -> usize {
        2 * <T>::max_size()
    }
    #[inline(always)]
    unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
        let bytes = self.x.poke_into(bytes);
        let bytes = self.y.poke_into(bytes);
        bytes
    }
}
impl<T: Peek, U> Peek for TypedVector2D<T, U> {
    #[inline(always)]
    unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
        let bytes = self.x.peek_from(bytes);
        let bytes = self.y.peek_from(bytes);
        bytes
    }
}
