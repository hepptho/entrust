use std::fs::File;
use std::{fs, io};
use tempfile::{tempdir, TempDir};

fn setup_test_store() -> io::Result<TempDir> {
    let temp_dir = tempdir()?;
    File::create(temp_dir.path().join("file1"))?;
    File::create(temp_dir.path().join("file2"))?;
    let dir1 = temp_dir.path().join("dir1");
    fs::create_dir_all(&dir1)?;
    File::create(dir1.join("file1"))?;
    File::create(dir1.join("file2"))?;
    let dir2 = temp_dir.path().join("dir2");
    fs::create_dir_all(&dir2)?;
    File::create(dir2.join("pass"))?;
    println!("test_store: {}", temp_dir.path().display());
    Ok(temp_dir)
}

#[test]
fn test_get_all() -> anyhow::Result<()> {
    let test_store = setup_test_store()?;
    let existing = par_core::get_existing_locations(test_store.path())?;
    assert_eq!(
        vec!["dir1/file1", "dir1/file2", "dir2/pass", "file1", "file2"],
        existing.files
    );
    assert_eq!(vec!["dir1/", "dir2/"], existing.dirs);
    Ok(())
}

#[test]
fn test_resolve_existing() -> anyhow::Result<()> {
    let test_store = setup_test_store()?;
    let file1 = par_core::resolve_existing_location(test_store.path(), "file1", false)?;
    assert_eq!(test_store.path().join("file1"), file1);

    let dir1 = par_core::resolve_existing_location(test_store.path(), "dir1", true)?;
    assert_eq!(test_store.path().join("dir1"), dir1);
    assert!(par_core::resolve_existing_location(test_store.path(), "dir1", false).is_err());

    let dir1_file1 = par_core::resolve_existing_location(test_store.path(), "dir1/file1", false)?;
    assert_eq!(test_store.path().join("dir1/file1"), dir1_file1);

    let dir2 = par_core::resolve_existing_location(test_store.path(), "dir2", true)?;
    assert_eq!(test_store.path().join("dir2"), dir2);
    let dir2_pass = par_core::resolve_existing_location(test_store.path(), "dir2", false)?;
    assert_eq!(test_store.path().join("dir2/pass"), dir2_pass);

    assert!(par_core::resolve_existing_location(test_store.path(), "no such file", true).is_err());

    Ok(())
}

#[test]
fn test_resolve_new() -> anyhow::Result<()> {
    let test_store = setup_test_store()?;

    let new = par_core::resolve_new_location(test_store.path(), "new")?;
    assert_eq!(test_store.path().join("new"), new);

    let new_in_new_dir = par_core::resolve_new_location(test_store.path(), "new_dir/new")?;
    assert_eq!(test_store.path().join("new_dir/new"), new_in_new_dir);

    assert!(par_core::resolve_new_location(test_store.path(), "file1").is_err());

    Ok(())
}
