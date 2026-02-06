use codama::{CodamaResult, CombineTypesVisitor, KorokVisitor};

pub struct SetDefinedTypesVisitor {
    combine_types: CombineTypesVisitor,
}

impl Default for SetDefinedTypesVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
        }
    }
}

impl SetDefinedTypesVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetDefinedTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        if korok.node.is_some() {
            return Ok(());
        }
        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        if korok.node.is_some() {
            return Ok(());
        }
        // Create a `DefinedTypeNode` from the enum, if it doesn't already exist.
        self.combine_types.visit_enum(korok)?;

        Ok(())
    }
}
