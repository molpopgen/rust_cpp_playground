pub struct OnlyRustUnderstandsMe {
    something: Option<i32>,
}

impl OnlyRustUnderstandsMe {
    fn new() -> Self {
        Self { something: None }
    }
}

#[no_mangle]
pub extern "C" fn new_opaque() -> *mut OnlyRustUnderstandsMe {
    opaque_pointer::raw(OnlyRustUnderstandsMe::new())
}

#[no_mangle]
pub extern "C" fn free_opaque(value: *mut OnlyRustUnderstandsMe) -> i32 {
    unsafe {
        match opaque_pointer::own_back(value) {
            Ok(_) => (),
            Err(_) => return -1,
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn init(value: i32, s: *mut OnlyRustUnderstandsMe) {
    unsafe {
        (*s).something = Some(value);
    }
}

#[no_mangle]
pub extern "C" fn get_value(s: *const OnlyRustUnderstandsMe) -> *const i32 {
    unsafe {
        match (*s).something {
            Some(x) => &x,
            None => std::ptr::null(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
