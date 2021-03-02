use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Selectors {
    channel: ChannelOwner
}

impl Selectors {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }

    pub(crate) async fn register(
        &self,
        name: &str,
        script: &str,
        is_content_script: bool
    ) -> Result<Rc<ResponseResult>, Rc<ConnectionError>> {
        let mut p = Map::<String, Value>::default();
        p.insert("name".into(), name.into());
        p.insert("script".into(), script.into());
        p.insert("contentScript".into(), is_content_script.into());
        let r = self
            .channel()
            .create_request("register".to_owned().try_into().unwrap())
            .set_params(p);
        let fut = self.channel().send_message(r).await?;
        fut.await
    }
}

impl RemoteObject for Selectors {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::{core::*, prelude::*};
    use std::env;

    crate::runtime_test!(register, {
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let conn = driver.run().await.unwrap();
        let p = Connection::wait_initial_object(Rc::downgrade(&conn))
            .await
            .unwrap();
        let p = p.upgrade().unwrap();
        let s: Rc<Selectors> = p.selectors.upgrade().unwrap();
        let fut = s.register("foo", "()", false);
        log::trace!("fut");
        let res = fut.await;
        dbg!(&res);
        assert!(res.is_err());
    });
}
