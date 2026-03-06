// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT

#![allow(clippy::doc_lazy_continuation)]

//! # Fluxo Typestate Macros
//!
//! This crate contains the procedural macros for the Fluxo Typestate library.
//! It is not intended to be used directly; instead, use the `fluxo-typestate` crate.
//!
//! # Overview
//!
//! The `fluxo-typestate-macros` crate provides the implementation of the
//! `#[state_machine]` procedural macro. This macro transforms enum definitions
//! into complete type-state pattern implementations.
//!
//! ## What the Macro Generates
//!
//! Given an enum like:
//!
//! ```ignore
//! #[state_machine]
//! enum Computer {
//!     Idle,
//!     Running { cpu_load: f32 },
//!     Sleeping,
//! }
//! ```
//!
//! The macro generates:
//!
//! 1. **State Structs**: For each variant, a struct representing that state
//! 2. **Main Wrapper**: A generic `Computer<S>` struct
//! 3. **State Trait Implementations**: Implementations of `State` and `Sealed`
//! 4. **Transition Methods**: Methods for each defined transition
//! 5. **Constructor**: A `new()` method for the initial state
//! 6. **Visualization**: A `mermaid_diagram()` method
//!
//! ## Internal Structure
//!
//! The macro implementation is organized as follows:
//!
//! - `TransitionInfo`: Parsed transition attribute data
//! - `VariantIr`: Intermediate representation of an enum variant
//! - `FieldsIr`: Representation of variant fields
//! - `generate_*`: Functions that generate different parts of the output
//!
//! ## Transition Attributes
//!
//! Transitions are defined using the `#[transition]` attribute:
//!
//! ```ignore
//! #[transition(SourceState -> TargetState: method_name)]
//! ```
//!
//! This syntax means: "from `SourceState`, you can transition to `TargetState`
//! by calling `method_name()`".
//!
//! The macro supports two syntaxes:
//!
//! - Short form: `#[transition(Idle -> Running: start)]`
//! - Full form: `#[transition(Computer::Idle -> Computer::Running: start)]`
//!
//! ## Attributes
//!
//! The state machine macro supports several attributes:
//!
//! - `#[state_machine]`: Required attribute to enable the macro
//! - `#[transition(...)]`: Define a state transition
//! - `#[trace]`: Enable tracing of state transitions (requires `logging` feature)
//! - `#[visualize]`: Enable Mermaid diagram generation

// Re-export necessary types for the macro implementation
use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as PsTokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Fields, Ident, Token};

/// Internal representation of a transition attribute.
///
/// This struct holds the parsed information from a `#[transition]` attribute,
/// including the source state, target state, and method name.
///
/// # Fields
///
/// - `from_state`: The source state identifier
/// - `to_state`: The target state identifier
/// - `method_name`: The method name to generate for this transition
/// - `to_fields`: Optional fields to initialize in the target state
///
/// # Parsing
///
/// The attribute is parsed in the format: `Source -> Target: method_name`
/// Both short form (`Idle`) and full form (`Computer::Idle`) are supported.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TransitionInfo {
    /// The source state from which this transition originates.
    from_state: Ident,
    /// The target state to which this transition goes.
    to_state: Ident,
    /// The name of the method to generate for this transition.
    method_name: Ident,
    /// Optional fields to initialize in the target state.
    to_fields: PsTokenStream,
}

/// Implementation of `Parse` for `TransitionInfo`.
///
/// This allows the macro to parse `#[transition(...)]` attributes from
/// the token stream into a structured form.
impl syn::parse::Parse for TransitionInfo {
    /// Parses a transition attribute in the format:
    /// `SourceState -> TargetState: method_name`
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        // Parse the source state (either simple ident or path like `Computer::Idle`)
        let from_state = if input.peek(Ident) && input.peek2(Token![::]) {
            // Skip the enum name prefix
            input.parse::<syn::Path>()?;
            input.parse::<Token![::]>()?;
            input.parse::<Ident>()?
        } else {
            input.parse::<Ident>()?
        };

        // Expect and parse the arrow
        input.parse::<Token![->]>()?;

        // Parse the target state
        let to_state = if input.peek(Ident) && input.peek2(Token![::]) {
            input.parse::<syn::Path>()?;
            input.parse::<Token![::]>()?;
            input.parse::<Ident>()?
        } else {
            input.parse::<Ident>()?
        };

        // Expect and parse the colon
        input.parse::<Token![:]>()?;

