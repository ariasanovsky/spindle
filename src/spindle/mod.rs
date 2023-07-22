use cudarc::driver::DeviceRepr;

use crate::{DevSpindle, HostSpindle};

pub mod try_from;

// todo! are we implicitly using unstated traits?
/* https://doc.rust-lang.org/reference/items/unions.html
    union fields can be:
        T: Copy
        &T or &mut T
        ManuallyDrop<T>
        tuples of union fields
        arrays of union fields
*/
pub unsafe trait RawConvert<X>
where
    Self: Sized,
    X: Copy,
{
    // todo! ?relax Copy once we stabilize numeric primitives
    // todo! compile_error! if size_of<X> > size_of<Self>
    // todo! compile_error! if align_of<X> > align_of<Self>
    // todo! this is ugly, is it correct?
    unsafe fn from_raw(raw: X) -> Self {
        // todo! mut?
        let /* mut */ y = core::mem::MaybeUninit::<Self>::uninit().as_mut_ptr();
        core::ptr::copy_nonoverlapping(&raw as *const X as *const Self, y, 1);
        y.read()
    }

    // todo! seems okay
    unsafe fn ref_raw(&self) -> &X {
        &*(self as *const Self as *const X)
    }

    unsafe fn as_raw(&self) -> X {
        // self.ref_raw().clone()  // Copy -> Clone
        *(self as *const Self as *const X)
    }
}

impl<X, U> DevSpindle<U, X>
where
    U: RawConvert<X> + DeviceRepr,
    X: Copy, // todo! relax once we stabilize numeric primitives
{
    // X marks the current state of the union
    // [x] self gets initialized from a Vec<X> usually
    // [x] using cudarc, the contents decay into a Vec<U>
    // [x] we'll use HostSpindle<U, X> to wrap and mark this Vec for safety

    // we want fn foo(x: X) -> Y to macro expand into a trait _Foo
    // which we impl for Self so that
    // self.foo() is a DevSpindle<U, Y>

    pub fn try_to_host(self) -> Result<HostSpindle<U, X>, super::error::Error> {
        self.try_into()
    }
}

impl<X, U> HostSpindle<U, X>
where
    U: RawConvert<X>,
    X: Copy, // todo! relax once we stabilize numeric primitives
{
    // see above
    // [x] this struct is the byproduct of a DevSpindle<U, X> being reclaimed
    // [x] it is natural to treat it as an iterator over X
    // [?] the user can convert it into a Vec<X> if they want
    // we will not perform operations on it directly

    pub fn get(&self, i: usize) -> Option<&X> {
        unsafe { self.0.get(i).map(|u| u.ref_raw()) }
    }

    pub fn iter(&self) -> std::iter::Map<std::slice::Iter<'_, U>, fn(&U) -> &X> {
        self.0.iter().map(|u| unsafe { u.ref_raw() })
    }
}

#[allow(unused)]
#[cfg(test)]
mod test_union_from_raw {
    use super::*;
    #[test]
    fn test_f64_bool_u32_union_from_raw() {
        union U {
            _0: f64,
            _1: bool,
            _2: u32,
        }
        unsafe impl RawConvert<f64> for U {}
        unsafe impl RawConvert<bool> for U {}
        unsafe impl RawConvert<u32> for U {}

        let u = U { _0: 1.0 };
        let f: f64 = unsafe { *u.ref_raw() };
        assert_eq!(f, 1.0);
        let f: f64 = unsafe { u.as_raw() };
        let u = U { _1: true };
        let b: &bool = unsafe { u.ref_raw() };
        // drop(u); // does not compile 💔
        assert_eq!(*b, true);
    }
}
