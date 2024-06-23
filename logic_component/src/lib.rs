mod return_type;
use return_type::ReturnType;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
