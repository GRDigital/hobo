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

// animatable properties should be an enum and `transition-property` should use it

// list of reasonable animatable properties:
//
// all
// background-color
// background-position
// background-size
// border-<side>
// border-<side>-color
// border-<side>-width
// border-<corner>-radius
// top, left, right, bottom
// row/column-gap
// color
// flex-basis
// flex-grow
// flex-shrink
// accent-color
// box-shadow
// clip-path
// font-size
// font-weight
// letter-spacing
// word-spacing
// line-height
// width/height, min-, max-
// margin/padding
// opacity
// order
// outline-color
// outline-offset
// outline-width
// rotate
// transform
// translate
// z-index
// zoom
