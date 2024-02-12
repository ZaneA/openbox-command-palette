use serde::Deserialize;
use std::process::Command;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;

#[derive(Deserialize)]
struct ActionArgument {
    #[serde(default)]
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Deserialize)]
struct Action {
    name: String, // only care about Execute
    #[serde(default)]
    #[serde(rename = "$value")]
    args: Vec<ActionArgument>,
}

#[derive(Deserialize)]
struct Item {
    label: String,
    #[serde(default)]
    #[serde(rename = "$value")]
    actions: Vec<Action>,
}

#[derive(Deserialize)]
struct Menu {
    id: String,
    label: String,
    execute: Option<String>,
    #[serde(default)]
    #[serde(rename = "$value")]
    items: Vec<Entry>,
}

#[derive(Deserialize)]
struct Separator {
    #[serde(default)]
    _label: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Entry {
    Menu(Menu),
    Item(Item),
    Separator(Separator),
}

#[derive(Deserialize)]
struct OpenboxPipeMenu {
    #[serde(rename = "$value")]
    items: Vec<Entry>,
}

#[derive(Deserialize)]
struct OpenboxMenu {
    #[serde(rename = "$value")]
    menus: Vec<Menu>,
}

fn label_text(prefix: &str, label: &str) -> String {
    let label = label.replace("_", "");

    if prefix.is_empty() {
        label
    } else {
        format!("{} {}", prefix, label)
    }
}

fn print_entries(wrapper_path: &str, prefix: &str, entries: &Vec<Entry>) {
    for entry in entries {
        match entry {
            Entry::Menu(menu) => {
                let prefix = label_text(prefix, &menu.label);

                // since this is a pipe menu, we need to call via our
                // wrapper script in order for the menu to continue being
                // useful
                if let Some(execute) = &menu.execute {
                    println!("{:080} # {} \"{}\"", prefix, wrapper_path, execute);
                }

                print_entries(wrapper_path, &prefix, &menu.items);
            },
            Entry::Item(item) => {
                let prefix = label_text(prefix, &item.label);

                for action in &item.actions {
                    if action.name == "Execute" {
                        for arg in &action.args {
                            println!("{:080} # {}", prefix, arg.value);
                        }
                    }
                }
            },
            Entry::Separator(_) => {},
        }
    }
}

fn execute_menu(wrapper_path: &str, path: &str) {
    if let Ok(result) = Command::new("sh")
        .arg("-c")
        .arg(path)
        .output() {
            let output = String::from_utf8(result.stdout)
                .expect("Couldn't parse output as valid UTF-8");
            
            let menu = serde_xml_rs::from_str::<OpenboxPipeMenu>(&output)
                .expect("Failed to parse pipe menu");
            
            print_entries(&wrapper_path, "", &menu.items);
        }
}

fn main() {
    let mut args = std::env::args();
    let arg0 = args.next().unwrap(); // first arg

    if args.len() == 2 {
        let wrapper_path = args.next().unwrap();
        let menu_path_or_command = args.next().unwrap();

        if let Ok(file) = File::open(&menu_path_or_command) {
            let permissions = file.metadata().unwrap().permissions();
            if permissions.mode() & 0o111 != 0 {
                // file exists and is executable
                execute_menu(&wrapper_path, &menu_path_or_command);
            } else {
                // file exists and is not executable, treat as openbox menu config
                let config = std::fs::read_to_string(&menu_path_or_command).unwrap();
                let menu_root: OpenboxMenu = serde_xml_rs::from_str(&config)
                    .expect("Failed to parse root menu");
                
                for menu in &menu_root.menus {
                    if menu.id.contains("root") {
                        // menu may specify a pipe menu to execute, or
                        // may contain items directly
                        if let Some(execute) = &menu.execute {
                            execute_menu(&wrapper_path, &execute);
                        } else {
                            print_entries(&wrapper_path, "", &menu.items);
                        }
                    }
                }
            }
        } else {
            // file doesn't exist so treat this as a command to be run
            execute_menu(&wrapper_path, &menu_path_or_command);
        };
    } else {
        println!("Usage: {} <path to wrapper> <path to openbox menu.xml or pipe menu command>", arg0);
    }
}
