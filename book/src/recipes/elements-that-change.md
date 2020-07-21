# Elements that change

Since there's no VDOM, rebuilding the DOM is done manually by literally rebuilding the altered parts. It is on the developer to minimize this to maintain element focus, scroll position, performance, etc. The same goes for styling - any complex modification is best expressed as recreating the whole style.

<!-- TODO: provide an example-->
