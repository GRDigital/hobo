use crate::{basic_element::BasicElement, prelude::*, Element};
use std::{cell::RefCell, rc::Rc};
use std::mem::MaybeUninit;

pub struct EventHandler(Box<dyn std::any::Any>);
pub type EventHandlers = RefCell<Vec<EventHandler>>;

macro_rules! generate_events {
	($($event_kind:path, $name:ident, $trait:ident, $f:ident);+$(;)*) => {paste::item!{
		pub trait EventTarget: Element {
			fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>>;
			$(
				fn $f(&self, f: impl FnMut($event_kind) + 'static) {
					use event_raw_exts::*;

					let handler = self.element().$f(f);
					self.event_handlers().push(handler);
				}

				fn [<$f _mut>]<T: 'static>(&self, this: &Rc<MaybeUninit<RefCell<T>>>, mut f: impl FnMut(&mut T, $event_kind) + 'static) {
					let weak = Rc::downgrade(this);
					self.$f(move |event| {
						let strong = if let Some(x) = weak.upgrade() { x } else { return; };
						let inited = unsafe { strong.assume_init() };
						f(&mut inited.borrow_mut(), event);
					})
				}

				fn [<with_ $f>](self, f: impl FnMut($event_kind) + 'static) -> Self where Self: Sized {
					self.$f(f);
					self
				}

				fn [<with_ $f _mut>]<T: 'static>(self, this: &Rc<MaybeUninit<RefCell<T>>>, f: impl FnMut(&mut T, $event_kind) + 'static) -> Self where Self: Sized {
					self.[<$f _mut>](this, f);
					self
				}
			)+
		}

		impl<T: AsRef<web_sys::Element> + 'static> BasicElement<T> {
			$(
				pub fn [<with_$f>](self, f: impl FnMut($event_kind) + 'static) -> Self {
					self.$f(f);
					self
				}
			)+
		}

		pub mod event_raw_exts {
			use super::*;

			$(
				#[extend::ext(pub, name = [<Raw $trait>])]
				impl web_sys::EventTarget {
					#[must_use]
					fn $f(&self, f: impl FnMut($event_kind) + 'static) -> EventHandler {
						let handler = Closure::<dyn FnMut($event_kind) + 'static>::new(f);
						self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
						EventHandler(Box::new(handler))
					}
				}
			)+
		}
	}};
}

generate_events! {
	web_sys::MouseEvent,    click,       OnClick,       on_click;
	web_sys::MouseEvent,    contextmenu, OnContextMenu, on_context_menu;
	web_sys::MouseEvent,    dblclick,    OnDblClick,    on_dbl_click;
	web_sys::MouseEvent,    mousedown,   OnMouseDown,   on_mouse_down;
	web_sys::MouseEvent,    mouseenter,  OnMouseEnter,  on_mouse_enter;
	web_sys::MouseEvent,    mouseleave,  OnMouseLeave,  on_mouse_leave;
	web_sys::MouseEvent,    mousemove,   OnMouseMove,   on_mouse_move;
	web_sys::MouseEvent,    mouseover,   OnMouseOver,   on_mouse_over;
	web_sys::MouseEvent,    mouseout,    OnMouseOut,    on_mouse_out;
	web_sys::MouseEvent,    mouseup,     OnMouseUp,     on_mouse_up;
	web_sys::KeyboardEvent, keydown,     OnKeyDown,     on_key_down;
	web_sys::KeyboardEvent, keyup,       OnKeyUp,       on_key_up;
	web_sys::Event,         change,      OnChange,      on_change;
	web_sys::Event,         scroll,      OnScroll,      on_scroll;
	web_sys::FocusEvent,    blur,        OnBlur,        on_blur;
	web_sys::FocusEvent,    focus,       OnFocus,       on_focus;
}

impl<T: EventTarget> EventTarget for Rc<RefCell<T>> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> {
		unsafe { self.try_borrow_unguarded() }.expect("rc is mutably borrowed").event_handlers()
	}
}
