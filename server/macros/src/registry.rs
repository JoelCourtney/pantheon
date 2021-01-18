use std::collections::{HashMap, HashSet};
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use inflector::Inflector;

pub(crate) fn registry(declared_content_files: usize) -> TokenStream {
    let mut counted_content_files: usize = 0;

    let registration = collect_registration();

    let mut content_functions = quote! {};
    let mut registry_statics = quote! {};
    for (key, entries) in &registration {
        let type_string_upper = key.to_pascal_case();
        let type_ident_upper = format_ident!("{}", type_string_upper);
        let type_string_lower = key.to_string();
        let type_ident_lower= format_ident!("{}", type_string_lower);
        let static_ident = format_ident!("{}", key.to_uppercase());

        let get_all_ident = format_ident!("get_all_{}", type_ident_lower);
        let default_ident = format_ident!("default_{}", type_ident_lower);
        let unknown_ident = format_ident!("unknown_{}", type_ident_lower);
        content_functions = quote! {
            #content_functions
            pub fn #type_ident_lower(search_name: &str) -> Option<Box<dyn #type_ident_upper>> {
                match #static_ident.get(search_name) {
                    Some((_, construct)) => Some(construct()),
                    None => None
                }
            }
            pub fn #get_all_ident() -> Vec<&'static str> {
                #static_ident.keys().copied().collect()
            }
            pub fn #default_ident() -> Box<dyn #type_ident_upper> {
                system::defaults::#type_ident_lower::#unknown_ident::new()
            }
        };

        let mut registry_static_entries = quote! {};
        for (collection, source, content) in entries {
            counted_content_files += 1;
            if collection != "system" || source != "defaults" {
                let collection_ident = format_ident!("{}", collection);
                let source_ident = format_ident!("{}", source);
                let content_ident = format_ident!("{}", content);
                registry_static_entries = quote! {
                    #registry_static_entries
                    #collection_ident::#source_ident::#type_ident_lower::#content_ident::CONTENT_NAME => (
                        Registration {
                            collection: #collection_ident::COLLECTION_NAME,
                            source: #collection_ident::#source_ident::COLLECTION_NAME
                        },
                        #collection_ident::#source_ident::#type_ident_lower::#content_ident::new as fn() -> Box<dyn #type_ident_upper>
                    ),
                }
            }
        }
        registry_statics = quote! {
            #registry_statics
            #[derive(Debug)]
            static ref #static_ident: HashMap<&'static str, (Registration, fn() -> Box<dyn #type_ident_upper>)> = hashmap! {
                #registry_static_entries
            };
        };
    }
    if counted_content_files == declared_content_files {
        (quote! {
            use std::collections::HashMap;
            use crate::character::*;
            use maplit::hashmap;
            use lazy_static::lazy_static;

            lazy_static! {
                #registry_statics
            }

            #content_functions
        }).into()
    } else {
        format!(
            r#"compile_error!("content file count mismatch: expected {}, found {}");"#,
            declared_content_files,
            counted_content_files
        ).parse().unwrap()
    }
}

fn collect_registration() -> HashMap<
    String /*type name*/,
    HashSet<(
        String /*collection dirname*/,
        String /*source dirname*/,
        String /*content filename*/
    )>
> {
    use std::path::Component::Normal;
    use walkdir::WalkDir;

    let mut result: HashMap<String, HashSet<(String, String, String)>> = HashMap::new();
    for entry in WalkDir::new("src/content") {
        let entry = entry.expect("expected dir entry");
        let file_name = entry.file_name().to_str().expect("bad os str");
        if file_name.rfind(".rs") != None && file_name != "mod.rs" {
            let comps: Vec<std::path::Component> = entry.path().components().collect();
            if comps.len() == 6 {
                let collection = match comps.get(2).expect("not enough components: 2") {
                    Normal(s) => s.to_str().expect("bad os str: 2").to_owned(),
                    _ => panic!()
                };
                let source = match comps.get(3).expect("not enough components: 3") {
                    Normal(s) => s.to_str().expect("bad os str: 3").to_owned(),
                    _ => panic!()
                };
                let typ = match comps.get(4).expect("not enough components: 4") {
                    Normal(s) => s.to_str().expect("bad os str: 4").to_owned(),
                    _ => panic!()
                };
                let content = file_name[..file_name.len() - 3].to_string();
                if !result.contains_key(&typ) {
                    result.insert(typ.to_owned(), HashSet::new());
                }
                result.get_mut(&typ)
                    .expect("how did we get here")
                    .insert((collection.to_owned(), source.to_owned(), content.to_owned()));
            }
        }
    }
    result
}