use super::in_mem;
use super::{FileSystem as _, Mode, OFlags};
use crate::platform::mock::MockPlatform;
use alloc::vec;

extern crate std;

#[test]
fn test_inmem_root_file_creation_and_deletion() {
    let platform = MockPlatform::new();

    in_mem::FileSystem::new(&platform).with_root_privileges(|fs| {
        // Test file creation
        let path = "/testfile";
        let fd = fs
            .open(path, OFlags::CREAT | OFlags::WRONLY, Mode::RWXU)
            .expect("Failed to create file");

        fs.close(fd).expect("Failed to close file");

        // Test file deletion
        fs.unlink(path).expect("Failed to unlink file");
        assert!(
            fs.open(path, OFlags::RDONLY, Mode::RWXU).is_err(),
            "File should not exist"
        );
    });
}

#[test]
fn test_inmem_root_file_read_write() {
    let platform = MockPlatform::new();

    in_mem::FileSystem::new(&platform).with_root_privileges(|fs| {
        // Create and write to a file
        let path = "/testfile";
        let fd = fs
            .open(path, OFlags::CREAT | OFlags::WRONLY, Mode::RWXU)
            .expect("Failed to create file");
        let data = b"Hello, world!";
        fs.write(&fd, data).expect("Failed to write to file");
        fs.close(fd).expect("Failed to close file");

        // Read from the file
        let fd = fs
            .open(path, OFlags::RDONLY, Mode::RWXU)
            .expect("Failed to open file");
        let mut buffer = vec![0; data.len()];
        let bytes_read = fs.read(&fd, &mut buffer).expect("Failed to read from file");
        assert_eq!(bytes_read, data.len());
        assert_eq!(&buffer, data);
        fs.close(fd).expect("Failed to close file");
    });
}

#[test]
fn test_inmem_root_directory_creation_and_removal() {
    let platform = MockPlatform::new();

    in_mem::FileSystem::new(&platform).with_root_privileges(|fs| {
        // Test directory creation
        let path = "/testdir";
        fs.mkdir(path, Mode::RWXU)
            .expect("Failed to create directory");

        // Test directory removal
        fs.rmdir(path).expect("Failed to remove directory");
        assert!(
            fs.open(path, OFlags::RDONLY, Mode::RWXU).is_err(),
            "Directory should not exist"
        );
    });
}
