use crate::{
	accessible::{
		Accessible, AccessibleBlocking, AccessibleProxy, AccessibleProxyBlocking, RelationType,
		Role,
	},
	collection::MatchType,
	convertable::{Convertable, ConvertableBlocking},
	hyperlink::Hyperlink,
	text::{Text, TextBlocking},
	InterfaceSet,
};
use async_trait::async_trait;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatcherArgs {
	roles: Vec<Role>,
	role_mt: MatchType,
	attr: HashMap<String, String>,
	attr_mt: MatchType,
	ifaces: InterfaceSet,
	ifaces_mt: MatchType,
}

#[async_trait]
pub trait AccessibleExt {
	type Error: std::error::Error;
	async fn get_parent_ext<'a>(&self) -> Result<Self, Self::Error>
	where
		Self: Sized;
	async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_children_indexes<'a>(&self) -> Result<Vec<i32>, Self::Error>;
	async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_children_caret<'a>(&self, after: bool) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	/* TODO: not sure where these should go since it requires both Text as a self interface and
	 * Hyperlink as children interfaces. */
	async fn get_next<'a>(
		&self,
		matcher_args: &MatcherArgs,
		backward: bool,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_relation_set_ext<'a>(
		&self,
	) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error>
	where
		Self: Sized;
	async fn find_inner<'a>(
		&self,
		after_or_before: i32,
		matcher_args: &MatcherArgs,
		backward: bool,
		recur: bool,
	) -> Result<Option<Self>, <Self as AccessibleExt>::Error>
	where
		Self: Sized;
	async fn match_(
		&self,
		matcher_args: &MatcherArgs,
	) -> Result<bool, <Self as AccessibleExt>::Error>;
}
// TODO: implement AccessibleExt
pub trait AccessibleBlockingExt {}

#[allow(clippy::module_name_repetitions)]
pub trait AccessibleExtError: Accessible + Convertable {
	type Error: std::error::Error
		+ From<<Self as Accessible>::Error>
		+ From<<Self as Convertable>::Error>
		// TODO: add all convertable error types
		+ From<<<Self as Convertable>::Text as Text>::Error>
		+ From<std::num::TryFromIntError>
		+ Send;
}

#[allow(clippy::module_name_repetitions)]
pub trait AccessibleBlockingExtError: AccessibleBlocking + ConvertableBlocking {
	type Error: std::error::Error
		+ From<<Self as AccessibleBlocking>::Error>
		+ From<<Self as ConvertableBlocking>::Error>
		// TODO: add all convertable error types
		+ From<<<Self as ConvertableBlocking>::Text as TextBlocking>::Error>
		+ From<std::num::TryFromIntError>;
}

#[async_trait]
impl<T: Accessible + Convertable> AccessibleExt for T {
	type Error = crate::AtspiError;

