//! everything that has to do with HTML event handling

use crate::{prelude::*, AsEntity, AsElement};

/// A dom event subscription, that will unsubscribe from dom when dropped
pub struct EventHandler {
	target: web_sys::EventTarget,
	name: &'static str,
	cb: Box<dyn std::convert::AsRef<JsValue>>,
}

impl Drop for EventHandler {
	fn drop(&mut self) {
		let res = self.target.remove_event_listener_with_callback(self.name, (*self.cb).as_ref().unchecked_ref());
		if let Err(_e) = res {
			// TODO?
			// log::warn!("remove_event_listener_with_callback failed with error: {}", serde_json::to_string_pretty(&e.into_serde::<serde_json::Value>().unwrap()).unwrap());
		}
	}
}

macro_rules! generate_events {
	($($event_kind:ident, $name:ident, $f:ident);+$(;)*) => {paste::item!{
		$(
			pub trait [<Raw $name:camel>] {
				fn $f(&self, f: impl FnMut(web_sys::$event_kind) + 'static) -> EventHandler;
			}

			impl [<Raw $name:camel>] for web_sys::EventTarget {
				fn $f(&self, mut f: impl FnMut(web_sys::$event_kind) + 'static) -> EventHandler {
					let handler = Closure::wrap(Box::new(move |e| f(e)) as Box<dyn FnMut(web_sys::$event_kind) + 'static>);
					self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).expect("can't add event listener");
					EventHandler {
						target: self.clone(),
						name: web_str::$name(),
						cb: Box::new(handler),
					}
				}
			}

			pub trait [<$name:camel>]: AsElement {
				fn [<add_ $f>](&self, f: impl FnMut(web_sys::$event_kind) + 'static) {
					let entity = self.as_entity();
					if entity.is_dead() { log::warn!("callback handler entity dead {:?}", entity); return; }
					let target = entity.get_cmp::<web_sys::EventTarget>();
					entity.get_cmp_mut_or_default::<Vec<EventHandler>>().push(target.$f(f));
				}

				fn $f(self, f: impl FnMut(web_sys::$event_kind) + 'static) -> Self where Self: Sized { self.[<add_ $f>](f); self }
				fn [<with_ $f>](self, mut f: impl FnMut(&Self, web_sys::$event_kind) + 'static) -> Self where Self: Sized + Clone + 'static {
					let self_clone = self.clone();
					self.[<add_ $f>](move |event| f(&self_clone, event));
					self
				}
			}

			impl<T: AsElement> [<$name:camel>] for T {}
		)+

		pub mod impls {$(
			pub use super::[<$name:camel>] as _;
			pub use super::[<Raw $name:camel>] as _;
		)+}
	}};
}

#[cfg(not(web_sys_unstable_apis))]
generate_events! {
	MouseEvent,      click,              on_click;
	MouseEvent,      contextmenu,        on_context_menu;
	MouseEvent,      dblclick,           on_dbl_click;
	MouseEvent,      mousedown,          on_mouse_down;
	MouseEvent,      mouseenter,         on_mouse_enter;
	MouseEvent,      mouseleave,         on_mouse_leave;
	MouseEvent,      mousemove,          on_mouse_move;
	MouseEvent,      mouseover,          on_mouse_over;
	MouseEvent,      mouseout,           on_mouse_out;
	MouseEvent,      mouseup,            on_mouse_up;
	KeyboardEvent,   keydown,            on_key_down;
	KeyboardEvent,   keyup,              on_key_up;
	Event,           change,             on_change;
	Event,           scroll,             on_scroll;
	UiEvent,         resize,             on_resize;
	FocusEvent,      blur,               on_blur;
	FocusEvent,      focus,              on_focus;
	TouchEvent,      touchstart,         on_touch_start;
	TouchEvent,      touchend,           on_touch_end;
	TouchEvent,      touchmove,          on_touch_move;
	TouchEvent,      touchcancel,        on_touch_cancel;
	WheelEvent,      wheel,              on_wheel;
	Event,           load,               on_load;
	Event,           canplay,            on_can_play;
	DragEvent,       drag,               on_drag;
	SubmitEvent,     submit,             on_submit;
	InputEvent,      input,              on_input;
	AnimationEvent,  animationcancel,    on_animation_cancel;
	AnimationEvent,  animationend,       on_animation_end;
	AnimationEvent,  animationiteration, on_animation_iteration;
	AnimationEvent,  animationstart,     on_animation_start;
	PopStateEvent,   popstate,           on_pop_state;
	HashChangeEvent, hashchange,         on_hash_change;
}

//TODO: Temp Hack!!
#[cfg(web_sys_unstable_apis)]
generate_events! {
	MouseEvent,      click,              on_click;
	MouseEvent,      contextmenu,        on_context_menu;
	MouseEvent,      dblclick,           on_dbl_click;
	MouseEvent,      mousedown,          on_mouse_down;
	MouseEvent,      mouseenter,         on_mouse_enter;
	MouseEvent,      mouseleave,         on_mouse_leave;
	MouseEvent,      mousemove,          on_mouse_move;
	MouseEvent,      mouseover,          on_mouse_over;
	MouseEvent,      mouseout,           on_mouse_out;
	MouseEvent,      mouseup,            on_mouse_up;
	KeyboardEvent,   keydown,            on_key_down;
	KeyboardEvent,   keyup,              on_key_up;
	Event,           change,             on_change;
	Event,           scroll,             on_scroll;
	UiEvent,         resize,             on_resize;
	FocusEvent,      blur,               on_blur;
	FocusEvent,      focus,              on_focus;
	TouchEvent,      touchstart,         on_touch_start;
	TouchEvent,      touchend,           on_touch_end;
	TouchEvent,      touchmove,          on_touch_move;
	TouchEvent,      touchcancel,        on_touch_cancel;
	WheelEvent,      wheel,              on_wheel;
	Event,           load,               on_load;
	Event,           canplay,            on_can_play;
	DragEvent,       drag,               on_drag;
	SubmitEvent,     submit,             on_submit;
	InputEvent,      input,              on_input;
	AnimationEvent,  animationcancel,    on_animation_cancel;
	AnimationEvent,  animationend,       on_animation_end;
	AnimationEvent,  animationiteration, on_animation_iteration;
	AnimationEvent,  animationstart,     on_animation_start;
	PopStateEvent,   popstate,           on_pop_state;
	HashChangeEvent, hashchange,         on_hash_change;

	ClipboardEvent, paste, on_paste;
}
// DeviceMotionEvent
// DeviceOrientationEvent
// DeviceProximityEvent
// ErrorEvent
// ExtendableEvent
// ExtendableMessageEvent
// FetchEvent
// AudioProcessingEvent
// FontFaceSetLoadEvent
// GamepadAxisMoveEvent
// GamepadButtonEvent
// GamepadEvent
// GpuUncapturedErrorEvent
// IdbVersionChangeEvent
// ImageCaptureErrorEvent
// BeforeUnloadEvent
// MediaEncryptedEvent
// MediaKeyError
// MediaKeyMessageEvent
// MediaQueryListEvent
// MediaRecorderErrorEvent
// MediaStreamEvent
// MediaStreamTrackEvent
// MessageEvent
// MidiConnectionEvent
// MidiMessageEvent
// BlobEvent
// NotificationEvent
// OfflineAudioCompletionEvent
// PageTransitionEvent
// PaymentMethodChangeEvent
// PaymentRequestUpdateEvent
// PointerEvent
// ClipboardEvent
// PopupBlockedEvent
// PresentationConnectionAvailableEvent
// PresentationConnectionCloseEvent
// ProgressEvent
// PromiseRejectionEvent
// PushEvent
// RtcDataChannelEvent
// RtcPeerConnectionIceEvent
// RtcTrackEvent
// RtcdtmfToneChangeEvent
// CloseEvent
// ScrollAreaEvent
// SecurityPolicyViolationEvent
// SpeechRecognitionError
// SpeechRecognitionEvent
// SpeechSynthesisErrorEvent
// SpeechSynthesisEvent
// StorageEvent
// TcpServerSocketEvent
// TcpSocketErrorEvent
// TcpSocketEvent
// CompositionEvent
// TimeEvent
// TrackEvent
// TransitionEvent
// UserProximityEvent
// WebGlContextEvent
// DeviceLightEvent
