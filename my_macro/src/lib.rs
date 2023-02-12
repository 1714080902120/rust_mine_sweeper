use syn;

use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, Span, TokenStream, Ident};
use quote::{quote, TokenStreamExt};

use proc_macro;

// struct Interp(TokenStream);

// impl ToTokens for Interp {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         tokens.append(Punct::new('#', Spacing::Alone));
//         tokens.append(self.0);
//     }
// }

#[proc_macro_derive(EasyInspector)]
pub fn easy_inspector_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let ast = syn::parse(input).unwrap();

    import_inspector()
}

fn import_inspector() -> proc_macro::TokenStream {
    // let f_debug = Interp("[cfg(feature = \"debug\")]");
    // let f_debug_derive_ref_inspector =
    //     Interp("[cfg_attr(feature = \"debug\", derive(Reflect, InspectorOptions))]");
    // let f_debug_ref_inspector =
    //     Interp("[cfg_attr(feature = \"debug\", reflect(InspectorOptions))]");
    // let f_debug_inspector = Interp("[cfg_attr(feature = \"debug\", inspector(validate = |ability| ability.current_charges <= ability.max_charges))]");
    // let derive_build_in =
    //     Interp("[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]");

    // let gen = quote! {
    //   use bevy::prelude::Component;
    //   let d = quote::quote!(#derive_build_in);
    //   use bevy_inspector_egui::prelude::*;
    //   quote::quote!(#f_debug);
    //   use bevy::prelude::Reflect;
    //   quote::quote!(#f_debug_derive_ref_inspector);
    //   quote::quote!(#f_debug_ref_inspector);
    //   quote::quote!(#f_debug_inspector);
    //   quote::quote!(#derive_build_in);
    // };
    // gen.into()

    let mut stream = TokenStream::new();
    quote_into::quote_into!(stream += use bevy::prelude::Component;);

    // get_debug_stream(&mut stream);


    proc_macro::TokenStream::from(stream)
}

/// #[cfg(feature = "debug")]
fn get_debug_stream(stream: &mut TokenStream) {
    let pound = Punct::new('#', Spacing::Alone);


    let mut debug_stream = TokenStream::new();

    debug_stream.append(Ident::new("cfg", Span::call_site()));

    let mut inner_stream = TokenStream::new();

    inner_stream.append(Ident::new("feature", Span::call_site()));
    inner_stream.append(Punct::new('=', Spacing::Alone));
    inner_stream.append(Literal::character('"'));
    inner_stream.append(Literal::string("debug"));
    inner_stream.append(Literal::character('"'));

    let parenthesis_group = Group::new(Delimiter::Parenthesis, inner_stream);

    debug_stream.append(parenthesis_group);

    let bracket_group = Group::new(Delimiter::Bracket, debug_stream);


    // add to main stream
    stream.append(pound);
    stream.append(bracket_group);
}



#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() -> String {
                String::from(stringify!(#name))
            }
        }
    };
    gen.into()
}
