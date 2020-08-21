use super::*;

#[test]
fn state_update() {
	use crate::enclose as e;

	let state = State::new(5);
	let sub = state.subscribe(e!((%state state) move || {
		assert_eq!(*state.view(), 10);
	}));
	*state.update() = 10;
}

#[test]
fn nested_state_update() {
	use crate::enclose as e;

	let state = State::new(5);
	let sub = state.subscribe(e!((%state state) move || {
		if *state.view() == 10 {
			*state.update() = 15;
		} else {
			assert_eq!(*state.view(), 15);
		}
	}));
	*state.update() = 10;
}

#[test]
fn nested_unsubscribe() {
	use crate::enclose as e;

	let state = State::new(5);
	let sub_key = Rc::new(RefCell::new(None));
	*sub_key.borrow_mut() = Some(state.subscribe_key(e!((%state state, %sub_key) move || {
		state.unsubscribe(sub_key.borrow_mut().take().unwrap());
		assert_eq!(*state.view(), 10);
	})));
	*state.update() = 10;
	assert!(sub_key.borrow().is_none());
	*state.update() = 15;
}

#[test]
fn sub_drop() {
	use crate::enclose as e;

	let state = State::new(5);
	let sub = state.subscribe(move || panic!("sub ran after dropped"));
	drop(sub);
	*state.update() = 10;
	assert_eq!(*state.view(), 10);
}
