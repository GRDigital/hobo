# Removing and replacing elements

Removing an **Element** (or an **Entity**) is as simple as calling `.remove()`. The method will recursively remove all entities in `hobo::Children` of the entity to be removed as well. All components that have been added to entities that are being removed are also removed and dropped.    

It is possible to replace an **Element** inplace, fixing up the `hobo::Children` in parent entity as well. This, however, replaces it with a new **Entity** so if a copy is held somewhere - it won't be valid anymore, so take care.

```rust,noplaypen
let new_element = hobo::create::div();
old_element.replace_with(new_element);
```
