//! # `DBus` interface proxy for: `org.a11y.atspi.Text`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Text.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
#![allow(clippy::too_many_arguments)]
// this is to silience clippy due to zbus expanding parameter expressions

use crate::atspi_proxy;
use atspi_common::{Granularity, ClipType, CoordType};

#[atspi_proxy(interface = "org.a11y.atspi.Text", assume_defaults = true)]
trait Text {
	/// AddSelection method
	fn add_selection(&self, start_offset: i32, end_offset: i32) -> zbus::Result<bool>;

	/// GetAttributeRun method
	fn get_attribute_run(
		&self,
		offset: i32,
		include_defaults: bool,
	) -> zbus::Result<(std::collections::HashMap<String, String>, i32, i32)>;

	/// GetAttributeValue method
	fn get_attribute_value(&self, offset: i32, attribute_name: &str) -> zbus::Result<String>;

	/// GetAttributes method
	fn get_attributes(
		&self,
		offset: i32,
	) -> zbus::Result<(std::collections::HashMap<String, String>, i32, i32)>;

	/// GetBoundedRanges method
	fn get_bounded_ranges(
		&self,
		x: i32,
		y: i32,
		width: i32,
		height: i32,
		coord_type: CoordType,
		x_clip_type: ClipType,
		y_clip_type: ClipType,
	) -> zbus::Result<Vec<(i32, i32, String, zbus::zvariant::OwnedValue)>>;

	/// GetCharacterAtOffset method
	fn get_character_at_offset(&self, offset: i32) -> zbus::Result<i32>;

	/// GetCharacterExtents method
	fn get_character_extents(
		&self,
		offset: i32,
		coord_type: CoordType,
	) -> zbus::Result<(i32, i32, i32, i32)>;

	/// GetDefaultAttributeSet method
	fn get_default_attribute_set(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// GetDefaultAttributes method
	fn get_default_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// GetNSelections method
	fn get_nselections(&self) -> zbus::Result<i32>;

	/// GetOffsetAtPoint method
	fn get_offset_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<i32>;

	/// GetRangeExtents method
	fn get_range_extents(
		&self,
		start_offset: i32,
		end_offset: i32,
		coord_type: CoordType,
	) -> zbus::Result<(i32, i32, i32, i32)>;

	/// GetSelection method
	fn get_selection(&self, selection_num: i32) -> zbus::Result<(i32, i32)>;

	/// GetStringAtOffset method
	fn get_string_at_offset(
		&self,
		offset: i32,
		granularity: Granularity,
	) -> zbus::Result<(String, i32, i32)>;

	/// GetText method
	fn get_text(&self, start_offset: i32, end_offset: i32) -> zbus::Result<String>;

	/// GetTextAfterOffset method
	fn get_text_after_offset(&self, offset: i32, type_: u32) -> zbus::Result<(String, i32, i32)>;

	/// GetTextAtOffset method
	fn get_text_at_offset(&self, offset: i32, type_: u32) -> zbus::Result<(String, i32, i32)>;

	/// GetTextBeforeOffset method
	fn get_text_before_offset(&self, offset: i32, type_: u32) -> zbus::Result<(String, i32, i32)>;

	/// RemoveSelection method
	fn remove_selection(&self, selection_num: i32) -> zbus::Result<bool>;

	/// ScrollSubstringTo method
	fn scroll_substring_to(
		&self,
		start_offset: i32,
		end_offset: i32,
		type_: u32,
	) -> zbus::Result<bool>;

	/// ScrollSubstringToPoint method
	fn scroll_substring_to_point(
		&self,
		start_offset: i32,
		end_offset: i32,
		type_: u32,
		x: i32,
		y: i32,
	) -> zbus::Result<bool>;

	/// SetCaretOffset method
	fn set_caret_offset(&self, offset: i32) -> zbus::Result<bool>;

	/// SetSelection method
	fn set_selection(
		&self,
		selection_num: i32,
		start_offset: i32,
		end_offset: i32,
	) -> zbus::Result<bool>;

	/// CaretOffset property
	#[dbus_proxy(property)]
	fn caret_offset(&self) -> zbus::Result<i32>;

	/// CharacterCount property
	#[dbus_proxy(property)]
	fn character_count(&self) -> zbus::Result<i32>;
}
