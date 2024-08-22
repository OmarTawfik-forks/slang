use std::collections::{HashMap, HashSet};
use std::fmt::{write, Write};

use anyhow::Result;
use rustdoc_types::{
    Crate, ExternalCrate, Id, Item, ItemEnum, ItemSummary, Module, Struct, Visibility,
};

pub struct Snapshot<'s> {
    index: &'s HashMap<Id, Item>,
    paths: &'s HashMap<Id, ItemSummary>,
    external_crates: &'s HashMap<u32, ExternalCrate>,

    w: String,
    indentation: usize,
    already_visited: HashSet<Id>,
}

impl<'s> Snapshot<'s> {
    pub fn create(_crate: &'s Crate) -> Result<String> {
        let Crate {
            root,
            crate_version: _,
            includes_private,
            index,
            paths,
            external_crates,
            format_version: _,
        } = _crate;

        assert!(!includes_private, "Unsupported: Private items.");

        let mut s = Self {
            index,
            paths,
            external_crates,

            w: String::new(),
            indentation: 0,
            already_visited: HashSet::new(),
        };

        index[root].serialize(&mut s)?;

        todo!("TLoop over all items in db, and panic if any item is not marked.");
    }

    fn add_padding(&mut self) {
        self.w.push_str(&" ".repeat(self.indentation * 4));
    }

    fn wrap_block(&mut self, callback: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        writeln!(self.w, "{{")?;

        self.indentation += 1;
        callback(self)?;
        self.indentation -= 1;

        self.add_padding();
        writeln!(self.w, "}}")?;

        Ok(())
    }
}

trait Serialize {
    fn serialize(&self, s: &mut Snapshot<'_>) -> Result<()>;
}

impl Serialize for Item {
    fn serialize(&self, s: &mut Snapshot<'_>) -> Result<()> {
        let Item {
            id,
            crate_id: _,
            name,
            span: _,
            visibility,
            docs,
            links: _,
            attrs,
            deprecation,
            inner,
        } = self;

        assert_eq!(
            visibility,
            &Visibility::Public,
            "Unsupported: private items: {id:?}",
        );

        assert!(deprecation.is_none(), "Unsupported: Deprecated items.");

        if let Some(docs) = docs {
            for line in docs.lines() {
                s.add_padding();
                writeln!(s.w, "/// {line}")?;
            }
        }

        for attr in attrs {
            s.add_padding();
            writeln!(s.w, "{attr}")?;
        }

        let name = name.clone().unwrap_or_default();

        match inner {
            ItemEnum::Module(Module {
                is_crate: _,
                items,
                is_stripped,
            }) => {
                assert!(!is_stripped, "Unsupported: Stripped modules.");

                s.add_padding();
                write!(s.w, "mod {name}")?;

                s.wrap_block(|s| {
                    for item_id in items {
                        s.index[item_id].serialize(s)?;
                    }

                    Ok(())
                })?;
            }
            ItemEnum::Struct(Struct {
                kind,
                generics,
                impls,
            }) => {

                s.add_padding();
                write!(s.w, "struct {name}")?;

                if !generics.is_empty() {
                    write!(s.w, "<")?;
                    for (i, generic) in generics.iter().enumerate() {
                        if i > 0 {
                            write!(s.w, ", ")?;
                        }
                        write!(s.w, "{generic}")?;
                    }
                    write!(s.w, ">")?;
                }

                if !impls.is_empty() {
                    write!(s.w, " where ")?;
                    for (i, impl_) in impls.iter().enumerate() {
                        if i > 0 {
                            write!(s.w, ", ")?;
                        }
                        write!(s.w, "{impl_}")?;
                    }
                }

                s.wrap_block(|s| {
                    match kind {
                        Some(kind) => {
                            s.add_padding();
                            writeln!(s.w, "kind: {kind},")?;
                        }
                        None => {}
                    }

                    Ok(())
                })?;
            }
            ItemEnum::StructField(_) => todo!(),
            ItemEnum::Enum(_) => todo!(),
            ItemEnum::Variant(_) => todo!(),
            ItemEnum::Function(_) => todo!(),
            ItemEnum::Trait(_) => todo!(),
            ItemEnum::TraitAlias(_) => todo!(),
            ItemEnum::Impl(_) => todo!(),
            ItemEnum::TypeAlias(_) => todo!(),
            ItemEnum::Constant { type_, const_ } => todo!(),
            ItemEnum::Static(_) => todo!(),
            ItemEnum::ForeignType => todo!(),
            ItemEnum::Macro(_) => todo!(),
            ItemEnum::ProcMacro(_) => todo!(),
            ItemEnum::Primitive(_) => todo!(),
            ItemEnum::AssocConst { type_, default } => todo!(),
            ItemEnum::AssocType {
                generics,
                bounds,
                default,
            } => todo!(),

            ItemEnum::ExternCrate { .. } | ItemEnum::Import(_) | ItemEnum::Union(_) => {
                panic!("Unsupported ItemEnum variant: {id:?}");
            }
        };

        Ok(())
    }
}