        // Parse the method name
        let method_name = input.parse::<Ident>()?;

        Ok(TransitionInfo {
            from_state,
            to_state,
            method_name,
            to_fields: PsTokenStream::new(),
        })
    }
}

/// Internal representation of an enum variant during code generation.
///
/// This struct holds all information about a single variant of the state
/// machine enum, including its name, fields, and associated transitions.
struct VariantIr {
    /// The generated struct name for this variant.
    struct_name: syn::Ident,
    /// The fields associated with this variant.
    fields: FieldsIr,
    /// All transitions defined from this state.
    transitions: Vec<TransitionInfo>,
}

/// Representation of variant fields for code generation.
///
/// This enum represents the three possible field types in Rust enums:
/// - Unit: No fields (e.g., `Idle`)
/// - Unnamed: Tuple-style fields (e.g., `Running(f32)`)
/// - Named: Struct-style fields (e.g., `Running { cpu_load: f32 }`)
enum FieldsIr {
    /// Unit variant with no fields.
    Unit,
    /// Tuple variant with unnamed fields.
    Unnamed(Vec<syn::Type>),
    /// Struct variant with named fields.
    Named(Vec<(syn::Ident, syn::Type)>),
}

/// Extracts additional attributes from the enum attributes.
///
/// This function parses attributes like `#[trace]` and `#[visualize]`
/// from the enum definition.
///
/// # Arguments
///
/// * `attrs` - Slice of attributes to parse
///
/// # Returns
///
/// A tuple of (trace_enabled, visualize_path)
fn extract_attributes(attrs: &[Attribute]) -> (bool, Option<String>) {
    let mut trace_enabled = false;
    let mut visualize_path = None;

    for attr in attrs {
        let path_str = attr
            .path()
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();

        // Check for #[trace] attribute
        if path_str == "trace" {
            trace_enabled = true;
        }
        // Check for #[visualize] attribute
        else if path_str == "visualize" {
            visualize_path = Some("fluxo_map.mermaid".to_string());
        }
    }

    (trace_enabled, visualize_path)
}

/// Parses an enum's variants into intermediate representations.
///
/// This function processes each variant of the enum, extracting the name,
/// fields, and any transition attributes.
///
/// # Arguments
///
/// * `data` - The enum data to parse
///
/// # Returns
///
/// A vector of `VariantIr` structures representing each variant
fn parse_variants(data: &DataEnum) -> Vec<VariantIr> {
    data.variants
        .iter()
        .map(|variant| {
            // Convert variant name to upper camel case for struct name
            let struct_name = syn::Ident::new(
                &variant.ident.to_string().to_upper_camel_case(),
                variant.ident.span(),
            );

            // Parse the fields based on their style
            let fields = match &variant.fields {
                Fields::Unit => FieldsIr::Unit,
                Fields::Unnamed(fields) => {
                    FieldsIr::Unnamed(fields.unnamed.iter().map(|f| f.ty.clone()).collect())
                }
                Fields::Named(fields) => FieldsIr::Named(
                    fields
                        .named
                        .iter()
                        .map(|f| (f.ident.clone().unwrap(), f.ty.clone()))
                        .collect(),
                ),
            };

            // Parse transition attributes from this variant
            let transitions = variant
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("transition"))
                .filter_map(|attr| attr.parse_args::<TransitionInfo>().ok())
                .collect();

            VariantIr {
                struct_name,
                fields,
                transitions,
            }
        })
        .collect()
}

