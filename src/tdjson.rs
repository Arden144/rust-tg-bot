use std::os::raw::{c_char, c_int, c_double, c_longlong, c_void};
use std::ffi::{CStr, CString};

#[link(name = "tdjson")]
extern "C" {
    fn td_json_client_create() -> *mut c_void;
    fn td_json_client_send(client: *mut c_void, request: *const c_char) -> c_void;
    fn td_json_client_receive(client: *mut c_void, timeout: c_double) -> *const c_char;
    fn td_json_client_execute(client: *mut c_void, request: *const c_char) -> *const c_char;
    fn td_json_client_destroy(client: *mut c_void) -> c_void;
    fn td_set_log_file_path(file_path: *const c_char) -> c_int;
    fn td_set_log_max_file_size(max_file_size: c_longlong) -> c_void;
    fn td_set_log_verbosity_level(new_verbosity_level: c_int) -> c_void;
}

pub struct NoResponseError;

impl std::fmt::Display for NoResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No response received from tdjson")
    }
}

pub struct Tdlib {
    client: *mut c_void,
}

impl Drop for Tdlib {
    fn drop(&mut self) {
        unsafe { td_json_client_destroy(self.client) };
    }
}

unsafe impl Send for Tdlib {}
unsafe impl Sync for Tdlib {}

impl Tdlib {
    pub fn new() -> Self {
        let client_pointer = unsafe { td_json_client_create() };
        Self {
            client: unsafe { &mut*client_pointer }
        }
    }
    pub fn send(&self, request: &str) {
        let c_request = CString::new(request).unwrap();
        unsafe { td_json_client_send(self.client, c_request.as_ptr()) };
    }
    pub fn receive(&self, timeout: f64) -> Result<String, NoResponseError> {
        let msg = unsafe { td_json_client_receive(self.client, timeout) };
        if !msg.is_null() {
            Ok(unsafe { CStr::from_ptr(msg).to_string_lossy().into_owned() })
        } else {
            Err(NoResponseError)
        }
    }
    pub fn execute(&self, request: &str) -> String {
        let c_request = CString::new(request).unwrap();
        let msg = unsafe { td_json_client_execute(self.client, c_request.as_ptr()) };
        unsafe { CStr::from_ptr(msg).to_string_lossy().into_owned() }
    }
    pub fn log_path(path: &str) -> i32 {
        let c_path = CString::new(path).unwrap();
        unsafe { td_set_log_file_path(c_path.as_ptr()) }
    }
    pub fn max_log_file_size(size: i64) {
        unsafe { td_set_log_max_file_size(size) };
    }
    pub fn log_verbosity_level(level: i32) {
        unsafe { td_set_log_verbosity_level(level) };
    }
}
