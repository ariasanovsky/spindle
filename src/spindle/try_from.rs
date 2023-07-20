use std::sync::Arc;

use cudarc::driver::{CudaDevice, CudaSlice, DeviceRepr};
use super::{DevSpindle, RawConvert, error::Error, HostSpindle};

impl<U, X> TryFrom<Vec<X>> for DevSpindle<U, X>
where
    U: RawConvert<X> + DeviceRepr,
{
    type Error = Error;

    fn try_from(value: Vec<X>) -> Result<Self, Self::Error> {
        let dev: Arc<CudaDevice> = CudaDevice::new(0)?;

        // todo! unnecessary alloc
        let value: Vec<U> = value.into_iter().map(|x| unsafe { U::from_raw(x) }).collect();

        let slice: CudaSlice<U> = dev.htod_sync_copy(&value)?;
        Ok(Self(slice, std::marker::PhantomData))
    }
}

impl<U, X> TryFrom<DevSpindle<U, X>> for HostSpindle<U, X>
where
    U: RawConvert<X> + DeviceRepr,
{
    type Error = Error;

    fn try_from(value: DevSpindle<U, X>) -> Result<Self, Self::Error> {
        let dev: Arc<CudaDevice> = value.0.device();
        let value: Vec<U> = dev.dtoh_sync_copy(&value.0)?;
        Ok(Self(value, std::marker::PhantomData))
    }
}

impl<U, X> TryFrom<HostSpindle<U, X>> for DevSpindle<U, X>
where
    U: RawConvert<X> + DeviceRepr,
{
    type Error = Error;

    fn try_from(value: HostSpindle<U, X>) -> Result<Self, Self::Error> {
        let dev: Arc<CudaDevice> = CudaDevice::new(0)?;
        let slice: CudaSlice<U> = dev.htod_sync_copy(&value.0)?;
        Ok(Self(slice, std::marker::PhantomData))
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test_conversions {
    #[test]
    fn test_f64_bool_u32_conversions() {
        use super::*;
        union MyUnion {
            f64: f64,
            bool: bool,
            u32: u32,
        }
        unsafe impl RawConvert<f64> for MyUnion {}
        unsafe impl RawConvert<bool> for MyUnion {}
        unsafe impl RawConvert<u32> for MyUnion {}
        unsafe impl DeviceRepr for MyUnion {}

        let host: Vec<f64> = vec![1.0, 2.0, 3.0];
        dbg!(&host);
        let spindle: DevSpindle<MyUnion, f64> = host.try_into().unwrap();
        println!("spindle");
        let host: HostSpindle<MyUnion, f64> = spindle.try_into().unwrap();
        println!("host");
        for u in host.0.iter() {
            println!("u: {:?}", unsafe { u.f64 });
        }
        host.iter().for_each(|x| { dbg!(x); });
        let host: Vec<&f64> = host.iter().collect();
        
        dbg!(&host);

        assert_eq!(host, vec![&1.0, &2.0, &3.0]);

        // let host: Vec<f64> = vec![1.0, 2.0, 3.0];
        // let spindle: DevSpindle<MyUnion, f64> = host.try_into().unwrap();
        // let host = spindle.try_to_host().unwrap();
        // let host: Vec<&f64> = host.iter().collect();
        // assert_eq!(host, vec![&1.0, &2.0, &3.0])
    }
}