#[macro_use]
extern crate serde;

mod imp;

mod api;
pub use api::*;

#[cfg_attr(test, macro_export)]
macro_rules! runtime_test {
    ($name:tt, $main:stmt) => {
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            log::trace!("actix");
            actix_rt::System::new().block_on(async { $main });
            log::trace!("tokio");
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { $main });
            // log::trace!("async_std");
            // async_std::task::block_on(async { $main });
        }
    };
}
