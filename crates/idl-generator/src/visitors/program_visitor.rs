use codama::KorokVisitor;
use hashbrown::HashMap;
use syn::Item;
use typhoon_syn::{Context, Instruction, InstructionsList};

use crate::helpers::AttributesHelper;

#[derive(Default)]
pub struct ProgramVisitor {
    pub instruction_list: InstructionsList,
    pub instructions: HashMap<String, Instruction>,
    pub contexts: HashMap<String, Context>,
}

impl ProgramVisitor {
    pub fn new() -> Self {
        ProgramVisitor::default()
    }
}

impl KorokVisitor for ProgramVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> codama::CodamaResult<()> {
        if korok.attributes.has_attribute("context") {
            self.contexts
                .insert(korok.ast.ident.to_string(), Context::try_from(korok.ast)?);
        }

        Ok(())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> codama::CodamaResult<()> {
        match korok.ast {
            Item::Macro(item_macro) => {
                if item_macro.mac.path.is_ident("handlers") {
                    self.instruction_list = InstructionsList::try_from(item_macro)?;
                };
            }
            Item::Fn(item_fn) => {
                if let Ok(ix) = Instruction::try_from(item_fn) {
                    self.instructions.insert(ix.name.to_string(), ix);
                }
            }
            _ => (),
        }

        Ok(())
    }
}
