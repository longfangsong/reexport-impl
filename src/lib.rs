use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprPath, ImplItem, ItemImpl, Token, Type};

#[proc_macro_attribute]
pub fn reexport(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    let attr = parse_macro_input!(attr as syn::Expr);
    if let Expr::Path(ExprPath { path, .. }) = attr {
        let mut struct_path = path.clone();
        let field_name = struct_path.segments.pop().unwrap();
        struct_path.segments.pop_punct();
        let mut reexport_impl = item.clone();
        reexport_impl.self_ty = Box::new(Type::Path(syn::TypePath {
            qself: None,
            path: struct_path,
        }));
        reexport_impl.items.clear();
        for item in &item.items {
            if let ImplItem::Fn(f) = item {
                let f_name = &f.sig.ident;
                let inputs = &f.sig.inputs;
                let output = &f.sig.output;
                let receiver = inputs
                    .iter()
                    .filter_map(|i| match i {
                        syn::FnArg::Receiver(r) => Some(r.clone()),
                        syn::FnArg::Typed(_) => None,
                    })
                    .next()
                    .unwrap();
                let argument_names: Vec<_> = inputs
                    .iter()
                    .filter_map(|i| match i {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(pat) => Some(pat.pat.clone()),
                    })
                    .collect();
                let new_f = quote! {
                    pub fn #f_name(#inputs) #output {
                        self.#field_name.#f_name(#(#argument_names),*)
                    }
                };
                reexport_impl.items.push(ImplItem::Verbatim(new_f));
            }
        }
        quote! {
            #item
            #reexport_impl
        }
        .into()
    } else {
        todo!()
    }
}
