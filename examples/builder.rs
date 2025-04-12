use temp_dir_builder::TempDirectoryBuilder;

fn main() {
    let temp_directory = TempDirectoryBuilder::default()
        .add_text_file("test/foo.txt", "bar")
        .add_empty_file("test/folder-a/folder-b/bar.txt")
        .add_file("test_file.rs", file!())
        .create()
        .expect("create tree fs");

    println!(
        "created successfully in {}",
        temp_directory.path().display()
    );

    let path = temp_directory.path().to_path_buf();

    assert!(path.exists());

    drop(temp_directory);

    assert!(!path.exists());
}
