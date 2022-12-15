//! # DBus interface proxies for: `org.a11y.atspi.Event.Object`, `org.a11y.atspi.Event.Window`, `org.a11y.atspi.Event.Mouse`, `org.a11y.atspi.Event.Keyboard`, `org.a11y.atspi.Event.Terminal`, `org.a11y.atspi.Event.Document`, `org.a11y.atspi.Event.Focus`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Event.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

pub mod document;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod object;
pub mod terminal;
pub mod window;

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use zbus::{
    names::{InterfaceName, MemberName, UniqueName},
    zvariant::{self, OwnedObjectPath, OwnedValue, Signature, Type, Value},
    Message,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventBody<'a, T> {
    #[serde(rename = "type")]
    pub kind: T,
    pub detail1: i32,
    pub detail2: i32,
    #[serde(borrow)]
    pub any_data: Value<'a>,
    #[serde(borrow)]
    pub properties: HashMap<&'a str, Value<'a>>,
}

impl<T> Type for EventBody<'_, T> {
    fn signature() -> Signature<'static> {
        <(&str, i32, i32, Value, HashMap<&str, Value>)>::signature()
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyQT {
    #[serde(rename = "type")]
    pub kind: String,
    pub detail1: i32,
    pub detail2: i32,
    pub any_data: OwnedValue,
    pub properties: (String, OwnedObjectPath),
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct EventBodyOwned {
    #[serde(rename = "type")]
    pub kind: String,
    pub detail1: i32,
    pub detail2: i32,
    pub any_data: OwnedValue,
    pub properties: HashMap<String, OwnedValue>,
}

impl From<EventBodyQT> for EventBodyOwned {
    fn from(body: EventBodyQT) -> Self {
        let mut props = HashMap::new();
        props.insert(
            body.properties.0,
            Value::ObjectPath(body.properties.1.into_inner()).to_owned(),
        );
        Self {
            kind: body.kind,
            detail1: body.detail1,
            detail2: body.detail2,
            any_data: body.any_data,
            properties: props,
        }
    }
}

pub struct Event {
    message: Arc<Message>,
    body: EventBodyOwned,
}

impl TryFrom<Arc<Message>> for Event {
    type Error = zbus::Error;

    fn try_from(message: Arc<Message>) -> zbus::Result<Self> {
        let qt_sig = Signature::try_from("siiv(so)").unwrap();
        let body: EventBodyOwned = match message.body_signature() {
            Ok(sig) => {
                if sig == qt_sig {
                    EventBodyOwned::from(message.body::<EventBodyQT>()?)
                } else {
                    message.body::<EventBodyOwned>()?
                }
            }
            Err(e) => return Err(e),
        };
        Ok(Self { message, body })
    }
}

// TODO Complete list
// The signal signatures found on the a11y bus.
const QT_EVENT: Signature<'static> = Signature::try_from("siiv(so)").unwrap();
const EVENT_SIG: Signature<'static> = Signature::try_from("siiva{sv}").unwrap();
const CACHE_ADD: Signature<'static> = Signature::try_from("(so)(so)(so)iiassusau").unwrap();
const CACHE_REM: Signature<'static> = Signature::try_from("so").unwrap();

 
// TODO complete match
impl TryFrom<Arc<Message>> for AtspiEvent {
    type Error = zbus::Error;

    fn try_from(message: Arc<Message>) -> zbus::Result<Self> {
        let body: EventBodyOwned = match message.body_signature()? {
            QT_EVENT => {
                let body = EventBodyOwned::from(message.body::<EventBodyQT>()?);
                let atspi_event: AtspiEvent = body.try_into()?;
            }
            EVENT_SIG => {
                let body = message.body::<EventBodyOwned>()?;
                let atspi_event: AtspiEvent = body.try_into()?;
            }
            CACHE_ADD => {todo!()},
            CACHE_REM => {todo!()},

        };
        Ok(Self { message, body })
    }
}

// we may want to have this live somewhere else
/// Compound type that aggregates a11y bus events.
#[derive(Clone, Debug, Eq)]
pub enum AtspiEvent {
    // Processed
        AddAccessible(()),
        RemoveAccessible(()),
        EventListenerRegistered(()),
        EventListenerDeregistered(()),
    // Registry
        EventListenerRegistered(()),
        EventListenerDeregistered(()),
    // DeviceEventListener
        KeystrokeListenerRegistered(()),
        KeystrokeListenerDeregistered(()),
    // Event
        PropertyChange(()),
        BoundsChanged(()),
        LinkSelected(()),
        StateChanged(()),
        ChildrenChanged(()),
        VisibleDataChanged(()),
        SelectionChanged(()),
        ModelChanged(()),
        ActiveDescendantChanged(()),
        Announcement(()),
        AttributesChanged(()),
        RowInserted(()),
        RowReordered(()),
        RowDeleted(()),
        ColumnInserted(()),
        ColumnReordered(()),
        ColumnDeleted(()),
        TextBoundsChanged(()),
        TextSelectionChanged(()),
        TextChanged(()),
        TextAttributesChanged(()),
        TextCaretMoved(()),
        PropertyChange(()),
        Minimize(()),
        Maximize(()),
        Restore(()),
        Close(()),
        Create(()),
        Reparent(()),
        DesktopCreate(()),
        DesktopDestroy(()),
        Destroy(()),
        Activate(()),
        Deactivate(()),
        Raise(()),
        Lower(()),
        Move(()),
        Resize(()),
        Shade(()),
        uUshade(()),
        Restyle(()),
        Abs(()),
        Rel(()),
        Button(()),
        Modifiers(()),
        LineChanged(()),
        ColumncountChanged(()),
        LinecountChanged(()),
        ApplicationChanged(()),
        CharwidthChanged(()),
        LoadComplete(()),
        Reload(()),
        LoadStopped(()),
        ContentChanged(()),
        AttributesChanged(()),
        PageChanged(()),
        Focus(()),
    //  Cache.xml
        AddAccessible(()),
        RemoveAccessible(()),
    // Socket    
        Available(()),
}


impl TryFrom<EventBodyOwned> for At



impl Event {
    /// Identifies the `sender` of the `Event`.
    /// # Errors
    /// - when deserializeing the header failed, or
    /// - When `zbus::get_field!` finds that 'sender' is an invalid field.
    pub fn sender(&self) -> zbus::Result<Option<UniqueName>> {
        Ok(self.message.header()?.sender()?.cloned())
    }

    /// The object path to the object where the signal is emitted from.
    #[must_use]
    pub fn path(&self) -> Option<zvariant::ObjectPath> {
        self.message.path()
    }

    /// Returns the atspi event string for this event type (E.G. "Object:StateChanged:Focused").
    ///
    /// This should not be used for matching on events as it needlessly allocates and copies the 3
    /// components of the event type. It is meant for logging, etc.
    #[must_use]
    pub fn event_string(&self) -> String {
        let interface = self.interface().expect("Event should have an interface");
        let interface = interface.rsplit('.').next().expect("Interface should contain a '.'");
        let member = self.member().expect("Event should have a member");
        let kind = self.kind();
        format!("{interface}:{member}:{kind}")
    }

    /// For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
    /// wrong such that the `&str` you can get from a
    /// [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
    /// name, not to the lifetime of the message as it should be. In future, this will return only
    /// the last component of the interface name (I.E. "Object" from
    /// "org.a11y.atspi.Event.Object").
    #[must_use]
    pub fn interface(&self) -> Option<InterfaceName<'_>> {
        self.message.interface()
    }

    // Identifies this `Event`'s member (signal-) name on the bus.
    #[must_use]
    pub fn member(&self) -> Option<MemberName<'_>> {
        self.message.member()
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.body.kind
    }

    #[must_use]
    pub fn detail1(&self) -> i32 {
        self.body.detail1
    }

    #[must_use]
    pub fn detail2(&self) -> i32 {
        self.body.detail2
    }

    #[must_use]
    pub fn any_data(&self) -> &zvariant::OwnedValue {
        &self.body.any_data
    }

    #[must_use]
    pub fn properties(&self) -> &HashMap<String, zvariant::OwnedValue> {
        &self.body.properties
    }

    #[must_use]
    pub fn message(&self) -> &Arc<Message> {
        &self.message
    }

    #[must_use]
    pub fn body(&self) -> &EventBodyOwned {
        &self.body
    }
}
