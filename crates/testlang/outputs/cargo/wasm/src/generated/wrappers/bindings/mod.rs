// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::wasm_crate::utils::define_rc_wrapper;

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::bindings::{
        BindingGraph, BindingGraphBorrow, Guest, GuestBindingGraph,
    };
}

mod rust {
    pub use crate::rust_crate::bindings::BindingGraph;
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
