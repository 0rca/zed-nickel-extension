use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct NLSBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct NickelExtension {}

fn language_server_binary(worktree: &zed::Worktree) -> Result<NLSBinary> {
    let binary_settings = LspSettings::for_worktree("nickel", worktree)
        .ok()
        .and_then(|lsp_settings| lsp_settings.binary);
    let binary_args = binary_settings
        .as_ref()
        .and_then(|binary_settings| binary_settings.arguments.clone());

    if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
        return Ok(NLSBinary {
            path,
            args: binary_args,
        });
    }

    if let Some(path) = worktree.which("nls") {
        return Ok(NLSBinary {
            path,
            args: binary_args,
        });
    }

    Result::Err("Language server binary not found".to_string())
}

impl zed::Extension for NickelExtension {
    fn new() -> Self {
        NickelExtension {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let nls_binary = language_server_binary(worktree)?;
        Ok(zed::Command {
            command: nls_binary.path,
            args: nls_binary.args.unwrap_or_else(|| vec![]),
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(NickelExtension);
