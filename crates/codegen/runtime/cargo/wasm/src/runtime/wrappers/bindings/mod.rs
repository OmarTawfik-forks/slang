use std::rc::Rc;

use semver::Version;

use crate::wasm_crate::utils::{define_rc_wrapper, IntoFFI};

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::bindings::{
        BindingGraph, BindingGraphBorrow, Guest, GuestBindingGraph,
    };
}

mod rust {
    pub use crate::rust_crate::bindings::Bindings as BindingGraph;
}

impl ffi::Guest for crate::wasm_crate::World {
    type BindingGraph = BindingGraphWrapper;
}

//================================================
//
// resource binding-graph
//
//================================================

define_rc_wrapper! { BindingGraph {
} }
