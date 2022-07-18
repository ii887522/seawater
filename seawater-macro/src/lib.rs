use iron_ingot::{snake_case_str::ToSnakeCase, CamelCaseStr};
use proc_macro::TokenStream;
use quote::{__private::Span, quote};
use std::cmp::Ordering::{Equal, Greater, Less};
use syn::{DeriveInput, Expr, Ident, Path};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let name = ast.ident;
  quote! {
    impl Component for #name {}
  }
  .into()
}

#[proc_macro]
pub fn find_archetype(input: TokenStream) -> TokenStream {
  let ast: Expr = syn::parse(input)
    .expect("Arguments of find_archetype must be a world variable and some component types!");
  if let Expr::Tuple(tuple) = ast {
    if let Expr::Path(world) = tuple.elems.first().expect("The first argument of find_archetype must be a world variable!") {
      let world = &world.path;
      let ty = tuple
        .elems
        .iter()
        .skip(1)
        .map(|ty| {
          if let Expr::Path(ty) = ty {
            &ty.path
          } else {
            panic!("All arguments of find_archetype except the first one must be component types!");
          }
        })
        .collect::<Vec<_>>();
      match ty.len().cmp(&1) {
        Greater => find_archetype_with_multiple_types(world, ty),
        Equal => find_archetype_with_single_type(world, ty),
        Less => panic!("Component types for find_archetype are required!"),
      }
    } else {
      panic!("The first argument of find_archetype must be a world variable!");
    }
  } else {
    panic!("Arguments of find_archetype must be a world variable and some component types!");
  }
}

fn find_archetype_with_single_type(world: &Path, ty: Vec<&Path>) -> TokenStream {
  let ty = ty[0];
  quote! {
    {
      use std::borrow::Cow;

      Cow::<[Option<#ty>]>::from(#world.get_components::<#ty>())
    }
  }
  .into()
}

fn find_archetype_with_multiple_types(world: &Path, ty: Vec<&Path>) -> TokenStream {
  let component = ty
    .iter()
    .map(|&ty| {
      Ident::new(
        ty.segments
          .iter()
          .map(|segment| {
            let ident = segment.ident.to_string();
            if ident.starts_with(char::is_uppercase) {
              CamelCaseStr::new(&ident).to_snake_case()
            } else {
              ident
            }
          })
          .fold("".to_owned(), |component, segment| {
            component + &segment + "_"
          })
          .trim_end_matches('_'),
        Span::mixed_site(),
      )
    })
    .collect::<Vec<_>>();
  let components = component
    .iter()
    .map(|component| Ident::new(&(component.to_string() + "s"), Span::mixed_site()))
    .collect::<Vec<_>>();
  quote! {
    {
      use std::borrow::Cow;
      use rayon::prelude::*;

      #( let #components = #world.get_components::<#ty>(); )*
      (0..0#(.max(#components.len()))*)
        .into_par_iter()
        .filter_map(|i| {
          #( let &#component = #components.get(i).unwrap_or(&None); )*
          if #( #component.is_some() || )* false {
            Some((#( #component, )*))
          } else {
            None
          }
        })
        .collect::<Cow::<[(#( Option<#ty>, )*)]>>()
    }
  }
  .into()
}
