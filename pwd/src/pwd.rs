use std::ffi::CStr;
use std::ffi::CString;
use libc::c_char;
use libc::spwd;

/* 
 * get_encpwd takes a username and finds its associated encrypted password
 * from /etc/shadow
 */
pub fn get_encpwd(user: String) -> Result<String, String>  {
    let pwd_struct = getspnam_rust(user);
    let pwd_struct = match pwd_struct {
        Ok(pwd_struct) => pwd_struct,
        Err(error)     => panic!("Problem calling getspnam_rust: {:?}",
            error),
    };

    /* check if pwd_struct ptr is aligned? */
    
    /* 
     * get password from struct and return it
     *
     * we can dereference a raw ptr after checking to make sure it is non-NULL
     * and unaligned
     */
    let pwdp_c_char = unsafe { (*pwd_struct).sp_pwdp };
    let pwdp_cstr = unsafe { CStr::from_ptr(pwdp_c_char) };
    let pwdp = pwdp_cstr.to_string_lossy().to_string();
    
    return Ok(pwdp);
}

/*
 * getspnam_rust is a wrapper around libc's getspnam that allows the user
 * to pass a rust String instead of a *const c_char and avoids directly
 * dealing with FFI/libc
 *
 */
pub fn getspnam_rust(user: String) -> Result<*mut spwd, String> {

    /* username cannot be empty */
    if user.is_empty() {
        return Err("empty username passed to getspnam".to_string());
    }

    /* convert from String to *const c_char */
    let user_str  = CString::new(user)
        .expect("CString::new failed");
    let user_char: *const c_char = user_str.as_ptr() as *const c_char;

    /* 
     * now we actually perform the call to libc::getspnam with our
     * *const c_char
     */
    let pwd_struct = unsafe { libc::getspnam(user_char) };

    /* if getspnam returned NULL, it could not find the user on the system */
    if pwd_struct.is_null() {
        return Err("could not find user".to_string());
    }

    return Ok(pwd_struct);
}
