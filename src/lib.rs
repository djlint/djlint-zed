//! The djLint extension for Zed: runs djLint as a language server for HTML
//! templates.

#![deny(
    warnings,
    ambiguous_negative_literals,
    deprecated_in_future,
    deprecated_safe,
    deref_into_dyn_supertrait,
    ffi_unwind_calls,
    future_incompatible,
    impl_trait_redundant_captures,
    keyword_idents,
    let_underscore,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_docs,
    non_ascii_idents,
    nonstandard_style,
    raw_borrows_via_references,
    redundant_imports,
    redundant_lifetimes,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unsafe_code,
    unstable_features,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::absolute_paths,
    clippy::allow_attributes_without_reason,
    clippy::arbitrary_source_item_ordering,
    clippy::as_conversions,
    clippy::blanket_clippy_restriction_lints,
    clippy::cast_precision_loss,
    clippy::cognitive_complexity,
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::integer_division_remainder_used,
    clippy::iter_over_hash_type,
    clippy::min_ident_chars,
    clippy::missing_docs_in_private_items,
    clippy::mod_module_files,
    clippy::multiple_crate_versions,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::separated_literal_suffix,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::single_call_fn,
    clippy::single_char_lifetime_names,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use zed_extension_api::{
    self as zed, LanguageServerId, Result, serde_json, settings::LspSettings,
};

const SERVER: &str = "djlint";

struct DjlintExtension;

impl DjlintExtension {
    fn command(worktree: &zed::Worktree) -> Result<zed::Command> {
        let (path, arguments, env) =
            LspSettings::for_worktree(SERVER, worktree)
                .ok()
                .and_then(move |settings| settings.binary)
                .map_or((None, None, None), move |binary| {
                    (binary.path, binary.arguments, binary.env)
                });

        let mut args =
            arguments.unwrap_or_else(move || vec![String::from("--lsp")]);
        let command = if let Some(command) =
            path.or_else(move || worktree.which(SERVER))
        {
            command
        } else if let Some(command) = worktree.which("uvx") {
            args.splice(
                0..0,
                ["--from", "djlint>=1.49", "djlint"].map(String::from),
            );
            command
        } else {
            return Err(String::from(
                "djLint is not on PATH. Install it with `pip install djlint`, \
                 or install uv, which runs it with no install.",
            ));
        };

        let mut shell_env = worktree.shell_env();
        shell_env.extend(env.into_iter().flatten());
        Ok(zed::Command { command, args, env: shell_env })
    }
}

#[expect(clippy::missing_trait_methods)]
impl zed::Extension for DjlintExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Self::command(worktree)
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(move |settings| settings.initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(move |settings| settings.settings)
            .map(move |settings| serde_json::json!({ SERVER: settings })))
    }
}

mod registration;
