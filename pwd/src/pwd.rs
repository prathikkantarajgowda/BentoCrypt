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
 * pwd.rs
 */

use crate::util;

/*
 * highest level function
 *
 * username_to_kek takes a username and derives a kek from it using the
 * functions get_encpwd and derive_kek
 */
pub fn username_to_kek() -> Result<String, String> {

    //println!("\n======================================================================================");
    //println!("|| pwd version 0.1, Copyright (C) 2021 Prathik Gowda <gowdapra@grinnell.edu>        ||");
    //println!("|| pwd comes with ABSOLUTELY NO WARRANTY                                            ||");
    //println!("|| This is free software, and you are welcome to redistribute it                    ||");
    //println!("|| under certain conditions.                                                        ||");
    //println!("======================================================================================\n");

    //println!("generating kek...\n");

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
    let kek = util::derive_kek(encpwd);

    println!("{}", kek);

    return Ok(kek);
}
