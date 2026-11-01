use zed_extension_api::{
    self as zed, LanguageServerId, Result, serde_json, settings::LspSettings,
};

const SERVER: &str = "djlint";

struct DjlintExtension;

impl DjlintExtension {
    fn command(worktree: &zed::Worktree) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|settings| settings.binary);
        let (path, arguments, env) =
            binary.map_or((None, None, None), |b| (b.path, b.arguments, b.env));
        let mut env: Vec<_> =
            env.map_or_else(Vec::new, |env| env.into_iter().collect());
        env.extend(worktree.shell_env());

        if let Some(command) = path.or_else(|| worktree.which(SERVER)) {
            return Ok(zed::Command {
                command,
                args: arguments.unwrap_or_else(|| vec!["--lsp".to_owned()]),
                env,
            });
        }
        if let Some(command) = worktree.which("uvx") {
            return Ok(zed::Command {
                command,
                args: arguments.unwrap_or_else(|| {
                    ["--from", "djlint>=1.49", "djlint", "--lsp"]
                        .map(str::to_owned)
                        .to_vec()
                }),
                env,
            });
        }
        Err("djLint is not on PATH. Install it with `pip install djlint`, or \
             install uv, which runs it with no install."
            .to_owned())
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
            .and_then(|settings| settings.initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .map(|settings| serde_json::json!({ SERVER: settings })))
    }
}

zed::register_extension!(DjlintExtension);
