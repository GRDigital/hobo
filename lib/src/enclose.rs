#[doc(hidden)]
#[macro_export]
macro_rules! __make_rc_upgrade_stmt {
	(%slot $expr:expr => $ident:ident) => {
		let $ident = if let Some(x) = ::std::rc::Weak::upgrade(&$ident) { $crate::Slot(x) } else { return; };
	};
	(%slot $e:ident) => {
		let $e = if let Some(x) = ::std::rc::Weak::upgrade(&$e) { $crate::Slot(x) } else { return; };
	};
	(%state $expr:expr => $ident:ident) => {
		let $ident = if let Some(x) = ::std::rc::Weak::upgrade(&$ident) { $crate::state::State(x) } else { return; };
	};
	(%state $e:ident) => {
		let $e = if let Some(x) = ::std::rc::Weak::upgrade(&$e) { $crate::state::State(x) } else { return; };
	};
	(% $expr:expr => $ident:ident) => {
		let $ident = if let Some(x) = ::std::rc::Weak::upgrade(&$ident) { x } else { return; };
	};
	(% $e:ident) => {
		let $e = if let Some(x) = ::std::rc::Weak::upgrade(&$e) { x } else { return; };
	};
	($($input:tt)*) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_stmt {
	(%slot $expr:expr => $ident:ident) => {
		let $ident = ::std::rc::Rc::downgrade(&$expr.0);
	};
	(%slot $e:ident) => {
		let $e = ::std::rc::Rc::downgrade(&$e.0);
	};
	(%state $expr:expr => $ident:ident) => {
		let $ident = ::std::rc::Rc::downgrade(&$expr.0);
	};
	(%state $e:ident) => {
		let $e = ::std::rc::Rc::downgrade(&$e.0);
	};
	(% $expr:expr => $ident:ident) => {
		let $ident = ::std::rc::Rc::downgrade(&$expr);
	};
	(% $e:ident) => {
		let $e = ::std::rc::Rc::downgrade(&$e);
	};
	(* $e:ident) => {
		let $e = $e.raw_element().clone();
	};
	(* $expr:expr => $ident:ident) => {
		let $ident = $expr.raw_element().clone();
	};
	($expr:expr => mut $ident:ident) => {
		let mut $ident = $expr.clone();
	};
	($expr:expr => $ident:ident) => {
		let $ident = $expr.clone();
	};
	(mut $e:ident) => {
		let mut $e = $e.clone();
	};
	($e:ident) => {
		let $e = $e.clone();
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __e_inner {
	// all inputs and the closure were consumed - return result basically
	(
		input = (),
		current_input = (),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (),
		header = ($($header:tt)*),
		body = ($body:tt),
	) => {{
		$($stmt)*
		$($header)* {
			$($rc_upgrade_stmt)*
			$body
		}
	}};

	// this input has finished - make statements
	(
		input = (, $($rest:tt)*),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (),
		header = ($($header:tt)*),
		body = ($body:tt),
	) => {
		$crate::__e_inner!{
			input = ($($rest)*),
			current_input = (),
			rc_upgrade_stmts = ($crate::__make_rc_upgrade_stmt!{$($current_input)*} $($rc_upgrade_stmt)*),
			stmts = ($crate::__make_stmt!{$($current_input)*} $($stmt)*),
			closure_input = (),
			header = ($($header)*),
			body = ($body),
		}
	};

	// last input's last piece was consumed - make statements
	(
		input = (),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (),
		header = ($($header:tt)*),
		body = ($body:tt),
	) => {
		$crate::__e_inner!{
			input = (),
			current_input = (),
			rc_upgrade_stmts = ($crate::__make_rc_upgrade_stmt!{$($current_input)*} $($rc_upgrade_stmt)*),
			stmts = ($crate::__make_stmt!{$($current_input)*} $($stmt)*),
			closure_input = (),
			header = ($($header)*),
			body = ($body),
		}
	};

	// consume an input piece token
	(
		input = ($current:tt $($rest:tt)*),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (),
		header = ($($header:tt)*),
		body = ($body:tt),
	) => {
		$crate::__e_inner!{
			input = ($($rest)*),
			current_input = ($($current_input)* $current),
			rc_upgrade_stmts = ($($rc_upgrade_stmt)*),
			stmts = ($($stmt)*),
			closure_input = (),
			header = ($($header)*),
			body = ($body),
		}
	};

	// special case because || is treated as one token
	(
		input = ($($rest:tt)*),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (move || $($closure_input:tt)*),
		header = ($($header:tt)*),
		body = (),
	) => {
		$crate::__e_inner!{
			input = ($($rest)*),
			current_input = ($($current_input)*),
			rc_upgrade_stmts = ($($rc_upgrade_stmt)*),
			stmts = ($($stmt)*),
			closure_input = (),
			header = ($($header)* move ||),
			body = ({ $($closure_input)* }),
		}
	};

	// closure opens - start consuming header
	(
		input = ($($rest:tt)*),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (move | $($closure_input:tt)*),
		header = ($($header:tt)*),
		body = (),
	) => {
		$crate::__e_inner!{
			input = ($($rest)*),
			current_input = ($($current_input)*),
			rc_upgrade_stmts = ($($rc_upgrade_stmt)*),
			stmts = ($($stmt)*),
			closure_input = ($($closure_input)*),
			header = ($($header)* move |),
			body = (),
		}
	};

	// closure closes (otherwise top would've matched) - the rest is the body
	(
		input = ($($rest:tt)*),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = (| $($current:tt)*),
		header = ($($header:tt)*),
		body = (),
	) => {
		$crate::__e_inner!{
			input = ($($rest)*),
			current_input = ($($current_input)*),
			rc_upgrade_stmts = ($($rc_upgrade_stmt)*),
			stmts = ($($stmt)*),
			closure_input = (),
			header = ($($header)* |),
			body = ({ $($current)* }),
		}
	};

	// consume closure argument list
	(
		input = ($($rest:tt)*),
		current_input = ($($current_input:tt)*),
		rc_upgrade_stmts = ($($rc_upgrade_stmt:tt)*),
		stmts = ($($stmt:tt)*),
		closure_input = ($current:tt $($closure_input:tt)*),
		header = ($($header:tt)*),
		body = (),
	) => {
		$crate::__e_inner!{
			input = ($($rest)*),
			current_input = ($($current_input)*),
			rc_upgrade_stmts = ($($rc_upgrade_stmt)*),
			stmts = ($($stmt)*),
			closure_input = ($($closure_input)*),
			header = ($($header)* $current),
			body = (),
		}
	};
}

/// Essentially a "move-by-clone" closure creator with some special cases for common hobo idioms    
/// for more information, refer to the [book](https://grdigital.github.io/hobo/state.html)
#[macro_export]
macro_rules! enclose {
	(($($input:tt)*) $($closure_input:tt)+) => {
		$crate::__e_inner!{
			input = ($($input)*),
			current_input = (),
			rc_upgrade_stmts = (),
			stmts = (),
			closure_input = ($($closure_input)+),
			header = (),
			body = (),
		};
	};
}

// #[test]
// fn test_enclosure() {
//     use std::rc::Rc;

//     let some_rc = Rc::new(50);
//     let clonable = 50;
//     // let f = enclose!((clonable => mut x, %some_rc) move || {
//     // let f = enclose!((%some_rc) move || {
//     //     panic!("rc shoulda been dropped");
//     // });
//     let f = enclose!((%some_rc) move || *some_rc = 100);
//         // panic!("rc shoulda been dropped");
//     // });
//     drop(some_rc);
//     f();
// }
