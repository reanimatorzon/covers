//! **This internal crate stores the implementation of macro**
//!
//! Keeping outside of the primary crate for testing purposes.
//!
//! @see [https://github.com/dtolnay/proc-macro-hack]()

#![feature(proc_macro_def_site)]

use std::collections::HashMap;

use proc_macro::Delimiter::{Brace, Parenthesis};
use proc_macro::*;

use Stage::*;

#[cfg(feature = "_")]
const ORIGINAL_FUNC_PREFIX: &str = "_";
#[cfg(feature = "__")]
const ORIGINAL_FUNC_PREFIX: &str = "__";

#[cfg(feature = "_orig_")]
const ORIGINAL_FUNC_PREFIX: &str = "_orig_";

#[derive(Clone, Copy)]
enum Stage {
    Start = 0,
    FnIdentFound = 1,
    FnNameFound = 2,
    FnArgsFound = 3,
    FnBodyFound = 4,
}

#[derive(Default)]
struct Params {
    reference: String,
    options: HashMap<String, String>,
}

/// Mocks an underline function wrapping its call
///
/// In most cases you need to pass only the single required argument
/// fully-qualified reference to a mock function.
///
/// There only one exception when you need to hint
/// macro with `scope = impl` when you try to mock
/// static struct method (in `impl` block).
///
/// Usage
/// ======
/// ```
/// #[covers(mock_foo)]
/// fn foo(name: &str) -> String {
///     format!("Response: Foo = {}", name)
/// }
///
/// fn mock_foo(another_name: &str) -> String {
///     format!("Response: Mocked(Foo = {})", another_name)
/// }
///
/// #[covers(module::mock_bar)]
/// fn bar(name: &str) -> String {
///     format!("Response: Bar = {}", name)
/// }
///
/// mod module {
///     use super::*;
///
///     pub fn mock_bar(name: &str) -> String {
///         let original_function_result = _bar(name);
///         format!("Response: Mocked({})", original_function_result)
///     }
///
///     pub fn yyy(this: Struct, name: &str) -> String {
///         format!("Response: Mocked({})", name)
///     }
/// }
///
/// pub struct Struct {}
///
/// impl Struct {
///     #[covers(Struct::mock_baz, scope = impl)]
///     fn baz(name: &str) -> String {
///         format!("Response: Baz = {}", name)
///     }
///
///     fn mock_baz(name: &str) -> String {
///         format!("Response: Baz = {}", name)
///     }
///
///     #[covers(module::yyy)]
///     fn xxx(self, name: &str) -> String {
///         format!("Response: Baz = {}", name)
///     }
/// }
///
/// assert_eq!(foo("John"), "1");
/// assert_eq!(bar("Jane"), "1");
/// ```
#[proc_macro_attribute]
pub fn covers(args: TokenStream, input: TokenStream) -> TokenStream {
    if cfg!(not(debug_assertions)) {
        return input;
    }

    let args = parse_params(args);

    let mut stage = Start;

    let mut original = vec![];
    let mut signature = vec![];

    let mut fn_orig_name = String::new();
    let mut fn_args_string = String::new();
    let mut fn_is_public = false;

    // FIXME: dirty hack for 'Self::' prefix to functions inside 'impl' block.
    let mut is_impl_scope = false;

    for token in input {
        match &token {
            TokenTree::Ident(ident) if cmp(&stage, FnIdentFound) < 0 => {
                if ident.to_string() == "pub" {
                    fn_is_public = true;
                    signature.push(token.clone());
                    original.push(token);
                } else if ident.to_string() == "fn" {
                    stage = FnIdentFound;
                    signature.push(token.clone());
                    if !&fn_is_public {
                        original.push(TokenTree::from(Ident::new("pub", Span::def_site())));
                    }
                    original.push(token);
                }
            },
            TokenTree::Ident(ident) if cmp(&stage, FnIdentFound) == 0 => {
                stage = FnNameFound;

                signature.push(create_name_token("", ident));

                let new_token = create_name_token(ORIGINAL_FUNC_PREFIX, ident);
                fn_orig_name = new_token.to_string();
                original.push(new_token);
            },
            TokenTree::Group(group) if cmp(&stage, FnArgsFound) < 0 && group.delimiter() == Parenthesis => {
                stage = FnArgsFound;
                fn_args_string = parse_args(group);
                is_impl_scope = fn_args_string.starts_with("self,") || fn_args_string == "self";
                signature.push(token.clone());
                original.push(token);
            },
            TokenTree::Group(group) if cmp(&stage, FnBodyFound) < 0 && group.delimiter() == Brace => {
                stage = FnBodyFound;
                original.push(token);
            },
            _ => {
                if cmp(&stage, FnBodyFound) < 0 {
                    signature.push(token.clone());
                }
                original.push(token);
            },
        };
    }

    // FIXME: dirty hack for 'Self::' prefix to functions inside 'impl' block.
    is_impl_scope = is_impl_scope || args.options.get("scope").filter(|scope| *scope == "impl").is_some();

    let code = format!(
        r#"
        {fn_original}

        {signature} {{
            #[cfg(test)]
            return {fn_mock_name}{arguments};
            #[cfg(not(test))]
            return {fq}{fn_orig_name}{arguments};
        }}
        "#,
        fn_original = original.into_iter().collect::<TokenStream>(),
        fn_orig_name = fn_orig_name,
        fn_mock_name = args.reference,
        signature = signature.into_iter().collect::<TokenStream>(),
        arguments = format!("({})", fn_args_string),
        fq = if is_impl_scope { "Self::" } else { "" }
    );

    code.parse::<TokenStream>().unwrap().into_iter().collect()
}

fn parse_params(args: TokenStream) -> Params {
    let params = args.to_string();
    let mut params: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
    assert!(
        !params.is_empty(),
        "At least fully-qualified reference to mock have to be provided!"
    );

    let mut response = Params::default();
    response.reference = params.remove(0).trim().to_string();
    for param in params {
        let entry: Vec<String> = param
            .split('=')
            .map(|s| s.trim().to_lowercase())
            .map(String::from)
            .collect();
        assert!(
            entry.len() == 2,
            "Extra parameters should be provided in `key = value` format!"
        );
        response.options.insert(entry[0].to_owned(), entry[1].to_owned());
    }
    response
}

fn create_name_token(prefix: &str, token: &Ident) -> TokenTree {
    TokenTree::from(Ident::new(&format!("{}{}", prefix, token.to_string()), token.span()))
}

fn parse_args(group: &Group) -> String {
    if group.stream().is_empty() {
        return "".to_string();
    }

    let mut vec = vec![];
    let mut args = vec![];

    for token in group.stream() {
        if let TokenTree::Punct(punct) = &token {
            if punct.to_string() == "," {
                args.push(parse_one_arg(&vec));
                vec.clear();
                continue;
            }
        }
        vec.push(token);
    }
    if !vec.is_empty() {
        args.push(parse_one_arg(&vec));
    }
    args.join(", ")
}

fn parse_one_arg(vec: &[TokenTree]) -> String {
    if vec.iter().last().unwrap().to_string() == "self" {
        "self".to_string()
    } else {
        vec[0].to_string()
    }
}

#[allow(clippy::clone_on_copy)]
fn cmp(current: &Stage, expected: Stage) -> i8 {
    (current.clone() as i8) - (expected as i8)
}
