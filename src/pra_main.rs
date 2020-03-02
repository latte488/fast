use libc;
use socket;
use std::str;

pub trait SockaddrIn {
    fn as_sockaddr_ptr(&self) -> *const libc::sockaddr;
    fn as_mut_sockaddr_ptr(&mut self) -> *mut libc::sockaddr;
}


impl SockaddrIn for libc::sockaddr_in {
    fn as_sockaddr_ptr(&self) -> *const libc::sockaddr {
        unsafe {
            std::mem::transmute::<*const libc::sockaddr_in, *const libc::sockaddr>(self)
        }
    }

    fn as_mut_sockaddr_ptr(&mut self) -> *mut libc::sockaddr {
        unsafe {
            std::mem::transmute::<*mut libc::sockaddr_in, *mut libc::sockaddr>(self)
        }
    }
}

fn main() {
    
    unsafe {
        
        let fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        println!("fd {}\n", fd);

        if fd == -1 {
            let s = std::ffi::CString::new("socket")
                .expect("CString::new faild")
                .as_ptr();
            libc::perror(s);
            std::process::exit(1);   
        }


        let addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port:   socket::htons(55488),
            sin_addr:   libc::in_addr { s_addr: libc::INADDR_ANY },
            sin_zero:   [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let addrlen = std::mem::size_of::<libc::sockaddr_in>() as u32;
        if libc::bind(fd, addr.as_sockaddr_ptr(), addrlen) == -1 {
            let s = std::ffi::CString::new("bind")
                .expect("CString::new fild")
                .as_ptr();
            libc::perror(s);
            std::process::exit(1);
        }

        let mut data = [0u8; 512];
        let mut addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port:   socket::htons(55488),
            sin_addr:   libc::in_addr { s_addr: libc::INADDR_ANY },
            sin_zero:   [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let mut addrlen: libc::socklen_t = 0;
        let len = libc::recvfrom(
            fd, 
            data.as_mut_ptr() as *mut libc::c_void, 
            data.len(), 
            0, 
            addr.as_mut_sockaddr_ptr(), 
            &mut addrlen);
        println!("fd {}\n", fd);

        if len == -1 {
            let s = std::ffi::CString::new("")
                .expect("CString::new faild")
                .as_ptr();
            libc::perror(s);
            std::process::exit(1);
        }

        let s = str::from_utf8(&data).expect("str::from_utf8 faild");
        println!("recv data {}", s);
        println!("recv szie {}", len);
        println!("addr ptr  {}", addr.sin_port);
    }
}
