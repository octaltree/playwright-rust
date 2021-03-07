use crate::imp::{
    self,
    core::*,
    frame::{Frame as Impl, GotoArgs},
    prelude::*,
    utils::DocumentLoadState
};

pub struct Frame {
    inner: Weak<Impl>
}

impl Frame {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn goto_builder<'a>(&mut self, url: &'a str) -> GotoBuilder<'a, '_> {
        GotoBuilder::new(self.inner.clone(), url)
    }
}

pub struct GotoBuilder<'a, 'b> {
    inner: Weak<Impl>,
    args: GotoArgs<'a, 'b>
}

impl<'a, 'b> GotoBuilder<'a, 'b> {
    fn new(inner: Weak<Impl>, url: &'a str) -> Self {
        let args = GotoArgs::new(url);
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.goto(args).await?;
        // TODO
        Ok(())
    }

    optional_setter!(
        timeout, f64;
        wait_until, DocumentLoadState;
        referer, &'b str);
}
