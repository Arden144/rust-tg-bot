use libc::{c_char, c_double};
use std::ffi::{c_void, CStr, CString};

#[link(name = "tdjson")]
extern "C" {
    fn td_json_client_create() -> *mut c_void;
    fn td_json_client_send(client: *mut c_void, request: *const c_char) -> c_void;
    fn td_json_client_receive(client: *mut c_void, timeout: c_double) -> *const c_char;
    fn td_json_client_execute(client: *mut c_void, request: *const c_char) -> *const c_char;
    fn td_json_client_destroy(client: *mut c_void) -> c_void;
}

pub struct Tdlib {
    client: *mut c_void,
}

impl Tdlib {
    pub fn new() -> Tdlib {
        Tdlib {
            client: unsafe { td_json_client_create() },
        }
    }
    pub fn send(&self, request: &str) {
        let c_request = CString::new(request).unwrap();
        unsafe { td_json_client_send(self.client, c_request.as_ptr()) };
    }
    pub fn receive(&self, timeout: f64) -> String {
        let msg = unsafe { td_json_client_receive(self.client, timeout) };
        unsafe { CStr::from_ptr(msg).to_string_lossy().into_owned() }
    }
    pub fn execute(&self, request: &str) -> String {
        let c_request = CString::new(request).unwrap();
        let msg = unsafe { td_json_client_execute(self.client, c_request.as_ptr()) };
        unsafe { CStr::from_ptr(msg).to_string_lossy().into_owned() }
    }
    pub fn destroy(&self) {
        unsafe { td_json_client_destroy(self.client) };
    }
}
