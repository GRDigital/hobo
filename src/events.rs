use crate::{prelude::*, Element};
use std::{cell::RefCell, rc::Rc};
use std::mem::MaybeUninit;

pub enum EventHandler {
	MouseEvent(Closure<dyn FnMut(web_sys::MouseEvent) + 'static>),
	KeyboardEvent(Closure<dyn FnMut(web_sys::KeyboardEvent) + 'static>),
	Event(Closure<dyn FnMut(web_sys::Event) + 'static>),
	FocusEvent(Closure<dyn FnMut(web_sys::FocusEvent) + 'static>),

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
	// MouseScrollEvent
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
	// TouchEvent
	// TrackEvent
	// TransitionEvent
	// UiEvent
	// UserProximityEvent
	// WebGlContextEvent
	// WheelEvent
	// XrInputSourceEvent
	// XrInputSourcesChangeEvent
	// CustomEvent
	// XrReferenceSpaceEvent
	// XrSessionEvent
	// DeviceLightEvent
}

pub type EventHandlers = RefCell<Vec<EventHandler>>;

macro_rules! generate_events {
	($($event_kind:ident, $name:ident, $f:ident);+$(;)*) => {paste::item!{
		/// Trait for all hobo elements that can handle various browser events
		pub trait EventTarget: Element {
			fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>>;
			$(
				fn $f(&self, f: impl FnMut(web_sys::$event_kind) + 'static) {
					let handler = self.element().$f(f);
					self.event_handlers().push(handler);
				}

				fn [<$f _mut>]<T: 'static>(&self, this: &Rc<MaybeUninit<RefCell<T>>>, mut f: impl FnMut(&mut T, web_sys::$event_kind) + 'static) {
					let weak = Rc::downgrade(this);
					self.$f(move |event| {
						let strong = if let Some(x) = weak.upgrade() { x } else { return; };
						let inited = unsafe { strong.assume_init() };
						f(&mut inited.borrow_mut(), event);
					})
				}

				fn [<with_ $f>](self, f: impl FnMut(web_sys::$event_kind) + 'static) -> Self where Self: Sized {
					self.$f(f);
					self
				}

				fn [<with_ $f _mut>]<T: 'static>(self, this: &Rc<MaybeUninit<RefCell<T>>>, f: impl FnMut(&mut T, web_sys::$event_kind) + 'static) -> Self where Self: Sized {
					self.[<$f _mut>](this, f);
					self
				}
			)+
		}

		#[extend::ext(pub, name = [<RawEventTarget>])]
		impl web_sys::EventTarget {$(
			#[must_use]
			fn $f(&self, f: impl FnMut(web_sys::$event_kind) + 'static) -> EventHandler {
				let handler = Closure::<dyn FnMut(web_sys::$event_kind) + 'static>::new(f);
				self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
				EventHandler::$event_kind(handler)
			}
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
	Event,         scroll,      on_scroll;
	FocusEvent,    blur,        on_blur;
	FocusEvent,    focus,       on_focus;
}

impl<T: EventTarget> EventTarget for Rc<RefCell<T>> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> {
		unsafe { self.try_borrow_unguarded() }.expect("rc is mutably borrowed").event_handlers()
	}
}
