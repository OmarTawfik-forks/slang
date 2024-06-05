pub mod cst;
pub mod cursor;
pub mod diagnostic;
pub mod kinds;
pub mod language;
pub mod query;
pub mod text_index;

#[path = "generated/slang.rs"]
pub mod slang;

// #[path = "generated/ast_selectors.rs"]
// pub mod ast_selectors;

pub mod ffi {
    pub use crate::wit::slang::exports::nomic::slang::parser::{
        Cursor, CursorBorrow, EdgeLabel, Guest, GuestCursor, GuestLanguage, GuestNonterminalNode,
        GuestParseError, GuestParseOutput, GuestQuery, GuestQueryMatchIterator, GuestTerminalNode,
        Language, LanguageBorrow, Node, NonterminalKind, NonterminalNode, NonterminalNodeBorrow,
        ParseError, ParseErrorBorrow, ParseOutput, ParseOutputBorrow, Query, QueryBorrow,
        QueryError, QueryMatch, QueryMatchIterator, QueryMatchIteratorBorrow, Severity,
        TerminalKind, TerminalNode, TerminalNodeBorrow, TextIndex, TextRange,
    };
}

pub mod rust {
    pub use crate::{
        cst::{Edge, Node, NonterminalNode, TerminalNode},
        cursor::Cursor,
        diagnostic::{Diagnostic, Severity},
        kinds::{EdgeLabel, NonterminalKind, TerminalKind},
        language::Language,
        parse_error::ParseError,
        parse_output::ParseOutput,
        query::{Query, QueryError, QueryMatch, QueryMatchIterator},
        text_index::{TextIndex, TextRange},
    };
}

pub trait IntoFFI<F> {
    fn _into_ffi(self) -> F;
}

pub trait FromFFI<R> {
    fn _from_ffi(self) -> R;
}

macro_rules! define_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (rust::$name);

            impl $crate::wit::IntoFFI<ffi::$name> for rust::$name {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](self))
                }
            }

            impl $crate::wit::FromFFI<rust::$name> for ffi::$name {
                #[inline]
                fn _from_ffi(self) -> rust::$name {
                    self.into_inner::<[<$name Wrapper>]>().0
                }
            }

            // As owned argument
            impl ffi:: $name {
                #[inline]
                pub fn _borrow_ffi(&self) -> &rust::$name {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As borrowed argument
            impl<'a> ffi:: [<$name Borrow>] <'a> {
                #[inline]
                pub fn _borrow_ffi(&'a self) -> &'a rust::$name {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As self
            impl [<$name Wrapper>] {
                #[inline]
                pub fn _borrow_ffi(&self) -> &rust::$name {
                    &self.0
                }
            }

            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_rc_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (std::rc::Rc<rust::$name>);

            impl $crate::wit::IntoFFI<ffi::$name> for std::rc::Rc<rust::$name> {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](self))
                }
            }

            impl $crate::wit::FromFFI<std::rc::Rc<rust::$name>> for ffi::$name {
                #[inline]
                fn _from_ffi(self) -> std::rc::Rc<rust::$name> {
                    self.into_inner::<[<$name Wrapper>]>().0
                }
            }

            // As owned argument
            impl ffi:: $name {
                #[inline]
                pub fn _borrow_ffi(&self) -> &std::rc::Rc<rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As borrowed argument
            impl<'a> ffi:: [<$name Borrow>] <'a> {
                #[inline]
                pub fn _borrow_ffi(&self) -> &std::rc::Rc<rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As self
            impl [<$name Wrapper>] {
                #[inline]
                pub fn _borrow_ffi(&self) -> &std::rc::Rc<rust::$name> {
                    &self.0
                }
            }

            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_refcell_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (std::cell::RefCell<rust::$name>);

            impl $crate::wit::IntoFFI<ffi::$name> for rust::$name {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](std::cell::RefCell::new(self)))
                }
            }

            impl $crate::wit::FromFFI<rust::$name> for ffi::$name {
                #[inline]
                fn _from_ffi(self) -> rust::$name {
                    self.into_inner::<[<$name Wrapper>]>().0.into_inner()
                }
            }

            // As owned argument
            impl ffi:: $name {
                #[inline]
                pub fn _borrow_ffi(&self) -> std::cell::Ref<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
                #[inline]
                pub fn _borrow_mut_ffi(&self) -> std::cell::RefMut<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_mut_ffi()
                }
            }

            // As borrowed argument
            impl<'a> ffi:: [<$name Borrow>] <'a> {
                #[inline]
                pub fn _borrow_ffi(&self) -> std::cell::Ref<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
                #[inline]
                pub fn _borrow_mut_ffi(&self) -> std::cell::RefMut<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_mut_ffi()
                }
            }

            // As self
            impl [<$name Wrapper>] {
                #[inline]
                pub fn _borrow_ffi(&self) -> std::cell::Ref<'_, rust::$name> {
                    self.0.borrow()
                }
                #[inline]
                pub fn _borrow_mut_ffi(&self) -> std::cell::RefMut<'_, rust::$name> {
                    self.0.borrow_mut()
                }
            }

            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! enum_to_enum {
    ($name:ident) => {
        impl $crate::wit::IntoFFI<ffi::$name> for rust::$name {
            #[inline]
            fn _into_ffi(self) -> ffi::$name {
                unsafe { core::mem::transmute(self) }
            }
        }

        impl $crate::wit::FromFFI<rust::$name> for ffi::$name {
            #[inline]
            fn _from_ffi(self) -> rust::$name {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
}

// The trick: https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub(crate) use {define_rc_wrapper, define_refcell_wrapper, define_wrapper, enum_to_enum};

#[allow(clippy::upper_case_acronyms)]
pub struct API;

//================================================
//
// interface language
//
//================================================

impl ffi::Guest for API {
    type Language = language::LanguageWrapper;
    type ParseError = language::ParseErrorWrapper;
    type ParseOutput = language::ParseOutputWrapper;
    type NonterminalNode = cst::NonterminalNodeWrapper;
    type TerminalNode = cst::TerminalNodeWrapper;
    type Cursor = cursor::CursorWrapper;
    type Query = query::QueryWrapper;
    type QueryMatchIterator = query::QueryMatchIteratorWrapper;
}
