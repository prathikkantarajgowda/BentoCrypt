use libc::c_char;
use libc::spwd;
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt
};
use rand::rngs::OsRng;
use std::ffi::CStr;
use std::ffi::CString;

/*
 * highest level function - most can just call this and ignore the
 * sub-functions below
 *
 * username_to_kek takes a username and derives a kek from it using the
 * functions get_encpwd and derive_kek
 */
pub fn username_to_kek(user: String) -> Result<String, String> {

    /* get the encrypted password */
    let encpwd = get_encpwd(user);

    /* check for errors */
    let encpwd = match encpwd {
        Ok(pwd) => pwd,
        Err(error) => panic!("Problem calling get_encpwd: {:?}",
            error),
    };

    /* return the result of derive_kek on our encrypted password */
    return derive_kek(encpwd);
}

/* 
 * get_encpwd takes a username and finds its associated encrypted password
 * from /etc/shadow
 */
pub fn get_encpwd(user: String) -> Result<String, String>  {

    /* get the pwd_struct from getspnam_rust and check for errors */
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

/*
 * derive_kek takes a password and creates a KEK using the scrypt algorithm
 */
pub fn derive_kek(pwd: String) -> Result<String, String> {

    /* generate salt using OS random number generator */
    let salt = SaltString::generate(&mut OsRng);

    /* then derive our KEK using our encrypted password and salt as input */
    let kek = Scrypt.hash_password_simple(pwd.as_bytes(), salt.as_ref());

    return match kek {
        Ok(kek)    => Ok(kek.to_string()),
        Err(error) => Err(error.to_string()),
    };
}
