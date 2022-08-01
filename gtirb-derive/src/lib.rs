use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::*;
use syn::*;

fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => {
            Some(&typepath.path)
        }
        _ => None,
    }
}

fn extract_wnodebox_segment(path: &Path) -> Option<&PathSegment> {
    let idents_of_path =
        path.segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
    vec!["WNodeBox|"]
        .into_iter()
        .find(|s| &idents_of_path == *s)
        .and_then(|_| path.segments.last())
}

fn get_parent(ast: &syn::DeriveInput) -> Option<&Type> {
    let maybe_parent_field = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                fields.named.iter().find(|field| match &field.ident {
                    Some(id) => id.to_string() == "parent",
                    _ => false,
                })
            }
            _ => None,
        },
        _ => None,
    };

    maybe_parent_field
        .map(|field| {
            extract_type_path(&field.ty)
                .and_then(|path| extract_wnodebox_segment(path))
                .and_then(|path_seg| {
                    let type_params = &path_seg.arguments;

                    match *type_params {
                        PathArguments::AngleBracketed(ref params) => {
                            params.args.first()
                        }
                        _ => None,
                    }
                })
                .and_then(|generic_arg| match *generic_arg {
                    GenericArgument::Type(ref ty) => Some(ty),
                    _ => None,
                })
        })
        .flatten()
}

fn impl_node_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    let name_snake = name_str.to_case(Case::Snake);
    let ref_name = format_ident!("{}Ref", name);
    let context_remove_name = format_ident!("remove_{}", name_snake);

    let base = quote! {
        #[derive(Debug)]
        pub struct #ref_name {
            inner: NodeBox<#name>,
            context: Context,
        }

        impl #ref_name {
            pub(crate) fn new(context: &Context, ptr: NodeBox<#name>) -> Self {
                Self {
                    inner: NodeBox::clone(&ptr),
                    context: context.clone(),
                }
            }
        }

        impl PartialEq for #ref_name {
            // Pointer equality
            fn eq(&self, other: &Self) -> bool {
                Rc::ptr_eq(&self.inner, &other.inner)
            }
        }

        impl Node<#name> for #ref_name {
            fn get_inner(&self) -> &NodeBox<#name> { &self.inner }
            fn get_context(&self) -> &Context { &self.context }
        }

        impl Drop for #ref_name {
            fn drop(&mut self) {
                // Remove from the context when we're dropping the
                // only extant reference to the Node.
                // Note: This will need to be handled differently
                // if we ever support Arc's.
                if Rc::strong_count(&self.inner) == 1 {
                    self.context.#context_remove_name(&self.uuid());
                }
            }
        }

        impl IsRefFor<#name> for #ref_name {
            fn new(context: &Context, ptr: NodeBox<#name>) -> Self {
                #ref_name::new(context, ptr)
            }
        }

        impl Unique for #name {
            fn uuid(&self) -> Uuid {
                self.uuid
            }
        }
    };

    let set_parent = if let Some(parent) = get_parent(ast) {
        quote! {
            impl #name {
                pub(crate) fn set_parent(&mut self, parent: Option<&NodeBox<#parent>>) {
                    self.parent = match parent {
                        Some(ptr) => Rc::downgrade(ptr),
                        None => WNodeBox::new(),
                    }
                }

                fn get_parent(&self) -> Option<NodeBox<#parent>> {
                    self.parent.upgrade()
                }
            }
        }
    } else {
        quote! {}
    };

    let gen = quote! {
        #base
        #set_parent
    };

    gen.into()
}

#[proc_macro_derive(Node)]
pub fn node_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_node_derive(&ast)
}
