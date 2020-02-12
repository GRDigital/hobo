use crate::prelude::*;
use std::cell::RefCell;

pub struct EventHandler(Box<dyn std::any::Any>);
pub type EventHandlers = RefCell<Vec<EventHandler>>;

macro_rules! generate_events {
	($($event_kind:path, $name:ident, $trait:ident, $f:ident);+$(;)*) => {paste::item!{
		pub trait EventTarget: crate::Element {
			fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>>;
			$(
				fn $f(&self, f: impl FnMut($event_kind) + 'static) where Self: Sized {
					use event_raw_exts::*;

					let handler = self.element().$f(f);
					self.event_handlers().push(handler);
				}

				#[allow(clippy::missing_safety_doc)]
				unsafe fn [<unsafe_ $f>]<'a>(&'a self, f: impl FnMut($event_kind) + 'a) where Self: Sized {
					use event_raw_exts::*;

					let handler = self.element().[<unsafe_ $f>](f);
					self.event_handlers().push(handler);
				}
			)+
		}

		pub mod event_raw_exts {
			use super::*;

			$(
				#[extend::ext(pub, name = [<Raw $trait>])]
				impl web_sys::EventTarget {
					#[must_use]
					fn $f(&self, f: impl FnMut($event_kind) + 'static) -> EventHandler where Self: Sized {
						let fbox: Box<dyn FnMut($event_kind) + 'static> = Box::new(f);
						let handler = Closure::wrap(fbox);
						self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
						EventHandler(Box::new(handler))
					}

					#[must_use]
					#[allow(clippy::missing_safety_doc)]
					unsafe fn [<unsafe_ $f>]<'a>(&'a self, f: impl FnMut($event_kind) + 'a) -> EventHandler where Self: Sized {
						let fbox: Box<dyn FnMut($event_kind) + 'a> = Box::new(f);
						let long_fbox: Box<dyn FnMut($event_kind) + 'static> = std::mem::transmute(fbox);
						let handler = Closure::wrap(long_fbox);
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
}
