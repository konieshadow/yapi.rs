use proc_macro::TokenStream;

#[proc_macro_derive(AutoTimestampModel)]
pub fn auto_timestamp_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_auto_timestamp_macro(&ast)
}

fn impl_auto_timestamp_macro(_ast: &syn::DeriveInput) -> TokenStream {
    let gen = quote::quote! {
        impl AutoTimestamp for ActiveModel {
            fn default_add() -> Self {
                let timestamp = time::OffsetDateTime::now_utc().unix_timestamp() as u32;
                Self {
                    add_time: sea_orm::Set(timestamp),
                    up_time: sea_orm::Set(timestamp),
                    ..Default::default()
                }
            }
        
            fn default_up() -> Self {
                let timestamp = time::OffsetDateTime::now_utc().unix_timestamp() as u32;
                Self {
                    up_time: sea_orm::Set(timestamp),
                    ..Default::default()
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PaginatorQuery)]
pub fn paginator_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_paginator_macro(&ast)
}

fn impl_paginator_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote::quote! {
        impl Paginator for #name {
            fn page_size(&self) -> usize {
                self.limit.unwrap_or(20)
            }
        
            fn page(&self) -> usize {
                if let Some(page) = self.page {
                    if page == 0 {
                        0
                    } else {
                        page - 1
                    }
                } else {
                    0
                }
            }
        }
    };
    gen.into()
}