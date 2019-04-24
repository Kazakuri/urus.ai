//! Code generation utilities

extern crate proc_macro;
extern crate proc_macro2;

use crate::proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{ Ident, Span };
use heck::SnakeCase;

#[proc_macro_derive(DbMessage)]
/// Implements Handler<T> for DbExecutor for the derived type.
///
/// The handle function will call the Repository function with the snake case equivalent name of the derived type.
pub fn db_message_derive(input: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(input).unwrap();

  let name = &ast.ident;
  let func = Ident::new(&name.to_string().to_snake_case(), Span::call_site());

  let gen = quote! {
    impl std::convert::From<#name> for actix_web::Binary {
      fn from(me: #name) -> Self {
        serde_urlencoded::to_string(&me).unwrap().into()
      }
    }

    impl Handler<#name> for DbExecutor {
      type Result = <#name as Message>::Result;

      fn handle(&mut self, msg: #name, _: &mut Self::Context) -> Self::Result {
        (self.0).#func(msg)
      }
    }
  };

  gen.into()
}
