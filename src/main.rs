mod tasks;
mod cli;

use cli::get_matches;
use tasks::{add_task, list_tasks, remove_task, complete_task};

fn main() {
    let matches = get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let description = matches.value_of("description").unwrap();
        add_task(description);
    } else if let Some(_) = matches.subcommand_matches("list") {
        list_tasks();
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches.value_of("id").unwrap().parse::<u32>().unwrap();
        remove_task(id);
    } else if let Some(matches) = matches.subcommand_matches("complete") {
        let id = matches.value_of("id").unwrap().parse::<u32>().unwrap();
        complete_task(id);
    }
}