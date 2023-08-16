use std::sync::Arc;

use crate::{error::Error, __union::RawConvert, __cudarc::{CudaSlice, DeviceRepr, CudaDevice}};
use super::{DevSlice, HostSlice};

impl<U, X> From<DevSlice<U, X>> for CudaSlice<U>
where
    U: crate::__union::RawConvert<X> + DeviceRepr,
    X: Copy,
{
    fn from(value: DevSlice<U, X>) -> Self {
        value.0
    }
}

impl<U, X> From<CudaSlice<U>> for DevSlice<U, X>
where
    U: RawConvert<X> + DeviceRepr,
    X: Copy,
{
    fn from(value: CudaSlice<U>) -> Self {
        Self(value, std::marker::PhantomData)
    }
}

impl<U, X> TryFrom<Vec<X>> for DevSlice<U, X>
where
    U: RawConvert<X> + DeviceRepr,
    X: Copy,
{
    type Error = Error;

    fn try_from(value: Vec<X>) -> Result<Self, Self::Error> {
        let dev: Arc<CudaDevice> = CudaDevice::new(0)?;

        // todo! unnecessary alloc
        let value: Vec<U> = value
            .into_iter()
            .map(|x| unsafe { RawConvert::from_raw(x) })
            .collect();

        let slice: CudaSlice<U> = dev.htod_sync_copy(&value)?;
        Ok(slice.into())
    }
}

impl<U, X> TryFrom<DevSlice<U, X>> for HostSlice<U, X>
where
    U: RawConvert<X> + DeviceRepr,
    X: Copy,
{
    type Error = Error;

    fn try_from(value: DevSlice<U, X>) -> Result<Self, Self::Error> {
        let value: CudaSlice<U> = value.into();
        let dev: Arc<CudaDevice> = value.device();
        let value: Vec<U> = dev.dtoh_sync_copy(&value)?;
        Ok(Self(value, std::marker::PhantomData))
    }
}

impl<U, X> TryFrom<HostSlice<U, X>> for DevSlice<U, X>
where
    U: RawConvert<X> + DeviceRepr,
    X: Copy,
{
    type Error = Error;

    fn try_from(value: HostSlice<U, X>) -> Result<Self, Self::Error> {
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
        let spindle: DevSlice<MyUnion, f64> = host.try_into().unwrap();
        println!("spindle");
        let host: HostSlice<MyUnion, f64> = spindle.try_into().unwrap();
        println!("host");
        for u in host.0.iter() {
            println!("u: {:?}", unsafe { u.f64 });
        }
        host.iter().for_each(|x| {
            dbg!(x);
        });
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
