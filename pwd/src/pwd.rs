use std::ffi::CStr;
use std::ffi::CString;

/* 
 * get_encpwd takes a username and finds its associated encrypted password
 * from /etc/shadow
 */
pub fn get_encpwd(user: &str) -> Result<String, String>  {
    if user.is_empty() {
        return Err("cannot pass an empty username".to_string());
    }
    
    /* 
     * convert our paramater "user" into a C-style string ptr so we can pass it
     * to getspnam
     */
    let user_CString:CString = CString::new(user)
        .expect("CString::new failed");
    let user_charptr = user_CString.as_ptr();

    // pass this *const c_char to getspnam
    let pwd_struct = unsafe { libc::getspnam(user_charptr) };
    
    // if getspnam returns NULL, return error
    
    // access the struct returned to find the password and return it
    let pwdp_c_char = (*pwd_struct).sp_pwdp; // raw pointer dereferences are unsafe in Rust. Any way around this?
    let pwdp_Cstr = unsafe { CStr::from_ptr(pwdp_c_char) };
    let pwdp = pwdp_Cstr.to_string_lossy().to_string();
    
    return Ok(pwdp);
}
