mod imp;

mod api;
pub use api::*;

#[cfg_attr(test, macro_export)]
macro_rules! runtime_test {
    ($name:tt, $main:stmt) => {
        #[test]
        fn $name() {
            actix_rt::System::new().block_on(async { $main });
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { $main });
            async_std::task::block_on(async { $main });
        }
    };
}
