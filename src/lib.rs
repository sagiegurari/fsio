#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    deref_nullptr,
    drop_bounds,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    forbidden_lint_groups,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    invalid_doc_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unsupported_calling_conventions,
    unsupported_naked_functions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    useless_deprecated
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # fsio
//!
//! File System and Path utility functions.
//!
//! # Usage
//!
//! There are multiple main modules for fsio:
//!
//! * fsio::path - Holds path related functions and traits. They do not directly modify the file system.
//! * fsio::file - File utility functions such as read_file, write_file, ...
//! * fsio::directory - Directory specific utility functions.
//!
//! ### Examples
//!
//! ```rust
//! use fsio::{directory, file, path};
//! use std::fs::File;
//! use std::io::Write;
//! use std::path::Path;
//! use std::str;
//!
//! fn main() {
//!     // file operations
//!     let mut result = file::ensure_exists("./target/__test/doc/example/file_test/dir1/dir2/file.txt");
//!     assert!(result.is_ok());
//!
//!     // create/append and read text files
//!     let mut file_path = "./target/__test/example/doc/file_test/append_text_file/file.txt";
//!     result = file::write_text_file(file_path, "some content");
//!     assert!(result.is_ok());
//!     result = file::append_text_file(file_path, "\nmore content");
//!     assert!(result.is_ok());
//!     let mut text = file::read_text_file(file_path).unwrap();
//!     assert_eq!(text, "some content\nmore content");
//!
//!     // create/append and read binary files
//!     file_path = "./target/__test/example/doc/file_test/append_and_read_file_test/file.txt";
//!     result = file::write_file(file_path, "some content".as_bytes());
//!     assert!(result.is_ok());
//!     result = file::append_file(file_path, "\nmore content".as_bytes());
//!     assert!(result.is_ok());
//!     let data = file::read_file(file_path).unwrap();
//!     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
//!
//!     // custom writing
//!     file_path = "./target/__test/example/doc/file_test/modify_file/file.txt";
//!     result = file::modify_file(
//!         file_path,
//!         &move |file: &mut File| file.write_all("some content".as_bytes()),
//!         false,
//!     );
//!     assert!(result.is_ok());
//!     text = file::read_text_file(file_path).unwrap();
//!     assert_eq!(text, "some content");
//!
//!     // delete file
//!     result = file::delete(file_path);
//!     assert!(result.is_ok());
//!
//!     // directory operations
//!     result = directory::create("./target/__test/example/doc/directory_test/dir1/dir2");
//!     assert!(result.is_ok());
//!
//!     result = directory::create_parent("./target/__test/example/doc/directory_test/dir1/files/file.txt");
//!     assert!(result.is_ok());
//!
//!     // delete directory
//!     result = directory::delete("./target/__test/example/doc/directory_test");
//!     assert!(result.is_ok());
//!
//!     // basename and parent directory examples
//!     let basename = path::get_basename("./src/path/mod.rs");
//!     assert_eq!(basename.unwrap(), "mod.rs");
//!
//!     let dirname = path::get_parent_directory("./src/path/mod.rs");
//!     assert_eq!(dirname.unwrap(), "./src/path");
//!
//!     // canonicalize examples
//!     let path_obj = Path::new("./src/path/mod.rs");
//!
//!     let path1 = path::canonicalize_as_string(&path_obj);
//!     let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");
//!
//!     assert_eq!(path1.unwrap(), path2);
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! fsio = "*"
//! ```
//!
//! If you need access to temporary file paths, enable the **temp-path** feature as follows:
//!
//! ```ini
//! [dependencies]
//! fsio = { version = "*", features = ["temp-path"] }
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/fsio/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/fsio/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
use doc_comment as _;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod directory;
pub mod error;
pub mod file;
pub mod path;
pub mod types;
