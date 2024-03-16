use lazy_static::lazy_static;
use std::{env, fs, path::PathBuf};

lazy_static! {
    static ref SAVE_DIR_PATHBUF: PathBuf = {
        let mut pb = PathBuf::from(env::var_os("appdata").unwrap());
        pb.pop();
        pb.push("LocalLow\\ZeekerssRBLX\\Lethal Company\\");
        pb
    };
}
fn get_save_dir_item_names() -> Option<impl Iterator<Item = String>> {
    Some(
        fs::read_dir(SAVE_DIR_PATHBUF.as_path())
            .ok()?
            .map(|file| file.unwrap().file_name().into_string().unwrap()),
    )
}

/// Returns a vector of saves' filenames without the path included.
///
/// More specifically, gets all the files that start with `LCSaveFile` inside `%appdata%\..\LocalLow\ZeekerssRBLX\Lethal Company\`,
/// sorted in descending order.
///
/// Returns `None` if the folder is nonexistent or reading it returned an `Err` somehow.
///
/// ```
/// let saves = get_current_save_names().unwrap();
/// assert_eq!(
///     saves,
///     vec![
///         "LCSaveFile1",
///         "LCSaveFile2",
///         "LCSaveFile3"
///     ]
/// );
/// ```
pub fn get_current_save_names() -> Option<Vec<String>> {
    Some(
        get_save_dir_item_names()?
            .filter(|file| file.starts_with("LCSaveFile"))
            .collect(),
    )
}

/// Returns a vector of backup filenames without the path included.
///
/// More specifically, gets all the files that start with `BKP_LCSaveFile` inside `%appdata%\..\LocalLow\ZeekerssRBLX\Lethal Company\`,
/// sorted in descending order.
///
/// Returns `None` if the folder is nonexistent or reading it returned an `Err` somehow.
///
/// ```
/// let backups = get_backup_save_names().unwrap();
/// assert_eq!(
///     backups,
///     vec![
///         "BKP_LCSaveFile4_2024-03-16_18-21-28",
///         "BKP_LCSaveFile1_2024-03-16_18-09-00",
///         "BKP_LCSaveFile1_2024-03-16_01-04-19"
///     ]
/// );
/// ```
pub fn get_backup_save_names() -> Option<Vec<String>> {
    let mut items: Vec<String> = get_save_dir_item_names()?
        .filter(|file| file.starts_with("BKP_LCSaveFile"))
        .collect();
    items.reverse();
    Some(items)
}

/// Creates a save file backup in the saves folder.
///
/// Given a save file's name, creates `BKP_LCSaveName_Y-m-d_H-M-S` inside `%appdata%\..\LocalLow\ZeekerssRBLX\Lethal Company\`
///
/// Returns `io::Result` of the `fs::copy` operation.
pub fn save_backup_by_name(name: &String) -> std::io::Result<u64> {
    let save_pathbuf: PathBuf = {
        let mut pb = SAVE_DIR_PATHBUF.clone();
        pb.push(name);
        pb
    };
    let mut to_pathbuf = save_pathbuf.clone();
    to_pathbuf.set_file_name(
        String::from("BKP_")
            + name
            + &chrono::Local::now()
                .format("_%Y-%m-%d_%H-%M-%S")
                .to_string(),
    );

    println!("{:?}", to_pathbuf.as_path());

    fs::copy(save_pathbuf.as_path(), to_pathbuf.as_path())
}

/// Copies provided backup file into the corresponding LCSaveFile
///
/// Given a backup file's name `BKP_LCSaveName_Y-m-d_H-M-S`, copies it into `LCSaveName`.
///
/// Panics if `name` doesn't start with `BKP_`.
///
/// Returns `io::Result` of the `fs::copy` operation.
pub fn load_backup_by_name(name: &String) -> std::io::Result<u64> {
    assert!(name.starts_with("BKP_"), "Invalid backup filename!");

    let save_pathbuf: PathBuf = {
        let mut pb = SAVE_DIR_PATHBUF.clone();
        pb.push(name);
        pb
    };
    let mut to_pathbuf = save_pathbuf.clone();
    to_pathbuf.set_file_name(name.split('_').nth(1).unwrap());

    fs::copy(save_pathbuf.as_path(), to_pathbuf.as_path())
}
