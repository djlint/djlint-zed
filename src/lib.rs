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

        let (command, args) = if let Some(command) =
            path.or_else(move || worktree.which(SERVER))
        {
            let args =
                arguments.unwrap_or_else(move || vec![String::from("--lsp")]);
            (command, args)
        } else if let Some(command) = worktree.which("uvx") {
            let args = arguments.unwrap_or_else(move || {
                ["--from", "djlint>=1.49", "djlint", "--lsp"]
                    .map(String::from)
                    .to_vec()
            });
            (command, args)
        } else {
            return Err(String::from(
                "djLint is not on PATH. Install it with `pip install djlint`, \
                 or install uv, which runs it with no install.",
            ));
        };

        let mut env: Vec<_> =
            env.map_or_else(Vec::new, move |env| env.into_iter().collect());
        env.extend(worktree.shell_env());
        Ok(zed::Command { command, args, env })
    }
}

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

zed::register_extension!(DjlintExtension);
