/*---------------------------------------------------------------------------------------------
 *  Copyright 2024 SES
 *  Licensed under the Apache 2.0 License. See LICENSE.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

use crate::new_io_error;

pub fn to_path_checked(path: &str) -> std::io::Result<&std::path::Path> {
    if path.is_empty() {
        return Err(new_io_error!(
            std::io::ErrorKind::Other,
            "Path cannot be empty"
        ));
    }

    if path.contains("..") {
        return Err(new_io_error!(
            std::io::ErrorKind::Other,
            "Path cannot contain .."
        ));
    }

    let path = std::path::Path::new(path);
    if cfg!(windows) {
        if !path.starts_with("/") {
            return Err(new_io_error!(
                std::io::ErrorKind::Other,
                "Path must be absolute"
            ));
        }
    } else if !path.is_absolute() {
        return Err(new_io_error!(
            std::io::ErrorKind::Other,
            "Path must be absolute"
        ));
    }

    Ok(path)
}