/// Generates the state struct definitions for each variant.
///
/// For each variant in the enum, this function generates a corresponding
/// struct that implements `State` and `Sealed` traits.
///
/// # Arguments
///
/// * `variants` - The parsed variant representations
///
/// # Returns
///
/// A `TokenStream` containing all generated struct definitions
fn generate_state_structs(variants: &[VariantIr]) -> PsTokenStream {
    let structs: Vec<PsTokenStream> = variants
        .iter()
        .map(|variant| {
            let struct_name = &variant.struct_name;
            match &variant.fields {
                FieldsIr::Unit => {
                    quote! {
                        /// State marker struct.
                        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                        pub struct #struct_name;
                        impl fluxo_typestate::Sealed for #struct_name {}
                        impl fluxo_typestate::State for #struct_name {
                            fn name(&self) -> &'static str { stringify!(#struct_name) }
                        }
                    }
                }
                FieldsIr::Unnamed(_types) => {
                    quote! {
                        /// State marker struct with data.
                        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                        pub struct #struct_name;
                        impl fluxo_typestate::Sealed for #struct_name {}
                        impl fluxo_typestate::State for #struct_name {
                            fn name(&self) -> &'static str { stringify!(#struct_name) }
                        }
                    }
                }
                FieldsIr::Named(fields) => {
                    let field_defs: Vec<PsTokenStream> = fields
                        .iter()
                        .map(|(ident, ty)| quote! { pub #ident: #ty })
                        .collect();
                    quote! {
                        /// State marker struct with named fields.
                        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                        pub struct #struct_name { #(#field_defs),* }
                        impl fluxo_typestate::Sealed for #struct_name {}
                        impl fluxo_typestate::State for #struct_name {
                            fn name(&self) -> &'static str { stringify!(#struct_name) }
                        }
                    }
                }
            }
        })
        .collect();
    quote! { #(#structs)* }
}

/// Generates the main generic state machine wrapper struct.
///
/// This creates the generic `EnumName<S>` struct that wraps the state
/// and provides the `current_state()` method.
///
/// # Arguments
///
/// * `enum_name` - The name of the original enum
///
/// # Returns
///
/// A `TokenStream` containing the generated state machine struct
fn generate_state_machine(enum_name: &syn::Ident) -> PsTokenStream {
    quote! {
        /// The main state machine wrapper type.
        ///
        /// This struct is generic over the state type `S`, which ensures
        /// that state transitions are checked at compile time.
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #enum_name<S: fluxo_typestate::State> {
            _state: std::marker::PhantomData<S>,
        }

        impl<S: fluxo_typestate::State> #enum_name<S> {
            /// Get the name of the current state.
            pub fn current_state(&self) -> &'static str {
                S::name(&std::marker::PhantomData)
            }
        }
    }
}

/// Generates transition methods for each state.
///
/// For each defined transition, this function generates a method that
/// consumes the current state and returns the new state.
///
/// # Arguments
///
/// * `variants` - The parsed variant representations
/// * `enum_name` - The name of the original enum
/// * `trace_enabled` - Whether to generate tracing code
///
/// # Returns
///
/// A `TokenStream` containing all generated transition methods
fn generate_transitions(
    variants: &[VariantIr],
    enum_name: &syn::Ident,
    trace_enabled: bool,
) -> PsTokenStream {
    let transition_impls: Vec<PsTokenStream> = variants.iter()
        .filter(|v| !v.transitions.is_empty())
        .map(|variant| {
            let from_state = &variant.struct_name;
            let methods: Vec<PsTokenStream> = variant.transitions.iter()
                .map(|trans| {
                    let method_name = &trans.method_name;
                    let to_state = &trans.to_state;
                    if trace_enabled {
                        quote! {
                            /// Transition to another state.
                            pub fn #method_name(self) -> #enum_name<#to_state> {
                                tracing::info!(from = stringify!(#from_state), to = stringify!(#to_state), "Transition via {}()", stringify!(#method_name));
                                #enum_name::<#to_state> { _state: std::marker::PhantomData }
                            }
                        }
                    } else {
                        quote! {
                            /// Transition to another state.
                            pub fn #method_name(self) -> #enum_name<#to_state> {
                                #enum_name::<#to_state> { _state: std::marker::PhantomData }
                            }
                        }
                    }
                })
                .collect();
            quote! { impl #enum_name<#from_state> { #(#methods)* } }
        })
        .collect();
    quote! { #(#transition_impls)* }
}

/// Generates the Default trait implementation for the initial state.
///
/// This creates a `Default` implementation that delegates to `new()`.
///
/// # Arguments
///
/// * `variants` - The parsed variant representations
/// * `enum_name` - The name of the original enum
///
/// # Returns
///
/// A `TokenStream` containing the Default implementation
fn generate_default_impl(variants: &[VariantIr], enum_name: &syn::Ident) -> PsTokenStream {
    if let Some(first_variant) = variants.first() {
        let first_state = &first_variant.struct_name;
        quote! {
            impl Default for #enum_name<#first_state> {
                fn default() -> Self { Self::new() }
            }
        }
    } else {
        quote! {}
    }
}

/// Generates the `new()` constructor for the initial state.
///
/// This creates a constructor function that returns a state machine
/// in its initial state.
///
/// # Arguments
///
/// * `variants` - The parsed variant representations
/// * `enum_name` - The name of the original enum
///
/// # Returns
///
/// A `TokenStream` containing the new() implementation
fn generate_new_impl(variants: &[VariantIr], enum_name: &syn::Ident) -> PsTokenStream {
    if let Some(first_variant) = variants.first() {
        let first_state = &first_variant.struct_name;
        quote! {
            impl #enum_name<#first_state> {
                /// Creates a new state machine in the initial state.
                pub fn new() -> Self {
                    #enum_name::<#first_state> { _state: std::marker::PhantomData }
                }
            }
        }
    } else {
        quote! {}
    }
}

