#![feature(arc_unwrap_or_clone)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_with;

pub mod api;
mod imp;

pub use crate::imp::core::{Driver, Error};
pub use api::playwright::Playwright;

#[doc(hidden)]
#[macro_export]
macro_rules! runtime_test {
    ($name:tt, $main:stmt) => {
        #[cfg(feature = "rt-tokio")]
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { $main });
        }

        #[cfg(feature = "rt-actix")]
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            actix_rt::System::new().block_on(async { $main });
        }

        #[cfg(feature = "rt-async-std")]
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            async_std::task::block_on(async { $main });
        }
    };
}

pub(crate) mod protocol {
    pub(crate) mod generated;
}
