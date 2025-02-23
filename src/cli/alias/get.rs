use color_eyre::eyre::{eyre, Result};

use crate::cli::command::Command;
use crate::config::Config;
use crate::output::Output;

/// Show an alias for a plugin
///
/// This is the contents of an alias.<PLUGIN> entry in ~/.config/rtx/config.toml
///
#[derive(Debug, clap::Args)]
#[clap(after_long_help = AFTER_LONG_HELP, verbatim_doc_comment)]
pub struct AliasGet {
    /// The plugin to show the alias for
    pub plugin: String,
    /// The alias to show
    pub alias: String,
}

impl Command for AliasGet {
    fn run(self, config: Config, out: &mut Output) -> Result<()> {
        match config.get_all_aliases().get(&self.plugin) {
            Some(plugin) => match plugin.get(&self.alias) {
                Some(alias) => Ok(rtxprintln!(out, "{}", alias)),
                None => Err(eyre!("Unknown alias: {}", &self.alias)),
            },
            None => Err(eyre!("Unknown plugin: {}", &self.plugin)),
        }
    }
}

static AFTER_LONG_HELP: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</underline></bold>
 $ <bold>rtx alias get nodejs lts/hydrogen</bold>
 18.0.0
"#
);

#[cfg(test)]
mod tests {
    use insta::{assert_display_snapshot, assert_snapshot};

    use crate::{assert_cli, assert_cli_err, test::reset_config};

    #[test]
    fn test_alias_get() {
        reset_config();
        let stdout = assert_cli!("alias", "get", "tiny", "my/alias");
        assert_snapshot!(stdout, @r###"
        3.0
        "###);
    }

    #[test]
    fn test_alias_get_plugin_unknown() {
        let err = assert_cli_err!("alias", "get", "unknown", "unknown");
        assert_display_snapshot!(err, @"Unknown plugin: unknown");
    }

    #[test]
    fn test_alias_get_alias_unknown() {
        let err = assert_cli_err!("alias", "get", "tiny", "unknown");
        assert_display_snapshot!(err, @"Unknown alias: unknown");
    }
}
