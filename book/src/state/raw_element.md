# Raw element access

Most simple event handlers, like clicks or hovers, usually just modify either style or contents of html elements in a way which doesn't not require any state manipulation apart from setting attributes or changing text. In this case, it's convenient to clone raw element reference into our `e!()` handlers.

```rust
e!((*foo) move || {
    foo.set_inner_text("wooo");
})
```

would turn into something like

```rust
{
    // this clone is only of an html element reference,
    // it doesn't actually clone the element
    let foo = foo.raw_element().clone();
    move || {
        foo.set_inner_text("wooo");
    }
}
```
