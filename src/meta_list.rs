use std::ops::Deref;

use darling::{ast::NestedMeta, FromMeta};
use syn::Meta;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MetaList(Vec<Meta>);

impl Deref for MetaList {
    type Target = Vec<Meta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Meta>> for MetaList {
    fn from(v: Vec<Meta>) -> Self {
        MetaList(v)
    }
}

impl FromMeta for MetaList {
    fn from_list(v: &[NestedMeta]) -> darling::Result<Self> {
        let mut metas = Vec::with_capacity(v.len());
        for nmi in v {
            if let NestedMeta::Meta(ref meta) = *nmi {
                metas.push(meta.clone());
            } else {
                return Err(darling::Error::unexpected_type("non-meta").with_span(nmi));
            }
        }

        Ok(MetaList(metas))
    }
}
