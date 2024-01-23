use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ToByteVec)]
pub fn to_byte_vec_derive(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        generics,
        ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    if let syn::Data::Struct(data_struct) = data {
        let where_clause = &generics.where_clause;
        let to_byte_stream = parse_fields_to_bytes(&data_struct);
        let stride_stream = parse_fields_to_stride(&data_struct);
        let offset_stream = parse_fields_to_offsets(&data_struct);
        quote! {
            impl #generics ToByteVec for #ident #generics #where_clause {
                fn to_byte_vec(self) -> Vec<u8> {
                    #to_byte_stream
                }
                fn stride(&self) -> Bytes {
                    #stride_stream
                }
                fn field_offset(&self, field_index: usize) -> Option<Bytes> {
                    #offset_stream
                }
            }
        }
        .into()
    } else {
        quote!().into()
    }
}

fn parse_fields_to_offsets(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(named) => {
            let named_fields = &named.named;
            if named_fields.is_empty() {
                return quote! {
                    None
                }
            }
            let mut built = quote!();
            for chosen_field_index in 1..named_fields.len() {
                let field_idents_before= named_fields.iter().take(chosen_field_index).
                    map(|f| f.ident.clone());
                let first = named_fields.first().map(|f| f.ident.clone());
                let mut sum_expr = quote!{
                    self.#first.stride().0
                };
                for f in field_idents_before.skip(1) {
                    sum_expr = quote!{
                        #sum_expr + self.#f.stride().0
                    }
                }
                let chosen_field_index = proc_macro2::Literal::usize_unsuffixed(chosen_field_index);
                built.extend(quote! {
                    #chosen_field_index => Some(Bytes(#sum_expr)),
                })
            }
            quote! {
                match field_index {
                    0 => Some(Bytes(0)),
                    #built
                    _ => None,
                }
            }
        }
        syn::Fields::Unnamed(unnamed) => {
            let unnamed_fields = &unnamed.unnamed;
            let mut built = quote!();
            if unnamed_fields.is_empty() {
                return quote!{
                    None
                }
            }
            for chosen_field_index in 1..unnamed_fields.len() {
                let field_indices_before = (1..chosen_field_index).map(proc_macro2::Literal::usize_unsuffixed);
                let first = proc_macro2::Literal::usize_unsuffixed(0);
                let mut sum_expr = quote!{
                    self.#first.stride().0
                };
                for i in field_indices_before {
                    sum_expr = quote!{
                        #sum_expr + self.#i.stride().0
                    }
                }
                let chosen_field_index = proc_macro2::Literal::usize_unsuffixed(chosen_field_index);
                built.extend(quote! {
                    #chosen_field_index => Some(Bytes(#sum_expr)),
                })
            }
            quote! {
                match field_index {
                    0 => Some(Bytes(0)),
                    #built
                    _ => None,
                }
            }
        }
        syn::Fields::Unit => quote! {
            None
        },
    }
}

fn parse_fields_to_stride(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(named) => {
            let named_fields = &named.named;
            let mut built = quote!(let mut total = 0;);
            for field in named_fields {
                let ident = field.ident.clone();
                built.extend(quote! {
                    total += self.#ident.stride().0;
                })
            }
            built.extend(quote!(Bytes(total)));
            built
        }
        syn::Fields::Unnamed(unnamed) => {
            let mut built = quote!(let mut total = 0;);
            for (i, _) in unnamed.unnamed.iter().enumerate() {
                let index = syn::Index::from(i);
                built.extend(quote! {
                    total += self.#index.stride().0;
                })
            }
            built.extend(quote!(Bytes(total)));
            built
        }
        syn::Fields::Unit => quote!(Bytes(0)),
    }
}

fn parse_fields_to_bytes(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(named) => {
            let named_fields = &named.named;
            let mut built = quote!(let mut bytes = Vec::new(););
            for field in named_fields {
                let ident = field.ident.clone();
                built.extend(quote! {
                    bytes.extend(self.#ident.to_byte_vec());
                })
            }
            built.extend(quote!(bytes));
            built
        }
        syn::Fields::Unnamed(unnamed) => {
            let mut built = quote!(let mut bytes = Vec::new(););
            for (i, _) in unnamed.unnamed.iter().enumerate() {
                let index = syn::Index::from(i);
                built.extend(quote! {
                    bytes.extend(self.#index.to_byte_vec());
                })
            }
            built.extend(quote!(bytes));
            built
        }
        _ => quote!(Vec::new()),
    }
}
