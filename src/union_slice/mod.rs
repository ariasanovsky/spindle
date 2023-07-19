use cudarc::driver::CudaSlice;

pub unsafe trait RawConvert<X>
where
    Self: Sized,
{
    // todo! compile_error! if size_of<X> > size_of<Self>
    unsafe fn from_raw(raw: X) -> Self {
        std::mem::transmute_copy(&raw)
    }

    unsafe fn into_raw(self) -> X {
        std::mem::transmute_copy(&self)
    }
}

pub struct DevUnionSlice<U, X>(CudaSlice<U>, std::marker::PhantomData<X>)
where
    U: RawConvert<X>
;

impl<X, U> DevUnionSlice<U, X>
where
    U: RawConvert<X>,
{
    // X marks the current state of the union
    // self gets initialized from a Vec<X> usually
    // we want fn foo(x: X) -> Y to macro expand into a trait _Foo
    // which we impl for Self so that
    // self.foo() is a DevUnionSlice<U, Y>
    // using cudarc, the contents decay into a Vec<U>
    // we'll use HostUnionVec<U, X> to wrap and mark this Vec for safety
}

pub struct HostUnionVec<U, X>(Vec<U>, std::marker::PhantomData<X>)
where
    U: RawConvert<X>
;

impl<X, U> HostUnionVec<U, X>
where
    U: RawConvert<X>,
{
    // see above
    // this struct is the byproduct of a HostUnionVec<U, X> being reclaimed
    // it is natural to treat it as an iterator over X
    // the user can convert it into a Vec<X> if they want
    // we will not perform operations on it directly
}

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
        let f: f64 = unsafe { u.into_raw() };
        assert_eq!(f, 1.0);
    }
}