# SVGs

There is a way to conveniently create inline SVGs without rewriting them manually with `hobo`'s syntax.

```rust,noplaypen
thread_local! {
    static LAST_ID: RefCell<u64> = RefCell::new(0);
}

fn get_svg_element(xml_node: &roxmltree::Node, id: u64) -> web_sys::SvgElement {
    let node: web_sys::SvgElement = wasm_bindgen::JsCast::unchecked_into(document().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), xml_node.tag_name().name()).unwrap());

    for attribute in xml_node.attributes() {
        // need to fixup ids to avoid id collisions in html if the same icon is used multiple times
        if attribute.name() == "id" {
            node.set_attribute(wasm_bindgen::intern(attribute.name()), &format!("{}{:x}", attribute.value(), id)).unwrap();
        } else {
            let mut value = attribute.value().to_owned();
            // optimistic expectation that ids only used in url references
            if value.contains("url(#") {
                value = value.replace(')', &format!("{:x})", id))
            }
            node.set_attribute(wasm_bindgen::intern(attribute.name()), &value).unwrap();
        }
    }

    for child in xml_node.children().filter(roxmltree::Node::is_element) {
        node.append_child(&get_svg_element(&child, id)).unwrap();
    }

    node
}

macro_rules! svg {
    ($($name:ident => $address:expr),*$(,)*) => {$(
        #[must_use]
        pub fn $name() -> e::Svg {
            let id = LAST_ID.with(move |last_id| {
                let mut last_id = last_id.borrow_mut();
                let id = *last_id;
                *last_id += 1;
                id
            });
            let element: web_sys::SvgElement = get_svg_element(&roxmltree::Document::parse(include_str!($address)).unwrap().root_element(), id);
            e::Svg(hobo::create::svg_element(&element))
        }
    )*};
}

svg![
    logo => r"../../public/img/icons/etc/logo.svg",
    discord => r"../../public/img/icons/shapes/discord.svg",
];
```

## Constructing inline SVGs

Of course, if you need to algorithmically construct an svg, such as if you're making a chart, you can do that too:

```rust
let svg = e::svg()
    .attr(web_str::viewBox(), "-1 -1 2 2")
    .child(e::circle()
        .attr(web_str::cx(), "0")
        .attr(web_str::cy(), "0")
        .attr(web_str::r(), "1")
        .class((
            css::fill!(colors::gray6),
        ))
    );
```
