//! This module contains the `MultiRef` type which is a wrapper around `Arc<RwLock<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
//! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
//! Needs `xml-rs`, `tokio` and `yaserde` as dependencies.

use std::{ops::Deref, sync::Arc};
use tokio::sync::RwLock;
use yaserde::{YaDeserialize, YaSerialize};

pub struct MultiRef<T> {
    inner: Arc<RwLock<T>>,
}

impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
    fn deserialize<R: std::io::prelude::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        let inner = T::deserialize(reader)?;
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
        })
    }
}

impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
    fn serialize<W: std::io::prelude::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        self.inner.blocking_write().serialize(writer)?;
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        self.inner
            .blocking_read()
            .serialize_attributes(attributes, namespace)
    }
}

impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
    fn default() -> Self {
        Self {
            inner: Arc::default(),
        }
    }
}

impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.blocking_read().fmt(f)
    }
}

impl<T> Deref for MultiRef<T> {
    type Target = Arc<RwLock<T>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
