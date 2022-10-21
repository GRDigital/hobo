# Children and Parent

Hierarchical DOM relations in `hobo` are maintained through regular **Components** - `hobo::Children` and `hobo::Parent`.   

Usually you won't have to care about it since `.add_child()` (and the like) and `.remove()` already take care of updating `Children` and `Parent` components of affected entities.

`hobo::Children` is just a `Vec` of `hobo::Entity`, `hobo::Parent` is a newtype wrapper over `hobo::Entity` as well. If you have an **Element** and you want to operate on all (or some) of its children - it's as simple as:

```rust,noplaypen
let children = foo.get_cmp::<hobo::Children>()
    .iter()
    .map(|entity| hobo::SomeElement(entity));

for child in children {
    child.set_text("hello from hobo!");
}
```

It is possible to detach a child from its parent to reattach it to a different **Element** later, but it's not as simple as removing `hobo::Parent` and fixing up `hobo::Children` of the parent entity since the DOM has to be modified as well. A convenient method exists however:

```rust,noplaypen
// this removes parent and fixes children component in parent as well
some_child.leave_parent();
new_parent.add_child(some_child);
```
