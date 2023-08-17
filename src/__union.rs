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
