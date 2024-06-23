import ctypes
from ctypes import POINTER, Structure, c_int, c_size_t, c_void_p

# Define the ReturnType structure in Python
class ReturnType(Structure):
    _fields_ = [
        ("array", c_int * 100),
        ("tuple", c_int * 2)
    ]


lib = ctypes.CDLL('./target/debug/logic_component.dll')

# Define the return type of the return_array function
lib.return_array.restype = POINTER(ReturnType)

# Call the Rust function
result = lib.add(5, 3)
print(f"Result of add: {result}")

# Call the return_array function and dereference the pointer
return_type_ptr = lib.return_array()
return_type = return_type_ptr.contents

# Access the fields of the struct
print(f"Array: {list(return_type.array)}")
print(f"Tuple: ({return_type.tuple[0]}, {return_type.tuple[1]})")
