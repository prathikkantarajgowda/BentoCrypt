use core::mem;
use core::result::Result;
use core::intrinsics::copy_nonoverlapping;

#[derive(Debug)]
pub enum Error { SizeError }

pub unsafe trait DataBlock : Sized {
    fn extract_from(&mut self, slice: &[u8]) -> Result<usize, Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            unsafe {
                let dst_ptr: *mut u8 = self as *mut Self as *mut u8;
                let src_ptr: *const u8 = slice.as_ptr();
                copy_nonoverlapping(src_ptr, dst_ptr, sz);
            }
            Ok(sz)
        }
    }

    fn dump_into(&self, slice: &mut [u8]) -> Result<usize, Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            unsafe {
                let dst_ptr: *mut u8 = slice.as_mut_ptr();
                let src_ptr: *const u8 = self as *const Self as *const u8;
                copy_nonoverlapping(src_ptr, dst_ptr, sz);
            }
            Ok(sz)
        }
    }

    #[inline]
    fn ref_from(slice: &[u8]) -> Result<&Self, Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            let ptr: *const u8 = slice.as_ptr();
            unsafe {
                Ok(&*(ptr as *const _))
            }
        }
    }

    #[inline]
    fn mut_ref_from(slice: &mut [u8]) -> Result<&mut Self, Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            let ptr: *mut u8 = slice.as_mut_ptr();
            unsafe {
                Ok(&mut *(ptr as *mut _))
            }
        }
    }

    #[inline]
    fn ref_from_prefix(slice: &[u8]) -> Result<(&Self, &[u8]), Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            let (slice, rest) = slice.split_at(sz);
            let ptr: *const u8 = slice.as_ptr();
            unsafe {
                Ok((&*(ptr as *const _), rest))
            }
        }
    }

    #[inline]
    fn ref_from_mut_prefix(slice: &mut [u8]) -> Result<(&Self, &mut [u8]), Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            let (slice, rest) = slice.split_at_mut(sz);
            let ptr: *const u8 = slice.as_ptr();
            unsafe {
                Ok((&*(ptr as *const _), rest))
            }
        }
    }

    #[inline]
    fn mut_ref_from_prefix(slice: &mut [u8]) -> Result<(&mut Self, &mut [u8]), Error> {
        let sz = mem::size_of::<Self>();
        if sz > slice.len() {
            Err(Error::SizeError)
        } else {
            let (slice, rest) = slice.split_at_mut(sz);
            let ptr: *mut u8 = slice.as_mut_ptr();
            unsafe {
                Ok((&mut *(ptr as *mut _), rest))
            }
        }
    }
}

macro_rules! unsafe_impl {
    ($t:ty) => {
        unsafe impl DataBlock for $t {}
    }
}

// Primitives
unsafe_impl!(u8);
unsafe_impl!(u16);
unsafe_impl!(u32);
unsafe_impl!(u64);
unsafe_impl!(u128);

unsafe_impl!(i8);
unsafe_impl!(i16);
unsafe_impl!(i32);
unsafe_impl!(i64);
unsafe_impl!(i128);

unsafe_impl!(bool);
unsafe_impl!(char);
unsafe_impl!(isize);
unsafe_impl!(usize);

unsafe_impl!(f32);
unsafe_impl!(f64);


// tuples
unsafe impl<A, B> DataBlock for (A, B)
where
    A : DataBlock,
    B : DataBlock,
{}

unsafe impl<A, B, C> DataBlock for (A, B, C)
where
    A : DataBlock,
    B : DataBlock,
    C : DataBlock,
{}

unsafe impl<A, B, C, D> DataBlock for (A, B, C, D)
where
    A : DataBlock,
    B : DataBlock,
    C : DataBlock,
    D : DataBlock,
{}

unsafe impl<A, B, C, D, E> DataBlock for (A, B, C, D, E)
where
    A : DataBlock,
    B : DataBlock,
    C : DataBlock,
    D : DataBlock,
    E : DataBlock,
{}


// Arraies
unsafe_impl_array!(1024);
