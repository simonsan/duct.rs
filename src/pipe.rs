extern crate libc;

use std::mem;
use std::os::unix::io::{FromRawFd, IntoRawFd, AsRawFd, RawFd};
use std::fs::File;
use std::process::Stdio;

#[derive(Debug)]
pub struct Handle {
    // The struct *owns* this file descriptor, and will close it in drop().
    fd: RawFd,
}

impl Handle {
    // TODO: Is giving unlocked access to the standard descriptors unsafe?
    pub fn stdin() -> Handle {
        dup_or_panic(libc::STDIN_FILENO)
    }

    pub fn stdout() -> Handle {
        dup_or_panic(libc::STDOUT_FILENO)
    }

    pub fn stderr() -> Handle {
        dup_or_panic(libc::STDERR_FILENO)
    }

    pub fn from_file(file: File) -> Handle {
        unsafe { Handle::from_raw_fd(file.into_raw_fd()) }
    }

    pub fn dup_file(file: &File) -> Handle {
        dup_or_panic(file.as_raw_fd())
    }

    pub fn into_file(self) -> File {
        unsafe { File::from_raw_fd(self.into_raw_fd()) }
    }

    pub fn into_stdio(self) -> Stdio {
        unsafe { Stdio::from_raw_fd(self.into_raw_fd()) }
    }
}

// TODO: Instead of making cloning so explicit, pass Handle around by reference and give it an
// &self make_stdio() method. Under the hood that will call dup(), but maybe someday when we have a
// more flexible Command implementation it won't need to.
impl Clone for Handle {
    fn clone(&self) -> Self {
        dup_or_panic(self.fd)
    }
}

impl FromRawFd for Handle {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Handle { fd: fd }
    }
}

impl IntoRawFd for Handle {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        mem::forget(self);  // prevent drop() from closing the fd
        fd
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let error = unsafe { libc::close(self.fd) };
        assert_eq!(error, 0);
    }
}

// (read, write)
// TODO: error handling
pub fn open_pipe() -> (Handle, Handle) {
    unsafe {
        let mut pipes = [0, 0];
        let error = libc::pipe(pipes.as_mut_ptr());
        assert_eq!(error, 0);
        make_uninheritable(pipes[0]);
        make_uninheritable(pipes[1]);
        (Handle::from_raw_fd(pipes[0]), Handle::from_raw_fd(pipes[1]))
    }
}

fn dup_or_panic(fd: RawFd) -> Handle {
    unsafe {
        let new_fd = libc::dup(fd);
        assert!(new_fd >= 0, "dup() returned an error");
        make_uninheritable(new_fd);
        FromRawFd::from_raw_fd(new_fd)
    }
}

unsafe fn make_uninheritable(fd: RawFd) {
    let ret = libc::ioctl(fd, libc::FIOCLEX);
    assert_eq!(ret, 0);
}

#[cfg(test)]
mod test {
    use super::open_pipe;
    use std::io::prelude::*;

    #[test]
    fn test_pipes() {
        let (r, w) = open_pipe();
        let mut r_file = r.clone().into_file();
        let mut w_file = w.clone().into_file();
        drop(w);
        w_file.write_all(b"some stuff").unwrap();
        drop(w_file);
        let mut output = Vec::new();
        r_file.read_to_end(&mut output).unwrap();
    }
}