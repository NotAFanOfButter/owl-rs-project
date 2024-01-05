use proc_macro::TokenStream;
use proc_macro2;
use syn;
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
        let to_byte_stream = parse_fields_to_bytes(data_struct);
        quote!{
            impl #generics ToByteVec for #ident #generics #where_clause {
                fn to_byte_vec(self) -> Vec<u8> {
                    #to_byte_stream
                }
            }
        }.into()
    } else {
        quote!().into()
    }
}

fn parse_fields_to_bytes(data: syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = match data.fields {
        syn::Fields::Named(named) => named.named,
        syn::Fields::Unnamed(unnamed) => unnamed.unnamed,
        syn::Fields::Unit => syn::punctuated::Punctuated::new()
    };
    if let syn::Fields::Named(named) = data.fields {
        let named_fields = named.named;
        let built = quote!(let mut bytes = Vec:new(););
        for field in fields {
            let ident = field.ident;
            built.extend(quote!{
                bytes.extend(self.#ident.to_byte_vec());
            })
        }
        built.extend(quote!(bytes));
        built
    } else if let syn::Fields::Unnamed(unnamed) = data.fields {
        for (i,_) in unnamed.unnamed.iter().enumerate() {
            
        }
    }
}
