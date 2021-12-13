//! everything that has to do with HTML event handling

use crate::{prelude::*, AsEntity, Element};

/// An enum for different event types
pub enum EventHandlerCallback {
	MouseEvent(Closure<dyn FnMut(web_sys::MouseEvent) + 'static>),
	KeyboardEvent(Closure<dyn FnMut(web_sys::KeyboardEvent) + 'static>),
	Event(Closure<dyn FnMut(web_sys::Event) + 'static>),
	FocusEvent(Closure<dyn FnMut(web_sys::FocusEvent) + 'static>),
	TouchEvent(Closure<dyn FnMut(web_sys::TouchEvent) + 'static>),
	WheelEvent(Closure<dyn FnMut(web_sys::WheelEvent) + 'static>),
	UiEvent(Closure<dyn FnMut(web_sys::UiEvent) + 'static>),

	// AnimationEvent
	// AnimationPlaybackEvent
	// DeviceMotionEvent
	// DeviceOrientationEvent
	// DeviceProximityEvent
	// DragEvent
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
	// HashChangeEvent
	// IdbVersionChangeEvent
	// ImageCaptureErrorEvent
	// InputEvent
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
	// MutationEvent
	// NotificationEvent
	// OfflineAudioCompletionEvent
	// PageTransitionEvent
	// PaymentMethodChangeEvent
	// PaymentRequestUpdateEvent
	// PointerEvent
	// PopStateEvent
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
	// XrInputSourceEvent
	// XrInputSourcesChangeEvent
	// CustomEvent
	// XrReferenceSpaceEvent
	// XrSessionEvent
	// DeviceLightEvent
}

/// A dom event subscription, that will unsubscribe from dom when dropped
pub struct EventHandler {
	target: web_sys::EventTarget,
	name: &'static str,
	cb: EventHandlerCallback,
}

impl Drop for EventHandler {
	fn drop(&mut self) {
		self.target.remove_event_listener_with_callback(self.name, match &self.cb {
			EventHandlerCallback::MouseEvent(cb) => cb.as_ref().unchecked_ref(),
			EventHandlerCallback::KeyboardEvent(cb) => cb.as_ref().unchecked_ref(),
			EventHandlerCallback::Event(cb) => cb.as_ref().unchecked_ref(),
			EventHandlerCallback::FocusEvent(cb) => cb.as_ref().unchecked_ref(),
			EventHandlerCallback::TouchEvent(cb) => cb.as_ref().unchecked_ref(),
			EventHandlerCallback::WheelEvent(cb) => cb.as_ref().unchecked_ref(),
			EventHandlerCallback::UiEvent(cb) => cb.as_ref().unchecked_ref(),
		}).unwrap();
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
						cb: EventHandlerCallback::$event_kind(handler),
					}
				}
			}

			pub trait [<$name:camel>]: Element {
				fn [<add_ $f>](&self, f: impl FnMut(web_sys::$event_kind) + 'static) {
					let entity = self.as_entity();
					if entity.is_dead() { log::warn!("callback handler entity dead {:?}", entity); return; }
					let target = entity.get_cmp::<web_sys::EventTarget>();
					entity.get_cmp_mut_or_default::<Vec<EventHandler>>().push(target.$f(f));
				}

				fn $f(self, f: impl FnMut(web_sys::$event_kind) + 'static) -> Self where Self: Sized { self.[<add_ $f>](f); self }
			}

			impl<T: Element> [<$name:camel>] for T {}
		)+

		pub mod impls {$(
			pub use super::[<$name:camel>];
			pub use super::[<Raw $name:camel>];
		)+}
	}};
}

generate_events! {
	MouseEvent,    click,       on_click;
	MouseEvent,    contextmenu, on_context_menu;
	MouseEvent,    dblclick,    on_dbl_click;
	MouseEvent,    mousedown,   on_mouse_down;
	MouseEvent,    mouseenter,  on_mouse_enter;
	MouseEvent,    mouseleave,  on_mouse_leave;
	MouseEvent,    mousemove,   on_mouse_move;
	MouseEvent,    mouseover,   on_mouse_over;
	MouseEvent,    mouseout,    on_mouse_out;
	MouseEvent,    mouseup,     on_mouse_up;
	KeyboardEvent, keydown,     on_key_down;
	KeyboardEvent, keyup,       on_key_up;
	Event,         change,      on_change;
	UiEvent,       scroll,      on_scroll;
	FocusEvent,    blur,        on_blur;
	FocusEvent,    focus,       on_focus;
	TouchEvent,    touchstart,  on_touch_start;
	TouchEvent,    touchend,    on_touch_end;
	TouchEvent,    touchmove,   on_touch_move;
	TouchEvent,    touchcancel, on_touch_cancel;
	WheelEvent,    wheel,       on_wheel;
	Event,         load,        on_load;
	Event,         canplay,     on_can_play;
}
