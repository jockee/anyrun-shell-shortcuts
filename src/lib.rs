use std::{collections::HashMap, env, fs, process::Command};

use abi_stable::std_types::{
        ROption::{self, RNone, RSome},
        RString, RVec,
    };
use anyrun_plugin::*;
use fuzzy_matcher::FuzzyMatcher;
use serde::Deserialize;

#[derive(Deserialize)]
struct Shortcut {
    command: RString,
    keywords: ROption<RString>,
    icon: ROption<RString>,
}

#[derive(Deserialize, Default)]
struct Config {
    shell: ROption<RString>,
    shortcuts: HashMap<RString, Shortcut>,
}

struct State {
    config: Config,
    shell: RString,
}

#[init]
fn init(config_dir: RString) -> State {
    let mut config = match fs::read_to_string(format!("{}/shell-shortcuts.ron", config_dir)) {
        Ok(content) => match ron::from_str::<Config>(&content) {
            Ok(config) => config,
            Err(why) => {
                println!(
                    "[Shell Shortcuts] Failed to parse '{}/shell-shortcuts.ron'",
                    config_dir
                );
                println!("[Shell Shortcuts] Error: '{}'", why);
                Config::default()
            }
        },
        Err(_) => Config::default(),
    };

    config.shortcuts.iter_mut().for_each(|(name, shortcut)| {
        let mut keywords = shortcut.keywords.take().unwrap_or_else(RString::new);

        //keywords.push(' ');
        //keywords.push_str(&shortcut.command);
        keywords.push(' ');
        keywords.push_str(name);

        shortcut.keywords = RSome(keywords);
    });

    let shell = if let RSome(shell) = &config.shell {
        shell.clone()
    } else {
        env::var("SHELL")
            .unwrap_or_else(|_| "bash".to_owned())
            .into()
    };

    State { config, shell }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Shell Shortcuts".into(),
        icon: "utilities-terminal".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().smart_case();

    state
        .config
        .shortcuts
        .iter()
        .filter(|(_, val)| match &val.keywords {
            RSome(keywords) => matcher.fuzzy_indices(&keywords, &input).is_some(),
            RNone => unreachable!("Keywords is undefined"),
        })
        .map(|(key, val)| Match {
            title: key.clone(),
            description: RNone,
            use_pango: false,
            icon: RSome(val.command.clone()),
            id: RNone,
        })
        .collect()
}

#[handler]
fn handler(selection: Match, state: &State) -> HandleResult {
    let command = selection.icon.unwrap();
    if let Err(why) = Command::new(state.shell.as_str())
        .arg("-c")
        .arg(command.as_str())
        .spawn()
    {
        println!(
            "[Shell Shortcuts] Failed to run command: {}",
            command.as_str()
        );
        println!("[Shell Shortcuts] Error: {}", why);
    }

    HandleResult::Close
}
