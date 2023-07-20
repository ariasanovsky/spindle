use cudarc::driver::{CudaSlice, DeviceRepr};

mod error;
mod try_from;

pub unsafe trait RawConvert<X>
where
    Self: Sized,
{
    // todo! compile_error! if size_of<X> > size_of<Self>
    fn from_raw(raw: X) -> Self {
        unsafe { std::mem::transmute_copy(&raw) }
    }

    unsafe fn raw_ref(&self) -> &X {
        &*(self as *const Self as *const X)
    }
}

pub struct DevSpindle<U, X>(CudaSlice<U>, std::marker::PhantomData<X>)
where
    U: RawConvert<X> + DeviceRepr
;

impl<X, U> DevSpindle<U, X>
where
    U: RawConvert<X> + DeviceRepr
{
    // X marks the current state of the union
    // [x] self gets initialized from a Vec<X> usually
    // [x] using cudarc, the contents decay into a Vec<U>
    // [x] we'll use HostSpindle<U, X> to wrap and mark this Vec for safety
    
    // we want fn foo(x: X) -> Y to macro expand into a trait _Foo
    // which we impl for Self so that
    // self.foo() is a DevSpindle<U, Y>

    pub fn try_to_host(self) -> Result<HostSpindle<U, X>, error::Error> {
        self.try_into()
    }
}

pub struct HostSpindle<U, X>(Vec<U>, std::marker::PhantomData<X>)
where
    U: RawConvert<X>
;

impl<X, U> HostSpindle<U, X>
where
    U: RawConvert<X>,
{
    // see above
    // [x] this struct is the byproduct of a DevSpindle<U, X> being reclaimed
    // [x] it is natural to treat it as an iterator over X
    // [?] the user can convert it into a Vec<X> if they want
    // we will not perform operations on it directly

    pub fn get(&self, i: usize) -> Option<&X> {
        unsafe { self.0.get(i).map(|u| u.raw_ref()) }
    }

    pub fn iter(&self) -> std::iter::Map<std::slice::Iter<'_, U>, fn(&U) -> &X> {
        self.0.iter().map(|u| unsafe { u.raw_ref() })
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test_union_from_raw {
    use super::*;
    #[test]
    fn test_f64_bool_u32_union_from_raw() {
        union U {
            f: f64,
            b: bool,
            u: u32,
        }
        unsafe impl RawConvert<f64> for U {}
        unsafe impl RawConvert<bool> for U {}
        unsafe impl RawConvert<u32> for U {}

        let u = U { f: 1.0 };
        let f: f64 = unsafe { *u.raw_ref() };
        assert_eq!(f, 1.0);
    }
}