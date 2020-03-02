
pub trait SockaddrIn {
    fn new(port: u16) -> libc::sockaddr_in;
    fn any() -> libc::sockaddr_in;
    fn as_sockaddr_ptr(&self) -> *const libc::sockaddr;
    fn as_mut_sockaddr_ptr(&mut self) -> *mut libc::sockaddr;
}

impl SockaddrIn for libc::sockaddr_in {
    

    fn new(port: u16) -> libc::sockaddr_in {
        let address = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port:   socket::htons(port),
            sin_addr:   libc::in_addr { s_addr: libc::INADDR_ANY },
            sin_zero:   [0; 8],
        };

        address
    }

    fn any() -> libc::sockaddr_in {
        libc::sockaddr_in::new(0u16)
    }

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

const MTU: usize = 512;

pub struct Message {
    pub address: libc::sockaddr_in,
    pub data: [u8; MTU],
    pub size: usize,
}

impl Message {
    pub fn new() -> Message {
        Message {
            address : libc::sockaddr_in::any(),
            data : [0u8; 512],
            size : 0usize
        }
    }
}

pub struct Udp {
    pub fd: libc::c_int
}

impl Udp {

    fn error() {
        let s = std::ffi::CString::new("")
            .expect("CString::new faild")
            .as_ptr();
        unsafe { libc::perror(s); }
        std::process::exit(1);
    }

    pub fn new() -> Udp {
        
        let fd;
        
        unsafe { 
            fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0); 
        }
        
        if fd == -1 {
            Udp::error()
        }

        Udp { fd }
    }

    pub fn bind(&mut self, port: u16) {
        
        let addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port:   socket::htons(port),
            sin_addr:   libc::in_addr { s_addr: libc::INADDR_ANY },
            sin_zero:   [0; 8],
        };
        
        let addrlen = std::mem::size_of::<libc::sockaddr_in>() as u32;
        
        let result;
        
        unsafe { 
            result = libc::bind(self.fd, addr.as_sockaddr_ptr(), addrlen); 
        }
        
        if result == -1 {
            Udp::error()
        }
    }

    pub fn recvfrom(&mut self, message: &mut Message) -> usize  {
        
        let mut addrlen: libc::socklen_t = 0;

        let len;
        
        unsafe {
            len = libc::recvfrom(
                self.fd,
                message.data.as_mut_ptr() as *mut libc::c_void,
                message.data.len(),
                0,
                message.address.as_mut_sockaddr_ptr(),
                &mut addrlen
            );
        }

        if len <= 0 {
            Udp::error();
        }
        
        len as usize
    }
}



impl Drop for Udp {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd); }
    }
}

