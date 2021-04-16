use crate::imp::{core::*, js_handle::JsHandle as Impl, prelude::*};
use std::fmt;

/// JsHandle represents an in-page JavaScript object. JsHandles can be created with the [`method: Page.evaluateHandle`]
/// method.
///
/// ```js
/// const windowHandle = await page.evaluateHandle(() => window);
///// ...
/// ```
/// 
/// JsHandle prevents the referenced JavaScript object being garbage collected unless the handle is exposed with
/// [`method: JsHandle.dispose`]. JsHandles are auto-disposed when their origin frame gets navigated or the parent context
/// gets destroyed.
///
/// JsHandle instances can be used as an argument in [`method: Page.evalOnSelector`], [`method: Page.evaluate`] and
/// [`method: Page.evaluateHandle`] methods.
pub struct JsHandle {
    inner: Weak<Impl>
}

impl PartialEq for JsHandle {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl JsHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub(crate) fn guid(&self) -> Result<Str<Guid>, Error> {
        Ok(upgrade(&self.inner)?.guid().to_owned())
    }

    /// Fetches a single property from the referenced object.
    pub async fn get_property(&mut self, name: &str) -> ArcResult<JsHandle> {
        upgrade(&self.inner)?
            .get_property(name)
            .await
            .map(JsHandle::new)
    }

    /// The method returns a map with **own property names** as keys and JsHandle instances for the property values.
    ///
    /// ```js
    /// const handle = await page.evaluateHandle(() => ({window, document}));
    /// const properties = await handle.getProperties();
    /// const windowHandle = properties.get('window');
    /// const documentHandle = properties.get('document');
    /// await handle.dispose();
    /// ```
    pub async fn get_properties(&mut self) -> ArcResult<HashMap<String, JsHandle>> {
        let m = upgrade(&self.inner)?.get_properties().await?;
        Ok(m.into_iter().map(|(k, v)| (k, JsHandle::new(v))).collect())
    }

    pub async fn dispose(&mut self) -> ArcResult<()> { upgrade(&self.inner)?.dispose().await }

    /// Returns a JSON representation of the object. If the object has a `toJSON` function, it **will not be called**.
    ///
    /// > NOTE: The method will return an empty JSON object if the referenced object is not stringifiable. It will throw an
    /// error if the object has circular references.
    pub async fn json_value<U>(&mut self) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        upgrade(&self.inner)?.json_value().await
    }

    // evaluate
}

impl fmt::Display for JsHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner) = self.inner.upgrade() {
            inner.fmt(f)
        } else {
            write!(f, "")
        }
    }
}

mod ser {
    use super::*;
    use serde::{ser, ser::SerializeStruct};

    impl Serialize for JsHandle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer
        {
            let mut s = serializer.serialize_struct("4a9c3811-6f00-49e5-8a81-939f932d9061", 1)?;
            let guid = &self.guid().map_err(<S::Error as ser::Error>::custom)?;
            s.serialize_field("guid", &guid)?;
            s.end()
        }
    }
}
