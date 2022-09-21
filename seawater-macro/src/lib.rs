mod consts;

use iron_ingot::{snake_case_str::ToSnakeCase, CamelCaseStr};
use proc_macro::TokenStream;
use quote::{__private::TokenTree, format_ident, quote};
use std::cmp::Ordering;
use syn::{Expr, ExprField, ExprPath, Field, Fields, ItemStruct, Path, Visibility};

enum Var<'a> {
  Path(&'a ExprPath),
  Field(&'a ExprField),
}

#[proc_macro]
pub fn define_component(input: TokenStream) -> TokenStream {
  let ast: ItemStruct = syn::parse(input).expect("Argument of define_component must be a struct!");
  let module_name = format_ident!(
    "{}",
    CamelCaseStr::new(&ast.ident.to_string()).to_snake_case()
  );
  let impl_name = format_ident!("{}{}", ast.ident, consts::IMPL);
  let struct_impl = match &ast.fields {
    Fields::Named(fields) => {
      let fields = fields.named.iter().map(|field| Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ..field.clone()
      });
      quote! {
        {
          #( #fields, )*
        }
      }
    }
    Fields::Unnamed(fields) => {
      let fields = fields.unnamed.iter().map(|field| Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ..field.clone()
      });
      quote! {
        (#( #fields, )*);
      }
    }
    Fields::Unit => quote! { ; },
  };
  let default_values = match &ast.fields {
    Fields::Named(fields) => {
      let idents = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap());
      let values = fields.named.iter().map(|field| {
        if let Some(attr) = field.attrs.iter().find(|&attr| {
          attr
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == consts::DEFAULT)
        }) {
          if let TokenTree::Group(group) = attr.tokens.clone().into_iter().next().unwrap() {
            let value = group.stream();
            quote! {
              #value
            }
          } else {
            panic!("default attribute name must be followed by a value enclosed in parentheses!");
          }
        } else {
          quote! {
            Default::default()
          }
        }
      });
      quote! {
        {
          #( #idents: #values, )*
        }
      }
    }
    Fields::Unnamed(fields) => {
      let values = fields.unnamed.iter().map(|field| {
        if let Some(attr) = field.attrs.iter().find(|&attr| {
          attr
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == consts::DEFAULT)
        }) {
          if let TokenTree::Group(group) = attr.tokens.clone().into_iter().next().unwrap() {
            let value = group.stream();
            quote! {
              #value
            }
          } else {
            panic!("default attribute name must be followed by a value enclosed in parentheses!");
          }
        } else {
          quote! {
            Default::default()
          }
        }
      });
      quote! {
        (#( #values, )*)
      }
    }
    Fields::Unit => quote! {},
  };
  let vis = ast.vis;
  let name = ast.ident;
  let accessors = match ast.fields {
    Fields::Named(fields) => {
      let names = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap());
      let types = fields.named.iter().map(|field| &field.ty);
      quote! {
        #(
          #[allow(clippy::mut_from_ref)]
          pub fn #names(&self) -> &mut #types {
            &mut unsafe { &mut *self.0.get() }.#names
          }

        )*
      }
    }
    Fields::Unnamed(fields) => {
      let names = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(name, _)| format_ident!("{}", name));
      let types = fields.unnamed.iter().map(|field| &field.ty);
      quote! {
        #(
          #[allow(clippy::mut_from_ref)]
          pub fn #names(&self) -> &mut #types {
            &mut unsafe { &mut *self.0.get() }.#names
          }

        )*
      }
    }
    Fields::Unit => quote! {},
  };
  quote! {
    #vis mod #module_name {
      use super::*;
      use std::cell::UnsafeCell;
      use seawater::prelude::*;

      #[derive(Clone, Debug, PartialEq)]
      struct #impl_name #struct_impl

      impl Default for #impl_name {
        fn default() -> Self {
          Self #default_values
        }
      }

      #[derive(Debug)]
      pub struct #name(UnsafeCell<#impl_name>);

      unsafe impl Sync for #name {}

      impl Clone for #name {
        fn clone(&self) -> Self {
          Self(UnsafeCell::new(unsafe { &*self.0.get() }.clone()))
        }
      }

      impl Component for #name {}

      impl PartialEq for #name {
        fn eq(&self, other: &Self) -> bool {
          (unsafe { &*self.0.get() }) == (unsafe { &*other.0.get() })
        }
      }

      impl #name {
        pub fn new() -> Self {
          Self(UnsafeCell::new(#impl_name::default()))
        }

        #accessors
      }
    }

    #vis use #module_name::#name;
  }
  .into()
}

#[proc_macro]
pub fn find_archetype(input: TokenStream) -> TokenStream {
  let ast: Expr = syn::parse(input)
    .expect("Arguments of find_archetype must be a world variable and some component types!");
  if let Expr::Tuple(tuple) = ast {
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
    match tuple
      .elems
      .first()
      .expect("The first argument of find_archetype must be a world variable!")
    {
      Expr::Path(world) => match ty.len().cmp(&1) {
        Ordering::Greater => find_archetype_with_multiple_types(Var::Path(world), ty),
        Ordering::Equal => find_archetype_with_single_type(Var::Path(world), ty),
        Ordering::Less => panic!("Component types for find_archetype are required!"),
      },
      Expr::Field(world) => match ty.len().cmp(&1) {
        Ordering::Greater => find_archetype_with_multiple_types(Var::Field(world), ty),
        Ordering::Equal => find_archetype_with_single_type(Var::Field(world), ty),
        Ordering::Less => panic!("Component types for find_archetype are required!"),
      },
      _ => panic!("The first argument of find_archetype must be a world variable!"),
    }
  } else {
    panic!("Arguments of find_archetype must be a world variable and some component types!");
  }
}

fn find_archetype_with_single_type(world: Var, ty: Vec<&Path>) -> TokenStream {
  let world = match world {
    Var::Path(path) => quote! { #path },
    Var::Field(field) => quote! { #field },
  };
  let ty = ty[0];
  quote! {
    {
      use rayon::prelude::*;

      #world.get_components::<#ty>().par_iter().collect::<Vec<_>>()
    }
  }
  .into()
}

fn find_archetype_with_multiple_types(world: Var, ty: Vec<&Path>) -> TokenStream {
  let world = match world {
    Var::Path(path) => quote! { #path },
    Var::Field(field) => quote! { #field },
  };
  let component = ty
    .iter()
    .map(|&ty| {
      format_ident!(
        "{}",
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
          .trim_end_matches('_')
      )
    })
    .collect::<Vec<_>>();
  let components = component
    .iter()
    .map(|component| format_ident!("{}", component.to_string() + "s"))
    .collect::<Vec<_>>();
  quote! {
    {
      use rayon::prelude::*;

      #( let #components = #world.get_components::<#ty>(); )*
      (0..0#(.max(#components.len()))*)
        .into_par_iter()
        .filter_map(|i| {
          #( let #component = #components.get(i).unwrap_or(&None); )*
          if #( #component.is_some() || )* false {
            Some((#( #component, )*))
          } else {
            None
          }
        })
        .collect::<Vec<_>>()
    }
  }
  .into()
}
