use crate::util;

/*
 * highest level function
 *
 * username_to_kek takes a username and derives a kek from it using the
 * functions get_encpwd and derive_kek
 */
pub fn username_to_kek() -> Result<String, String> {

    /* get the login username and check for errors */
    let user = util::getlogin_rust();
    let user = match user {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    /* get the encrypted password and check for errors */
    let encpwd = util::get_encpwd(user);
    let encpwd = match encpwd {
        Ok(pwd) => pwd,
        Err(error) => return Err(error),
    };

    /* return the result of derive_kek on our encrypted password */
    return util::derive_kek_argon(encpwd);
}
