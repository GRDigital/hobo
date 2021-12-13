# Every way to make a class

Apart from regular `.class()`/`.set_class()` options there's several others:

* `.mark::<T>()`/`.unmark::<T>()` - can generate classes from any type for targeted selection, for an example check [Selector](selector.md)
* `.set_class_tagged::<Tag: Hash>(tag, style)` - generate and assign a class using `tag` as key, the regular `.class()` method uses this internally with just an incrementing `u64` for a tag
* `.set_class_typed::<Type>(style)` - similar to `.set_class_tagged`, but generates the tag from `Type` rather than using an instance of `Type`, rarely but sometimes used if you have a "kind" of class that you want to override
* signals - you can have your classes be set reactively, in response to some changes in a `Mutable`
