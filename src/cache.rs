//! # `DBus` interface proxy for: `org.a11y.atspi.Cache`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Cache.xml`.
//!

use crate::{accessible::Role, InterfaceSet, StateSet};
use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, Type};
use atspi_macros::atspi_proxy;
use async_trait::async_trait;

/// The item type provided by `Cache:Add` signals
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct CacheItem {
    /// The accessible object (within the application)   (so)
    pub object: (String, OwnedObjectPath),
    /// The application (root object(?)    (so)
    pub app: (String, OwnedObjectPath),
    /// The parent object.  (so)
    pub parent: (String, OwnedObjectPath),
    /// The accessbile index in parent.  i
    pub index: i32,
    /// Child count of the accessible  i
    pub children: i32,
    /// The exposed interfece(s) set.  as
    pub ifaces: InterfaceSet,
    /// The short localized name.  s
    pub short_name: String,
    /// Accessible role. u
    pub role: Role,
    /// More detailed localized name.
    pub name: String,
    /// The states applicable to the accessible.  au
    pub states: StateSet,
}

#[test]
fn zvariant_type_signature_of_cache_item() {
    assert_eq!(
        CacheItem::signature(),
        zbus::zvariant::Signature::from_static_str("((so)(so)(so)iiassusau)").unwrap()
    );
}

// impl CacheItem {
//     fn accessible(&self, conn: &Connection) -> AccessibleProxy<'_> {
//         let conn = conn.inner().connection();
//         let (name, path) = (self.object.0, self.object.1);
//         ProxyBuilder::new(conn)
//     }
// }
//

#[atspi_proxy(interface = "org.a11y.atspi.Cache", default_path = "/org/a11y/atspi/cache")]
trait Cache {
    /// GetItems method
    fn get_items(&self) -> zbus::Result<Vec<CacheItem>>;

    /// AddAccessible signal
    #[dbus_proxy(signal)]
    fn add_accessible(&self, node_added: CacheItem) -> zbus::Result<()>;

    /// RemoveAccessible signal
    #[dbus_proxy(signal)]
    fn remove_accessible(&self, node_removed: (String, OwnedObjectPath)) -> zbus::Result<()>;
}
use crate::{AtspiProxy, Interface};
impl<'a> AtspiProxy for CacheProxy<'a> {
    const INTERFACE: Interface = Interface::Cache;
}
