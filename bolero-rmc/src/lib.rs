//! libfuzzer plugin for bolero
//!
//! This crate should not be used directly. Instead, use `bolero`.

#[doc(hidden)]
#[cfg(any(test, all(feature = "lib", fuzzing_rmc)))]
pub mod lib {
    use bolero_engine::{
        Driver, DriverMode, Engine, TargetLocation, Test, TestInput, TypeGenerator,
    };

    #[derive(Debug, Default)]
    pub struct RmcEngine {
        driver_mode: Option<DriverMode>,
    }

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
            self.driver_mode = Some(mode);
        }

        fn run(self, mut test: T) -> Self::Output {
            let mut input = RmcInput;
            let was_valid = test.test(&mut input).expect("test should pass");
            __VERIFIER_assume(was_valid);
        }
    }

    fn __nondet<T>() -> T {
        todo!()
    }

    #[allow(non_snake_case)]
    fn __VERIFIER_assume(cond: bool) {
        let _ = cond;
        unimplemented!()
    }

    struct RmcInput;

    impl<Output> TestInput<Output> for RmcInput {
        type Driver = RmcDriver;

        fn with_slice<F: FnMut(&[u8]) -> Output>(&mut self, f: &mut F) -> Output {
            f(__nondet())
        }

        fn with_driver<F: FnMut(&mut Self::Driver) -> Output>(&mut self, f: &mut F) -> Output {
            f(&mut RmcDriver)
        }
    }

    struct RmcDriver;

    impl Driver for RmcDriver {
        fn gen<T: TypeGenerator>(&mut self) -> Option<T> {
            Some(__nondet())
        }

        fn mode(&self) -> DriverMode {
            DriverMode::Forced
        }

        fn fill_bytes(&mut self, bytes: &mut [u8]) -> Option<()> {
            bytes.copy_from_slice(__nondet());
            Some(())
        }
    }
}

#[doc(hidden)]
#[cfg(all(feature = "lib", fuzzing_rmc))]
pub use lib::*;
