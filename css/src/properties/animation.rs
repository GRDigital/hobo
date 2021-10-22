crate::macros::easy_enum! {animation-direction normal reverse alternate alternate-reverse}
crate::macros::easy_enum! {animation-fill-mode none forwards backwards both}
crate::macros::easy_enum! {animation-iteration-count infinite [number]}
crate::macros::easy_enum! {animation-name none [string]}
crate::macros::easy_enum! {animation-play-state paused running}
crate::macros::easy_enum! {animation-timing-function linear ease ease-in ease-out ease-in-out step-start step-end [raw]}
crate::macros::easy_enum! {animation-duration [unit]}
crate::macros::easy_enum! {animation-delay [unit]}

crate::macros::easy_enum! {transition-property none all [raw]}
crate::macros::easy_enum! {transition-duration [unit]}
crate::macros::easy_enum! {transition-timing-function linear ease ease-in ease-out ease-in-out step-start step-end [raw]}
crate::macros::easy_enum! {transition-delay [unit]}
