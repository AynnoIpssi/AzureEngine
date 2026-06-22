use std::io::Read;
use std::io::Write;
use std::os::fd::RawFd;
use std::os::unix::net::UnixStream;
use libc::{msghdr, size_t};
use std::os::unix::io::AsRawFd;



pub struct WaylandConnection {
    stream: UnixStream,
}

impl WaylandConnection {

    pub fn stream(&self) -> &UnixStream { &self.stream }
    pub fn new(stream: UnixStream) -> WaylandConnection {
        WaylandConnection { stream }
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), String> {
        self.stream.write_all(data)
            .map_err(|e| e.to_string())
    }

    pub fn send_with_fd(&mut self, data: &[u8], fd: RawFd) -> Result<(), String> {
        let iov = libc::iovec {
            iov_base: data.as_ptr() as *mut libc::c_void,
            iov_len: data.len(),
        };



        let cmsg_buffer_len = unsafe {
            libc::CMSG_SPACE(std::mem::size_of::<RawFd>() as u32) as usize
        };
        let mut cmsg_buffer = vec![0u8; cmsg_buffer_len];

        let msghdr = libc::msghdr {
            msg_name: std::ptr::null_mut(),
            msg_namelen: 0,
            msg_iov: &iov as *const libc::iovec as *mut libc::iovec,
            msg_iovlen: 1,
            msg_control:  cmsg_buffer.as_mut_ptr() as *mut libc::c_void,
            msg_flags: 0,
            msg_controllen: cmsg_buffer_len,
        };

        let cmsg_ptr = unsafe {
            libc::CMSG_FIRSTHDR(&msghdr)
        };

        if cmsg_ptr.is_null() {
            return Err("Failed to get cmsg header".to_string());
        }

        unsafe {
            (*cmsg_ptr).cmsg_level = libc::SOL_SOCKET;
            (*cmsg_ptr).cmsg_type = libc::SCM_RIGHTS;
        }

        unsafe {
            (*cmsg_ptr).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<RawFd>() as u32) as usize;
        }

        unsafe {
            let data_ptr = libc::CMSG_DATA(cmsg_ptr) as *mut RawFd;
            std::ptr::write(data_ptr, fd);
        }

        let socket_fd = self.stream.as_raw_fd();

        let result = unsafe {
            libc::sendmsg(socket_fd, &msghdr, 0)
        };

        if result == -1 {
            return Err("Failed to send message with fd".to_string());
        }

        Ok(())

    }

    pub fn receive(&mut self, buf: &mut [u8]) -> Result<(), String> {
        self.stream.read_exact(buf)
            .map_err(|e| e.to_string())
    }
}





