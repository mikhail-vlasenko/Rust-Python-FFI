mod return_type;

use std::collections::HashMap;
use return_type::ReturnType;
use std::sync::Mutex;


#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn return_array() -> *const ReturnType {
    let array = [1; 100];
    let tuple = (1, 2);
    let return_type = ReturnType {
        array,
        tuple,
    };
    Box::into_raw(Box::new(return_type))
}


lazy_static::lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[derive(Debug)]
struct State {
    counter: i32,
    some_keys: HashMap<String, i32>
}

impl State {
    fn new() -> Self {
        State {
            counter: 0,
            some_keys: HashMap::new()
        }
    }
}

#[no_mangle]
pub extern "C" fn increment_counter() -> i32 {
    let mut state = STATE.lock().unwrap();
    state.counter += 1;
    state.counter
}

#[no_mangle]
pub extern "C" fn get_counter() -> i32 {
    let state = STATE.lock().unwrap();
    state.counter
}

#[no_mangle]
pub extern "C" fn set_key(key: *const u8, key_len: usize, value: i32) {
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let key = String::from_utf8_lossy(key).to_string();
    let mut state = STATE.lock().unwrap();
    state.some_keys.insert(key, value);
}

#[no_mangle]
pub extern "C" fn get_value(key: *const u8, key_len: usize) -> i32 {
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let key = String::from_utf8_lossy(key).to_string();
    let state = STATE.lock().unwrap();
    match state.some_keys.get(&key) {
        Some(value) => *value,
        None => 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
