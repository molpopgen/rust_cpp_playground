pub struct OnlyRustUnderstandsMe {
    something: Option<i32>,
    integers: Vec<i32>,
}

#[repr(C)]
pub struct IntegerRange {
    beg: *const i32,
    end: *const i32,
}

impl OnlyRustUnderstandsMe {
    fn new() -> Self {
        Self {
            something: None,
            integers: vec![4; 3],
        }
    }
}

#[no_mangle]
pub extern "C" fn new_opaque() -> *mut OnlyRustUnderstandsMe {
    opaque_pointer::raw(OnlyRustUnderstandsMe::new())
}

#[no_mangle]
pub unsafe extern "C" fn free_opaque(value: *mut OnlyRustUnderstandsMe) -> i32 {
    match opaque_pointer::own_back(value) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn init(value: i32, s: *mut OnlyRustUnderstandsMe) {
    (*s).something = Some(value);
}

#[no_mangle]
pub unsafe extern "C" fn get_value(s: *const OnlyRustUnderstandsMe) -> *const i32 {
    match (*s).something {
        Some(x) => &x,
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_integer_range(s: *const OnlyRustUnderstandsMe) -> IntegerRange {
    let beg = (*s).integers.as_ptr();
    //let end = beg.offset((*s).integers.len() as isize);
    let end = beg.add((*s).integers.len());

    IntegerRange { beg, end }
}