	/// Retrieves children with their respective indexes in parent, the current `Accessible`.
	async fn get_indexed_children<'a>(&self) -> Result<Vec<(i32, Self)>, Self::Error> {
		let children = self.get_children().await?;
		let mut indexed: Vec<(i32, Self)> = Vec::with_capacity(children.len());
		for child in children {
			let idx = child.get_index_in_parent().await?;
			indexed.push((idx, child));
		}
		Ok(indexed)
	}
	async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		Ok(self.get_children().await?)
		/*
		let children_parts = self.get_children().await?;
		let mut children = Vec::new();
		for child_parts in children_parts {
			let acc = AccessibleProxy::builder(self.connection())
				.destination(child_parts.0)?
				.cache_properties(CacheProperties::No)
				.path(child_parts.1)?
				.build()
				.await?;
			children.push(acc);
		}
		Ok(children)
				*/
	}
	async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let index = self.get_index_in_parent().await?.try_into()?;
		// Clippy false positive: Standard pattern for excluding index item from list.
		#[allow(clippy::if_not_else)]
		let children: Vec<Self> = parent
			.get_children()
			.await?
			.into_iter()
			.enumerate()
			.filter_map(|(i, a)| if i != index { Some(a) } else { None })
			.collect();
		Ok(children)
	}
	async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let index = self.get_index_in_parent().await?.try_into()?;
		let children: Vec<Self> = parent
			.get_children_ext()
			.await?
			.into_iter()
			.enumerate()
			.filter_map(|(i, a)| if i < index { Some(a) } else { None })
			.collect();
		Ok(children)
	}
	async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let index = self.get_index_in_parent().await?.try_into()?;
		let children: Vec<Self> = parent
			.get_children_ext()
			.await?
			.into_iter()
			.enumerate()
			.filter_map(|(i, a)| if i > index { Some(a) } else { None })
			.collect();
		Ok(children)
	}

	/// Return 'hyperlink children' before and after the caret position, if hyperlink child objects exist.
	///
	/// If hyperlink objects'  'IndexAtStart' property does not yield a value, then these children are found in a third vector.
	///
	/// # Errors
	/// This function assumes all children of a text-object are hyperlinks.
	/// If that assumption proves false it yields an Error.
	async fn get_hyperlinks_adjacent_to_caret<'a>(&self) -> Result<[Vec<Self>; 3], Self::Error>
	where
		Self: Sized,
	{
		// If 'self' is text-object, get caret-offset, or bail.
		let text_iface = self.to_text().await?;
		let caret_pos = text_iface.caret_offset().await?;

		// Collect the text-object's children.
		// Collect in either of three categories:
		let mut lhs = Vec::new();
		let mut rhs = Vec::new();
		let mut unknown = Vec::new();

		// This function assumes text-object children are hyperlinks.
		// If they are not, we would like to know.
		let text_children = self.get_children_ext().await?;
		for child in text_children {
			// If any of the children is not a hyperlink, we bail.
			let hyperlink = child.to_hyperlink().await?;
			match hyperlink.start_index().await {
				Ok(start_idx) if start_idx <= caret_pos => lhs.push(child),
				Ok(start_idx) if start_idx >= caret_pos => rhs.push(child),
				_ => unknown.push(child),
			}
		}

		Ok([lhs, rhs, unknown])
	}
	async fn get_next<'a>(
		&self,
		matcher_args: &MatcherArgs,
		backward: bool,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized,
	{
		// TODO if backwards, check here
		let caret_children = self.get_children_caret(backward).await?;
		for child in caret_children {
			if child.match_(matcher_args).await? {
				return Ok(Some(child));
			} else if let Some(found_sub) =
				child.find_inner(0, matcher_args, backward, true).await?
			{
				return Ok(Some(found_sub));
			}
		}
		let mut last_parent_index = self.get_index_in_parent().await?;
		if let Ok(mut parent) = self.get_parent_ext().await {
			while parent.get_role().await? != Role::InternalFrame {
				let found_inner_child = parent
					.find_inner(last_parent_index, matcher_args, backward, false)
					.await?;
				if found_inner_child.is_some() {
					return Ok(found_inner_child);
				}
				last_parent_index = parent.get_index_in_parent().await?;
				parent = parent.get_parent_ext().await?;
			}
		}
		Ok(None)
	}
	async fn get_relation_set_ext<'a>(
		&self,
	) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error>
	where
		Self: Sized,
	{
		let raw_relations = self.get_relation_set().await?;
		let mut relations = HashMap::new();
		for relation in raw_relations {
			let mut related_vec = Vec::new();
			for related in relation.1 {
				related_vec.push(related);
			}
			relations.insert(relation.0, related_vec);
		}
		Ok(relations)
	}
	async fn find_inner<'a>(
		&self,
		before_or_after: i32,
		matcher_args: &MatcherArgs,
		backward: bool,
		recur: bool,
	) -> Result<Option<Self>, <Self as AccessibleExt>::Error>
	where
		Self: Sized,
	{
		let children = if backward {
			let mut vec = self.get_children_ext().await?;
			vec.reverse();
			vec
		} else {
			self.get_children_ext().await?
		};
		for child in children {
			let child_index = child.get_index_in_parent().await?;
			if !recur
				&& ((child_index <= before_or_after && !backward)
					|| (child_index >= before_or_after && backward))
			{
				continue;
			}
			if child.match_(matcher_args).await? {
				return Ok(Some(child));
			}
			/* 0 here is ignored because we are recursive; see the line starting with if !recur */
			if let Some(found_descendant) =
				child.find_inner(0, matcher_args, backward, true).await?
			{
				return Ok(Some(found_descendant));
			}
		}
		Ok(None)
	}
	// TODO: make match more broad, allow use of other parameters; also, support multiple roles, since right now, multiple will just exit immediately with false
	async fn match_(
		&self,
		matcher_args: &MatcherArgs,
	) -> Result<bool, <Self as AccessibleExt>::Error> {
		let roles = &matcher_args.roles;
		if roles.len() != 1 {
			return Ok(false);
		}
		// our unwrap is protected from panicing with the above check
		Ok(self.get_role().await? == *roles.get(0).unwrap())
	}
}

impl<T: AccessibleBlocking + ConvertableBlocking + AccessibleBlockingExtError> AccessibleBlockingExt
	for T
{
}

assert_impl_all!(AccessibleProxy: Accessible, AccessibleExt);
assert_impl_all!(AccessibleProxyBlocking: AccessibleBlocking, AccessibleBlockingExt);
