//! # DBus interface proxy for: `org.a11y.atspi.Selection`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Selection.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Selection", assume_defaults = true)]
trait Selection {
    /// ClearSelection method
    fn clear_selection(&self) -> zbus::Result<bool>;

    /// DeselectChild method
    fn deselect_child(&self, child_index: i32) -> zbus::Result<bool>;

    /// DeselectSelectedChild method
    fn deselect_selected_child(&self, selected_child_index: i32) -> zbus::Result<bool>;

    /// GetSelectedChild method
    fn get_selected_child(
        &self,
        selected_child_index: i32,
    ) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;

    /// IsChildSelected method
    fn is_child_selected(&self, child_index: i32) -> zbus::Result<bool>;

    /// SelectAll method
    fn select_all(&self) -> zbus::Result<bool>;

    /// SelectChild method
    fn select_child(&self, child_index: i32) -> zbus::Result<bool>;

    /// NSelectedChildren property
    #[dbus_proxy(property)]
    fn nselected_children(&self) -> zbus::Result<i32>;
}
