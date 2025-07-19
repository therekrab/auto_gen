// There are commands - the final result of calculations.

#[derive(Debug, Clone)]
pub enum Command {
    Named(String),
    Group(GroupKind, Vec<Command>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupKind {
    Parallel,
    Sequential,
}

impl Command {
    pub fn to_json(&self) -> String {
        match self {
            Self::Named(name) => named_json(name),
            Self::Group(group, commands) => named_group(group, commands),
        }
    }

    fn unpack_group(&self, kind: &GroupKind) -> Option<Vec<Command>> {
        match self {
            Self::Named(_) => None,
            Self::Group(group_kind, inner) => {
                if group_kind != kind {
                    return None;
                }
                Some(inner.clone())
            }
        }
    }
}

fn named_json(name: &str) -> String {
    format!(r#"{{"type":"named","data":{{"name":"{name}"}}}}"#,)
}

fn named_group(group: &GroupKind, commands: &[Command]) -> String {
    let commands_json = commands
        .iter()
        .map(|cmd| cmd.to_json())
        .collect::<Vec<String>>()
        .join(",");
    let kind = group.name();
    format!(r#"{{"type":"{kind}","data":{{"commands":[{commands_json}]}}}}"#)
}

pub fn finalize_json(json: &str) -> String {
    format!(
        r#"{{"version":"2025.0","command":{json},"resetOdom":false,"folder":null,"choreoAuto":false}}"#
    )
}

impl GroupKind {
    fn name(&self) -> String {
        match self {
            Self::Sequential => "sequence",
            Self::Parallel => "parallel",
        }
        .to_string()
    }

    pub fn group(&self, commands: &[Command]) -> Command {
        let mut final_commands = Vec::new();
        for cmd in commands {
            // if cmd is the same kind as self, then we just push all of its contents into
            // final_commands. Otherwise, we can just push the command itself.
            if let Some(mut inner) = cmd.unpack_group(self) {
                final_commands.append(&mut inner);
            } else {
                final_commands.push(cmd.clone());
            }
        }
        Command::Group(self.clone(), final_commands)
    }
}

// This is a helpful util for debugging, not really useful in the app itself.
impl Command {
    #[allow(dead_code)]
    pub fn pretty(&self) -> String {
        match self {
            Self::Named(name) => format!("<{name}>"),
            Self::Group(group, cmds) => {
                let pretty_cmds = cmds
                    .iter()
                    .map(Command::pretty)
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("[{}: {}]", group.name(), pretty_cmds)
            }
        }
    }
}
