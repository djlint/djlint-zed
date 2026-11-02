//! The djLint extension for Zed: runs djLint as a language server for HTML
//! templates.

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
