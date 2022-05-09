use std::ffi::CString;
use std::os::raw::c_char;
use uuid::Uuid;

/// The returned char pointer should be freed by the `cstring_free` function.
/// For detailed usage in any language, please see:
/// https://jakegoulding.com/rust-ffi-omnibus/string_return/
#[no_mangle]
pub extern "C" fn generate(hyphenated: bool, urn: bool, uppercase: bool) -> *mut c_char {
    let uuid = Uuid::new_v4();
    let output = CString::new(if hyphenated {
        let uuid = uuid.as_hyphenated();
        if uppercase {
            uuid.encode_upper(&mut Uuid::encode_buffer()).to_string()
        } else {
            uuid.to_string()
        }
    } else if urn {
        let uuid = uuid.as_urn();
        if uppercase {
            uuid.encode_upper(&mut Uuid::encode_buffer()).to_string()
        } else {
            uuid.to_string()
        }
    } else {
        let uuid = uuid.as_simple();
        if uppercase {
            uuid.encode_upper(&mut Uuid::encode_buffer()).to_string()
        } else {
            uuid.to_string()
        }
    })
    .unwrap();
    output.into_raw()
}

/// # Safety
///
/// This function is for freeing a pointer of the argument.
/// The pointer should be allocated with `CString` in the Rust world.
#[no_mangle]
pub unsafe extern "C" fn cstring_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }

    // retake pointer to free memory
    let _ = CString::from_raw(s);
}

#[cfg(test)]
mod tests {
    use inline_c::assert_c;

    #[test]
    fn test_c_api_simple_uuid() {
        (assert_c! {
            #include "uuid-v4-cli.h"
            #include <stdio.h>
            #include <string.h>

            int main() {
                char* output = generate(false, false, false);
                printf("%lu", strlen(output));
                cstring_free(output);
                return 0;
            }
        })
        .success()
        .stdout("32");
    }
    #[test]
    fn test_c_api_hyphenated_uuid() {
        (assert_c! {
            #include "uuid-v4-cli.h"
            #include <stdio.h>
            #include <string.h>

            int main() {
                char* output = generate(true, false, false);
                printf("%lu", strlen(output));
                cstring_free(output);
                return 0;
            }
        })
        .success()
        .stdout("36");
    }
    #[test]
    fn test_c_api_urn_uuid() {
        (assert_c! {
            #include "uuid-v4-cli.h"
            #include <stdio.h>
            #include <string.h>

            int main() {
                char* output = generate(false, true, false);
                printf("%lu", strlen(output));
                cstring_free(output);
                return 0;
            }
        })
        .success()
        .stdout("45");
    }
}
