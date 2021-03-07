use crate::imp::{core::*, page::Page as PageImpl, prelude::*};

pub struct Keyboard {
    inner: Weak<PageImpl>
}

pub struct Mouse {}

pub struct TouchScreen {
    inner: Weak<PageImpl>
}

impl Keyboard {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn down<'a>(&self, key: &'a str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_down(key).await
    }

    pub async fn up<'a>(&self, key: &'a str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_up(key).await
    }

    pub async fn r#type<'b>(&self, text: &'b str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_type(text, delay).await
    }

    pub async fn press<'b>(&self, key: &'b str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_press(key, delay).await
    }
}

impl TouchScreen {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn tap<'a>(&self, x: f64, y: f64) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.screen_tap(x, y).await
    }
}
