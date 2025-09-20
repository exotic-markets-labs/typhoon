use {
    crate::{mod_path::ModContext, read_and_parse_file},
    std::path::Path,
    syn::{visit::Visit, Item, ItemMod, Visibility},
};

pub struct Resolver<'a> {
    path: &'a Path,
    root: bool,
    mod_context: ModContext,
    pub items: Vec<Item>,
}

impl<'a> Resolver<'a> {
    pub fn new(path: &'a Path, root: bool) -> Self {
        Self {
            path,
            root,
            mod_context: Default::default(),
            items: Vec::new(),
        }
    }
}

impl Visit<'_> for Resolver<'_> {
    fn visit_file(&mut self, i: &'_ syn::File) {
        for item in &i.items {
            let is_public = match &item {
                Item::Fn(f) => matches!(f.vis, Visibility::Public(_)),
                Item::Struct(s) => matches!(s.vis, Visibility::Public(_)),
                Item::Enum(e) => matches!(e.vis, Visibility::Public(_)),
                _ => false,
            };

            if is_public {
                self.items.push(item.clone());
            }
            self.visit_item(item);
        }
    }

    fn visit_item_mod(&mut self, i: &ItemMod) {
        self.mod_context.push(i.into());

        if let Some((_, items)) = &i.content {
            for item in items {
                self.visit_item(item);
            }
        } else {
            let candidates = self.mod_context.relative_to(self.path, self.root);
            let first_candidate = candidates.iter().find(|p| p.exists());

            if let Some(file) = first_candidate.and_then(|p| read_and_parse_file(p).ok()) {
                self.visit_file(&file);
            };
        }

        self.mod_context.pop();
    }
}
