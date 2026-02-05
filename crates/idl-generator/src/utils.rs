use codama::{get_type_node, DefinedTypeLinkNode, TypeNode};
use syn::{Error, Type};

pub fn extract_type(ty: &Type) -> Result<TypeNode, syn::Error> {
    if let Some(ty_node) = get_type_node(ty) {
        Ok(ty_node)
    } else {
        let Type::Path(ty_path) = ty else {
            return Err(Error::new_spanned(ty, "Invalid defined type."));
        };

        let seg = ty_path
            .path
            .segments
            .last()
            .ok_or(Error::new_spanned(ty, "Invalid defined path type."))?;

        Ok(TypeNode::Link(DefinedTypeLinkNode::new(
            seg.ident.to_string(),
        )))
    }
}
