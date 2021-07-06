/* 
 * pwd: a program to derive a KEK from the login user's encrypted password
 * Copyright (C) 2021  Prathik Gowda (gowdapra@grinnell.edu)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 *
 * util.rs
 */

use hex;
use libc::c_char;
use libc::spwd;
use rand_core::OsRng;
use rand_core::RngCore;
use scrypt;
use std::ffi::CStr;
use std::ffi::CString;

pub const SCRYPT_N: u8    = 14; /* iterations  = LOG2(16348) = 14 */
pub const SCRYPT_R: u32   =  8; /* block size  = 8                */
pub const SCRYPT_P: u32   =  1; /* parallelism = 1                */

pub const SALTLEN:  usize = 32; /* salt length = 32 bytes         */
pub const KEKLEN:   usize = 32; /* kek length  = 32 bytes         */

/*
 * safe wrapper to use libc::getlogin() and return the login username as a
 * Rust String
 */
pub fn getlogin_rust() -> Result<String, String> {

    /* call getlogin and check for null */
    let username_char: *const c_char = unsafe {
        libc::getlogin()
    };

    if username_char.is_null() {
        return Err("libc::getlogin failed".to_string());
    }

    /* 
     * convert it to a string 
     *
     * WARNING: will fail if given faulty pointer
     */
    let username_cstr: &CStr = unsafe {
        CStr::from_ptr(username_char)
    };

    let username: String = username_cstr
        .to_string_lossy()
        .to_string();
    
    return Ok(username);
}

/* 
 * get_encpwd takes a username and finds its associated encrypted password
 * from /etc/shadow
 */
pub fn get_encpwd(user: String) -> Result<String, String>  {

    /* get the pwd_struct from getspnam_rust and check for errors */
    let pwd_struct = getspnam_rust(user);
    let pwd_struct: *mut spwd = match pwd_struct {
        Ok(pwd_struct) => pwd_struct,
        Err(error)     => return Err(error),
    };

    /* 
     * get password from struct and return it
     *
     * we can dereference a raw ptr after checking to make sure it is non-NULL
     * and unaligned
     */
    let pwdp_cstr: &CStr = unsafe {
	    CStr::from_ptr((*pwd_struct).sp_pwdp)
    };
    
    let pwdp: String = pwdp_cstr
        .to_string_lossy()
        .to_string();

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
    let user_str: CString        = CString::new(user)
        .expect("CString::new failed");
    let user_char: *const c_char = user_str.as_ptr() as *const c_char;

    /* 
     * now we actually perform the call to libc::getspnam with our
     * *const c_char
     */
    let pwd_struct = unsafe {
	    libc::getspnam(user_char)
    };

    /* if getspnam returned NULL, it could not find the user on the system */
    if pwd_struct.is_null() {
        return Err("could not find user".to_string());
    }

    return Ok(pwd_struct);
}

/*
 * derive_kek takes a password and creates a KEK using the scrypt algorithm
 *
 * KEK is 32-bytes, hex encoded (so 64 characters)
 */
pub fn derive_kek(pwd: String) -> String {

    /* set up scrypt parameters */
    let params: scrypt::Params  = scrypt::Params::new(SCRYPT_N, 
                                                      SCRYPT_R,
                                                      SCRYPT_P).unwrap();
    let password: &[u8]         = pwd.as_bytes();
    let mut salt: [u8; SALTLEN] = [0u8; SALTLEN];
    let mut kek:  [u8; KEKLEN]  = [0u8; KEKLEN];
    OsRng.fill_bytes(&mut salt);

    /* call scrypt key derivation function using our (default) parameters */
    scrypt::scrypt(password, &salt, &params, &mut kek)
        .unwrap();

    /* return String hex encoding of our kek */
    return hex::encode(kek);
}
