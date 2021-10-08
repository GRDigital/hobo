use crate::prelude::*;

#[derive(Default, Debug, shrinkwraprs::Shrinkwrap, Clone, Copy, PartialEq, Eq, Hash, AsEntity)]
#[shrinkwrap(mutable)]
pub struct Parent(pub Entity);

#[derive(Default, Debug, shrinkwraprs::Shrinkwrap, Clone, PartialEq, Eq, Hash)]
#[shrinkwrap(mutable)]
pub struct Children(pub Vec<Entity>);

impl Parent {
	pub fn ancestors(entity: impl AsEntity) -> Vec<Entity> {
		fn inner(entity: Entity, ancestors: &mut Vec<Entity>) {
			if let Some(parent) = entity.try_get_cmp::<Parent>().map(|x| x.0) {
				inner(parent, ancestors);
				ancestors.push(parent);
			}
		}

		let mut v = Vec::new();
		inner(entity.as_entity(), &mut v);
		v
	}

	#[deprecated]
	pub fn ancestor_with_cmp<T: 'static>(entity: Entity) -> Entity {
		let parent = entity.get_cmp::<Parent>().0;
		if parent.has_cmp::<T>() { parent } else { Parent::ancestor_with_cmp::<T>(parent) }
	}
}

impl Children {
	pub fn descendants(entity: impl AsEntity) -> Vec<Entity> {
		fn inner(entity: Entity, descendants: &mut Vec<Entity>) {
			if let Some(children) = entity.try_get_cmp::<Children>() {
				descendants.copy_from_slice(&children.0);
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

	#[deprecated]
	pub fn collect_with_cmp<T: 'static>(&self) -> Vec<Entity> {
		self.0.iter().filter(|e| e.has_cmp::<T>()).copied().collect::<Vec<_>>()
	}
}