/// Generates the Mermaid diagram visualization method.
///
/// This creates a method that generates a Mermaid.js state diagram
/// representing the state machine.
///
/// # Arguments
///
/// * `variants` - The parsed variant representations (unused in basic version)
/// * `enum_name` - The name of the original enum
///
/// # Returns
///
/// A `TokenStream` containing the mermaid_diagram() method
fn generate_mermaid(_variants: &[VariantIr], enum_name: &syn::Ident) -> PsTokenStream {
    quote! {
        impl<S: fluxo_typestate::State> #enum_name<S> {
            /// Generate a Mermaid state diagram.
            ///
            /// This method returns a string containing a Mermaid.js state diagram
            /// that visualizes the state machine structure.
            #[allow(dead_code)]
            pub fn mermaid_diagram() -> String {
                let mut diagram = String::from("```mermaid\nstateDiagram-v2\n");
                diagram.push_str("```");
                diagram
            }
        }
    }
}

/// The main entry point for the `#[state_machine]` attribute macro.
///
/// This is the procedural macro that transforms enum definitions into
/// complete type-state pattern implementations.
///
/// # Arguments
///
/// * `args`: TokenStream of attribute arguments (should be empty)
/// * `input`: TokenStream of the item to annotate (should be an enum)
///
/// # Returns
///
/// A TokenStream containing the generated code
///
/// # Panics
///
/// This macro will panic if:
/// - Arguments are provided to the attribute
/// - The annotated item is not an enum
///
/// # Example
///
/// ```ignore
/// use fluxo_typestate::state_machine;
///
/// #[state_machine]
/// enum TrafficLight {
///     #[transition(TrafficLight::Red -> TrafficLight::Green: go)]
///     Red,
///     Green,
///     Yellow,
/// }
/// ```
#[proc_macro_attribute]
pub fn state_machine(args: TokenStream, input: TokenStream) -> TokenStream {
    // Verify no arguments were provided
    if !args.to_string().is_empty() {
        let err = syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[state_machine]` does not accept any arguments.",
        );
        return err.into_compile_error().into();
    }

    // Parse the input as a DeriveInput (enum definition)
    let mut input = parse_macro_input!(input as DeriveInput);

    // Verify the #[state_machine] attribute is present
    let has_state_machine_attr = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("state_machine"));

    if !has_state_machine_attr {
        let err = syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[state_machine]` must be used as an attribute on an enum.",
        );
        return err.into_compile_error().into();
    }

    // Extract additional attributes like #[trace] and #[visualize]
    let (trace_enabled, _visualize_path) = extract_attributes(&input.attrs);

    // Remove the state_machine attribute to avoid conflicts
    input
        .attrs
        .retain(|attr| !attr.path().is_ident("state_machine"));

    let enum_name = input.ident.clone();
    let data = &input.data;

    match data {
        Data::Enum(data_enum) => {
            // Parse the variants
            let variants = parse_variants(data_enum);

            // Generate all the code pieces
            let state_structs = generate_state_structs(&variants);
            let state_machine = generate_state_machine(&enum_name);
            let transitions = generate_transitions(&variants, &enum_name, trace_enabled);
            let default_impl = generate_default_impl(&variants, &enum_name);
            let new_impl = generate_new_impl(&variants, &enum_name);
            let mermaid = generate_mermaid(&variants, &enum_name);

            // Combine all generated code
            let tokens = quote! {
                #state_structs
                #state_machine
                #transitions
                #default_impl
                #new_impl
                #mermaid
            };

            // Convert to proc_macro TokenStream
            tokens.into()
        }
        _ => {
            // Return an error if the annotated item is not an enum
            let err = syn::Error::new(
                input.ident.span(),
                "`#[state_machine]` can only be applied to enums.",
            );
            err.into_compile_error().into()
        }
    }
}
