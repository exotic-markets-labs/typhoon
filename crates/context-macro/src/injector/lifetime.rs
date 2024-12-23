use syn::{parse_quote, visit_mut::VisitMut, PathArguments};

pub struct LifetimeInjector;

impl VisitMut for LifetimeInjector {
    fn visit_generics_mut(&mut self, i: &mut syn::Generics) {
        i.params.push(parse_quote!('info));
    }

    fn visit_type_path_mut(&mut self, i: &mut syn::TypePath) {
        if let Some(seg) = i.path.segments.last_mut() {
            if seg.ident == "Mut" || seg.ident == "Option" {
                self.visit_path_segment_mut(seg);

                return;
            }

            match seg.arguments {
                PathArguments::AngleBracketed(ref mut gen_args) => {
                    gen_args.args.insert(0, parse_quote!('info));
                }
                PathArguments::None => {
                    seg.arguments = PathArguments::AngleBracketed(parse_quote!(<'info>));
                }
                PathArguments::Parenthesized(_) => {}
            }
        }
    }
}
