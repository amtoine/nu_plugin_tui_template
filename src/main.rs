use std::io::IsTerminal;

use nu_plugin::{
    serve_plugin, EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand,
    SimplePluginCommand,
};
use nu_protocol::{Example, LabeledError, Signature, Span, Type, Value};

struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Template)]
    }
}

struct Template;

impl SimplePluginCommand for Template {
    type Plugin = TemplatePlugin;

    fn name(&self) -> &str {
        "nu_plugin_tui_template"
    }

    fn usage(&self) -> &str {
        "A simple Nushell plugin using a TUI."
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self)).input_output_type(Type::Nothing, Type::Nothing)
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["plugin", "template"]
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        vec![Example {
            example: "nu_plugin_tui_template",
            description: "run the TUI template plugin",
            result: None,
        }]
    }

    fn run(
        &self,
        _plugin: &TemplatePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        if !std::io::stdin().is_terminal() {
            return Err(LabeledError::new("Can't start nu_plugin_explore")
                .with_label("must run in a terminal", call.head)
                .with_help(
                    "ensure that you are running in a terminal, and that the plugin is not \
                    communicating over stdio",
                ));
        }

        let foreground = engine.enter_foreground()?;
        nu_plugin_tui_template::run().map_err(|err| match err.downcast_ref::<LabeledError>() {
            Some(err) => err.clone(),
            None => LabeledError::new("unexpected internal error").with_label(
                "could not transform error into ShellError, there was another kind of crash...",
                call.head,
            ),
        })?;
        foreground.leave()?;

        Ok(Value::nothing(Span::unknown()))
    }
}

fn main() {
    serve_plugin(&TemplatePlugin, MsgPackSerializer {})
}
