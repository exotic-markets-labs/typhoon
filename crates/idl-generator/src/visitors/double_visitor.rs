use {codama::KorokVisitor, codama_koroks::RootKorok};

pub struct DoubleVisitor<'a> {
    visitor: Box<dyn KorokVisitor + 'a>,
}

impl<'a> DoubleVisitor<'a> {
    pub fn new(visitor: impl KorokVisitor + 'a) -> Self {
        Self {
            visitor: Box::new(visitor),
        }
    }
}

impl KorokVisitor for DoubleVisitor<'_> {
    fn visit_root(&mut self, korok: &mut RootKorok) -> codama::CodamaResult<()> {
        self.visitor.visit_root(korok)?;
        self.visitor.visit_root(korok)?;
        Ok(())
    }
}
