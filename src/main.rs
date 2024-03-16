mod savefile;

use savefile::{
    get_backup_save_names, get_current_save_names, load_backup_by_name, save_backup_by_name,
};

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn format_save_vec(items: &[String]) -> String {
    items
        .to_owned()
        .iter()
        .enumerate()
        .map(|(i, val)| (i + 1).to_string() + ". " + val)
        .collect::<Vec<String>>()
        .join("\n")
}

fn interaction_backup_create() {
    let mut input = String::new();
    let saves = get_current_save_names().unwrap_or_default();

    loop {
        clear_terminal();

        println!(
            "Current saves:\n\n{}\n\nChoose a number:",
            format_save_vec(&saves)
        );

        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            i if i.is_empty() => break,
            i if i.chars().all(|c| c.is_ascii_digit()) => {
                let index: usize = i.parse().unwrap();

                if index <= saves.len() {
                    match save_backup_by_name(saves.get(index - 1).unwrap()) {
                        Ok(_) => println!("Backup created successfully!"),

                        Err(e) => println!("{:?}", e),
                    }
                    std::io::stdin().read_line(&mut input).unwrap();
                    break;
                }
            }
            _ => {}
        }
    }
}

fn interaction_backup_load() {
    let mut input = String::new();
    let backups = get_backup_save_names().unwrap_or_default();
    loop {
        clear_terminal();

        println!(
            "Current backups:\n\n{}\n\nChoose a number:",
            format_save_vec(&backups)
        );

        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            i if i.is_empty() => break,
            i if i.chars().all(|c| c.is_ascii_digit()) => {
                let index: usize = i.parse().unwrap();

                if index <= backups.len() {
                    match load_backup_by_name(backups.get(index - 1).unwrap()) {
                        Ok(_) => println!("Backup loaded successfully!"),
                        Err(e) => println!("{:?}", e),
                    }
                    std::io::stdin().read_line(&mut input).unwrap();
                    break;
                }
            }
            _ => {}
        }
    }
}

fn main() {
    assert!(
        cfg!(windows),
        "This program is Windows-only. Where are your saves located anyway? I have no idea"
    );

    loop {
        clear_terminal();

        print!("\n\nChoose an option:\n1. Create a backup\n2. Load a backup\n\n: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => interaction_backup_create(),
            "2" => interaction_backup_load(),
            _ => {}
        }
    }
}
