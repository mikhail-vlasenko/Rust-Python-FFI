import ctypes

# Load the shared library into ctypes
lib = ctypes.CDLL('./target/debug/logic_component.dll')

# Call the Rust function
result = lib.add(5, 3)
print(result)
