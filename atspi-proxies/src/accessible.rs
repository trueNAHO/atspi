//! # `DBus` interface proxy for: `org.a11y.atspi.Accessible`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Accessible.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use crate::atspi_proxy;
use crate::AtspiError;
use atspi_common::{InterfaceSet, ObjectPair, RelationType, Role, StateSet};

#[atspi_proxy(interface = "org.a11y.atspi.Accessible", assume_defaults = true)]
trait Accessible {
	/// GetApplication method
	fn get_application(&self) -> zbus::Result<ObjectPair>;

	/// GetAttributes method
	fn get_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// GetChildAtIndex method
	fn get_child_at_index(&self, index: i32) -> zbus::Result<ObjectPair>;

	/// GetChildren method
	fn get_children(&self) -> zbus::Result<Vec<ObjectPair>>;

	/// GetIndexInParent method; this will give an index between 0 and n, where n is the number of children in the parent.
	fn get_index_in_parent(&self) -> zbus::Result<i32>;

	/// GetInterfaces method
	fn get_interfaces(&self) -> zbus::Result<InterfaceSet>;

	/// GetLocalizedRoleName method
	fn get_localized_role_name(&self) -> zbus::Result<String>;

	/// GetRelationSet method
	fn get_relation_set(&self) -> zbus::Result<Vec<(RelationType, Vec<ObjectPair>)>>;

	/// GetRole method
	fn get_role(&self) -> zbus::Result<Role>;

	/// GetRoleName method
	fn get_role_name(&self) -> zbus::Result<String>;

	/// GetState method
	fn get_state(&self) -> zbus::Result<StateSet>;

	/// AccessibleId property
	#[dbus_proxy(property)]
	//fn accessible_id(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
	fn accessible_id(&self) -> zbus::Result<String>;

	/// ChildCount property
	#[dbus_proxy(property)]
	fn child_count(&self) -> zbus::Result<i32>;

	/// Description property
	#[dbus_proxy(property)]
	fn description(&self) -> zbus::Result<String>;

	/// Locale property
	#[dbus_proxy(property)]
	fn locale(&self) -> zbus::Result<String>;

	/// Name property
	#[dbus_proxy(property)]
	fn name(&self) -> zbus::Result<String>;

	/// Parent property
	#[dbus_proxy(property)]
	fn parent(&self) -> zbus::Result<ObjectPair>;
}

impl TryFrom<AccessibleProxy<'_>> for ObjectPair {
	type Error = AtspiError;
	fn try_from(proxy: AccessibleProxy<'_>) -> Result<ObjectPair, Self::Error> {
		Ok((proxy.destination().to_string(), proxy.path().to_string().try_into()?))
	}
}
impl TryFrom<&AccessibleProxy<'_>> for ObjectPair {
	type Error = AtspiError;
	fn try_from(proxy: &AccessibleProxy<'_>) -> Result<ObjectPair, Self::Error> {
		Ok((proxy.destination().to_string(), proxy.path().to_string().try_into()?))
	}
}

impl PartialEq for AccessibleProxy<'_> {
	fn eq<'a>(&self, other: &Self) -> bool {
		self.path() == other.path() //&& self.destination() == other.destination()
	}
}
impl Eq for AccessibleProxy<'_> {}

#[cfg(test)]
mod tests {
	use crate::accessible::Role;

	#[test]
	fn test_output_of_role_name() {
		assert_eq!(Role::Invalid.name(), "invalid");
		assert_eq!(Role::PushButtonMenu.name(), "push button menu");
	}
}
