use codama::KorokVisitor;
use codama_koroks::ItemKorok;

pub struct FilterItemsVisitor<'a, F>
where
    F: Fn(&ItemKorok) -> bool,
{
    pub filter: F,
    pub visitor: Box<dyn KorokVisitor + 'a>,
}

impl<'a, F> FilterItemsVisitor<'a, F>
where
    F: Fn(&ItemKorok) -> bool,
{
    pub fn new<T: KorokVisitor + 'a>(filter: F, visitor: T) -> Self {
        Self {
            filter,
            visitor: Box::new(visitor),
        }
    }
}

impl<F> KorokVisitor for FilterItemsVisitor<'_, F>
where
    F: Fn(&ItemKorok) -> bool,
{
    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) {
        if (self.filter)(korok) {
            self.visitor.visit_item(korok);
        } else {
            self.visit_children(korok);
        }
    }
}
