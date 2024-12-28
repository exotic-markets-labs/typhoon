use {
    codama_korok_visitors::KorokVisitor,
    codama_koroks::StructKorok,
    codama_syn_helpers::extensions::{PathExtension, TypeExtension},
    std::{cell::RefCell, collections::HashSet, rc::Rc},
    syn::Item,
};

pub struct FilterByImplsVisitor<'a> {
    traits: &'a [&'static str],
    cache: Rc<RefCell<HashSet<String>>>,
    visitor: Box<dyn KorokVisitor + 'a>,
}

impl<'a> FilterByImplsVisitor<'a> {
    pub fn new<T: KorokVisitor + 'a>(
        traits: &'a [&'static str],
        cache: Rc<RefCell<HashSet<String>>>,
        visitor: T,
    ) -> Self {
        FilterByImplsVisitor {
            traits,
            visitor: Box::new(visitor),
            cache,
        }
    }
}

impl KorokVisitor for FilterByImplsVisitor<'_> {
    fn visit_unsupported_item(&mut self, korok: &mut codama_koroks::UnsupportedItemKorok) {
        self.visit_children(korok);

        let Item::Impl(impl_item) = korok.ast else {
            return;
        };

        let Some((_, trait_path, _)) = &impl_item.trait_ else {
            return;
        };

        if !self
            .traits
            .iter()
            .any(|trait_name| trait_path.last().ident == trait_name)
        {
            return;
        }

        if let Ok(impl_path) = impl_item.self_ty.as_path() {
            self.cache.borrow_mut().insert(impl_path.last_str());
        }
    }

    fn visit_struct(&mut self, korok: &mut StructKorok) {
        if korok.node.is_some() {
            return;
        }

        if self.cache.borrow().contains(&korok.ast.ident.to_string()) {
            self.visitor.visit_struct(korok);
        } else {
            self.visit_children(korok);
        }
    }
}
