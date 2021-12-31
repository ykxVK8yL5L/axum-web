#![allow(clippy::enum_glob_use, clippy::needless_pass_by_value)]

extern crate proc_macro;

mod error;

use crate::error::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitStr, Visibility};

struct Arg {
    vis: Visibility,
    path: LitStr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            vis: input.parse()?,
            path: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Arg);
    let vis = &input.vis;
    let rel_path = input.path.value();

    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(rel_path),
        None => PathBuf::from(rel_path),
    };

    let expanded = match source_file_names(dir) {
        Ok(names) => names.into_iter().map(|name| mod_item(vis, name)).collect(),
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };

    TokenStream::from(expanded)
}


#[proc_macro]
pub fn create_controllers_route(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Arg);
    let vis = &input.vis;
    let rel_path = input.path.value();
    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(rel_path),
        None => PathBuf::from(rel_path),
    };
    let create_route = match source_file_names(dir.clone()) {
        Ok(names) => names.into_iter().map(|name| route_item(vis, name)).collect(),
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };
    let merge_route = match source_file_names(dir.clone()) {
        Ok(names) =>names.into_iter().map(|name| route_merge_item(vis,name)).collect(),
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };

    let merge_route = quote! {
        #vis let empty_route = Router::new()#merge_route; 
    };
   
    let expanded = quote!{
        #create_route
        #merge_route
    };
    TokenStream::from(expanded)
}




fn route_item(vis: &Visibility, name: String) -> TokenStream2 {
    let ident = Ident::new(&name, Span::call_site());
    let route_name =  format!("{}_route", name);
    let route_fn = format!("create_{}_router", name);
    let name_ident = Ident::new(&route_name, Span::call_site());
    let fn_ident = Ident::new(&route_fn, Span::call_site());
    quote! {
        #vis let #name_ident = #ident::#fn_ident(config);
    }
    // #vis app.merge(#name_ident);
}

fn route_merge_item(vis: &Visibility, name: String) -> TokenStream2 {
    let route_name =  format!("{}_route", name);
    let name_ident = Ident::new(&route_name, Span::call_site());
    quote! {
        #vis .merge(#name_ident)
    }
}


fn mod_item(vis: &Visibility, name: String) -> TokenStream2 {
    if name.contains('-') {
        let path = format!("{}.rs", name);
        let ident = Ident::new(&name.replace('-', "_"), Span::call_site());
        quote! {
            #[path = #path]
            #vis pub mod #ident;
        }
    } else {
        let ident = Ident::new(&name, Span::call_site());
        quote! {
            #vis pub mod #ident;
        }
    }
}

fn source_file_names<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut failures = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
            continue;
        }

        let path = Path::new(&file_name);
        if path.extension() == Some(OsStr::new("rs")) {
            match file_name.into_string() {
                Ok(mut utf8) => {
                    utf8.truncate(utf8.len() - ".rs".len());
                    names.push(utf8);
                }
                Err(non_utf8) => {
                    failures.push(non_utf8);
                }
            }
        }
    }

    failures.sort();
    if let Some(failure) = failures.into_iter().next() {
        return Err(Error::Utf8(failure));
    }

    if names.is_empty() {
        return Err(Error::Empty);
    }

    names.sort();
    Ok(names)
}