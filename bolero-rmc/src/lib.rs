//! rmc plugin for bolero
//!
//! This crate should not be used directly. Instead, use `bolero`.

#[cfg(not(rmc))]
#[allow(dead_code)]
mod rmc {
    pub unsafe fn nondet<T>() -> T {
        todo!()
    }

    pub fn assume(cond: bool) {
        assert!(cond)
    }
}

#[doc(hidden)]
#[cfg(any(test, all(feature = "lib", fuzzing_rmc), all(feature = "lib", rmc)))]
pub mod lib {
    use super::*;
    use bolero_engine::{
        Driver, DriverMode, Engine, TargetLocation, Test, TestInput, TypeGenerator,
    };
    use core::ops::{Bound, RangeBounds};

    #[derive(Debug, Default)]
    pub struct RmcEngine;

    impl RmcEngine {
        pub fn new(_location: TargetLocation) -> Self {
            Self::default()
        }
    }

    impl<T: Test> Engine<T> for RmcEngine
    where
        T::Value: core::fmt::Debug,
    {
        type Output = ();

        fn set_driver_mode(&mut self, mode: DriverMode) {
            // rmc doesn't have a mode
            let _ = mode;
        }

        fn set_shrink_time(&mut self, shrink_time: core::time::Duration) {
            // rmc does its own shrinking
            let _ = shrink_time;
        }

        fn run(self, mut test: T) -> Self::Output {
            let mut input = RmcInput;
            match test.test(&mut input) {
                Ok(was_valid) => {
                    // make sure the input that we generated was valid
                    rmc::assume(was_valid);
                }
                Err(_) => {
                    panic!("test failed");
                }
            }
        }
    }

    struct RmcInput;

    impl<Output> TestInput<Output> for RmcInput {
        type Driver = RmcDriver;

        fn with_slice<F: FnMut(&[u8]) -> Output>(&mut self, f: &mut F) -> Output {
            const MAX_LEN: usize = 256;
            let array: [u8; MAX_LEN] = unsafe { rmc::nondet() };
            let len = unsafe { rmc::nondet() };
            rmc::assume(len <= MAX_LEN);
            f(&array[..len])
        }

        fn with_driver<F: FnMut(&mut Self::Driver) -> Output>(&mut self, f: &mut F) -> Output {
            f(&mut RmcDriver)
        }
    }

    struct RmcDriver;

    macro_rules! gen {
        ($name:ident, $ty:ident) => {
            fn $name(&mut self, min: Bound<&$ty>, max: Bound<&$ty>) -> Option<$ty> {
                let value: $ty = unsafe { rmc::nondet() };
                rmc::assume((min, max).contains(&value));
                Some(value)
            }
        };
    }

    impl Driver for RmcDriver {
        gen!(gen_u8, u8);

        gen!(gen_i8, i8);

        gen!(gen_u16, u16);

        gen!(gen_i16, i16);

        gen!(gen_u32, u32);

        gen!(gen_i32, i32);

        gen!(gen_u64, u64);

        gen!(gen_i64, i64);

        gen!(gen_u128, u128);

        gen!(gen_i128, i128);

        gen!(gen_usize, usize);

        gen!(gen_isize, isize);

        gen!(gen_f32, f32);

        gen!(gen_f64, f64);

        fn gen<T: TypeGenerator>(&mut self) -> Option<T> {
            T::generate(self)
        }

        fn gen_char(&mut self, min: Bound<&char>, max: Bound<&char>) -> Option<char> {
            let value = unsafe { rmc::nondet() };
            let value = core::char::from_u32(value);
            rmc::assume(value.is_some());
            let value = value.unwrap();
            rmc::assume((min, max).contains(&value));
            Some(value)
        }

        fn gen_bool(&mut self, _probability: Option<f32>) -> Option<bool> {
            Some(unsafe { rmc::nondet() })
        }
    }
}

#[doc(hidden)]
#[cfg(any(all(feature = "lib", fuzzing_rmc), all(feature = "lib", rmc)))]
pub use lib::*;
