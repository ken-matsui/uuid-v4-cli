use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use uuid::Uuid;

#[no_mangle]
pub extern "C" fn generate(hyphenated: bool, urn: bool, uppercase: bool) -> *const c_char {
    let uuid = Uuid::new_v4();
    let output = CString::new(if hyphenated {
        let uuid = uuid.to_hyphenated();
        if uppercase {
            uuid.encode_upper(&mut Uuid::encode_buffer()).to_string()
        } else {
            uuid.to_string()
        }
    } else if urn {
        let uuid = uuid.to_urn();
        if uppercase {
            uuid.encode_upper(&mut Uuid::encode_buffer()).to_string()
        } else {
            uuid.to_string()
        }
    } else {
        let uuid = uuid.to_simple();
        if uppercase {
            uuid.encode_upper(&mut Uuid::encode_buffer()).to_string()
        } else {
            uuid.to_string()
        }
    })
    .unwrap();
    let ptr = output.as_ptr();
    mem::forget(output);
    ptr
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
                const char* output = generate(false, false, false);
                printf("%lu", strlen(output));
                free((char*)output);
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
                const char* output = generate(true, false, false);
                printf("%lu", strlen(output));
                free((char*)output);
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
                const char* output = generate(false, true, false);
                printf("%lu", strlen(output));
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout("45");
    }
}
