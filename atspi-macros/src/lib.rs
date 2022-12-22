use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

//
// Derive macro for that implements TryFrom<Event> on a per name / member basis.
//

#[proc_macro_derive(TrySignify)]
pub fn try_signify(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    // Extract the name of the struct
    let name = &ident;

    // Remove the suffix "Event" from the name of the struct
    let name_string = name.to_string();
    let member = name_string.strip_suffix("Event").unwrap();

    // Generate the expanded code
    let expanded = quote! {
        impl TryFrom<AtspiEvent> for #name {
            type Error = crate::AtspiError;

            fn try_from(msg: AtspiEvent) -> Result<Self, Self::Error> {
                let msg_member = msg.member();
                if msg_member == Some(MemberName::from_static_str(#member)?) {
                    return Ok(Self(msg));
                };

                let tname = std::any::type_name::<Self>().to_string();
                let member = tname.strip_suffix("Event").unwrap();
                let error = format!("specific type's member: {} != msg type member: {:?}", member, msg_member);
                Err(crate::AtspiError::MemberMatchError(error))
            }
        }


        impl<'a> #name {
            fn inner(&'a self) -> &'a AtspiEvent {
                &self.0
            }
        }

        impl Signified for #name {
            fn properties(&self) -> &HashMap<String, OwnedValue> {
                self.inner().properties()
            }
        }
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Doc)]
pub fn doc(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Doc for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Win)]
pub fn win(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Win for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Term)]
pub fn term(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Term for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Obj)]
pub fn obj(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Obj for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Mse)]
pub fn mse(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Mse for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Kbd)]
pub fn kbd(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Kbd for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Focus)]
pub fn focus(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Focus for #name {}
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}
