use crate::prelude::*;

/// A component with a single `Entity` representing the parent
#[derive(Default, Debug, shrinkwraprs::Shrinkwrap, Clone, Copy, PartialEq, Eq, Hash, AsEntity)]
#[shrinkwrap(mutable)]
pub struct Parent(pub Entity);

/// A component with a simple `Vec<Entity>` of child entities
#[derive(Default, Debug, shrinkwraprs::Shrinkwrap, Clone, PartialEq, Eq, Hash)]
#[shrinkwrap(mutable)]
pub struct Children(pub Vec<Entity>);

impl Parent {
	pub fn ancestors(entity: impl AsEntity) -> Vec<Entity> {
		fn inner(entity: Entity, ancestors: &mut Vec<Entity>) {
			if let Some(parent) = entity.try_get_cmp::<Parent>().map(|x| x.0) {
				ancestors.push(parent);
				inner(parent, ancestors);
			}
		}

		let mut v = Vec::new();
		inner(entity.as_entity(), &mut v);
		v
	}
}

impl Children {
	pub fn descendants(entity: impl AsEntity) -> Vec<Entity> {
		fn inner(entity: Entity, descendants: &mut Vec<Entity>) {
			if let Some(children) = entity.try_get_cmp::<Children>() {
				descendants.extend_from_slice(&children.0);
				for &child in &children.0 {
					inner(child, descendants);
				}
			}
		}

		let mut v = Vec::new();
		inner(entity.as_entity(), &mut v);
		v
	}

	pub fn clear(entity: impl AsEntity) {
		if let Some(children) = entity.try_get_cmp_mut::<Children>().map(|mut x| x.0.drain(..).collect::<Vec<_>>()) {
			for child in children {
				child.remove();
			}
		}
	}
}
