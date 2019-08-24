#!/usr/bin/env python3

from ctypes import *
import os

large_image1_path = "../test_images/sample_01_large.jpg".encode(encoding="utf-8")
medium_image1_path = "../test_images/sample_01_medium.jpg".encode(encoding="utf-8")
small_image1_path = "../test_images/sample_01_small.jpg".encode(encoding="utf-8")

large_image2_path = "../test_images/sample_02_large.jpg".encode(encoding="utf-8")
medium_image2_path = "../test_images/sample_02_medium.jpg".encode(encoding="utf-8")
small_image2_path = "../test_images/sample_02_small.jpg".encode(encoding="utf-8")

large_image3_path = "../test_images/sample_03_large.jpg".encode(encoding="utf-8")
medium_image3_path = "../test_images/sample_03_medium.jpg".encode(encoding="utf-8")
small_image3_path = "../test_images/sample_03_small.jpg".encode(encoding="utf-8")

large_image4_path = "../test_images/sample_04_large.jpg".encode(encoding="utf-8")
medium_image4_path = "../test_images/sample_04_medium.jpg".encode(encoding="utf-8")
small_image4_path = "../test_images/sample_04_small.jpg".encode(encoding="utf-8")

test_images=[
	large_image1_path, medium_image1_path, small_image1_path,
	large_image2_path, medium_image2_path, small_image2_path,
	large_image3_path, medium_image3_path, small_image3_path,
	large_image4_path, medium_image4_path, small_image4_path
]

def unsigned64(number):
	return (number & 0xfffffffffffffffff)

print("starting ffi test")

# Load the shared library
lib = None
if os.name == 'nt':
	path = os.path.dirname(__file__) + "\\pihash.dll"
	print(path)
	lib = CDLL(path)
else:
	lib = cdll.LoadLibrary("libpihash.so")


class PIHashes(Structure):
	_fields_ = [
		("ahash", c_ulonglong),
		("dhash", c_ulonglong),
		("phash", c_ulonglong)]
	

# Setting the ctypes return type references for the foreign functions
# returns a pointer to the library that we'll need to pass to all function calls
lib.ext_init.restype = c_void_p
lib.ext_init.argtypes = [c_char_p]
# Returns a longlong hash, takes a pointer and a string
lib.ext_get_ahash.restype = c_ulonglong
lib.ext_get_ahash.argtypes = [c_void_p, c_char_p]
lib.ext_get_dhash.restype = c_ulonglong
lib.ext_get_dhash.argtypes = [c_void_p, c_char_p]
lib.ext_get_phash.restype = c_ulonglong
lib.ext_get_phash.argtypes = [c_void_p, c_char_p]
lib.ext_get_phashes.restype = c_void_p
lib.ext_get_phashes.argtypes = [c_void_p, c_char_p]
lib.ext_free_phashes.argtypes = [c_void_p]
# Takes a pointer and frees the struct at that memory location
lib.ext_free.argtypes = [c_void_p]

#initialize the library
lib_struct = lib.ext_init("./.hash_cache".encode('utf-8'))

#print("Pointer to lib_struct: ", lib_struct)

# Print the path and the bytes as hex for debugging
#print(large_image_path)
#print('\\x'+'\\x'.join('{:02x}'.format(x) for x in large_image_path))

for image in test_images:
	print("Requesting hashes for: %s"% image)
	phashes = lib.ext_get_phashes(lib_struct, image)
	pihashes = PIHashes.from_address(phashes)
	lib.ext_free_phashes(phashes)
	print("ahash: %i"% unsigned64(pihashes.ahash))
	print("dhash: %i"% unsigned64(pihashes.dhash))
	print("phash: %i"% unsigned64(pihashes.phash))
	# print("ahash: %i"% unsigned64(lib.ext_get_ahash(lib_struct, image)))
	# print("dhash: %i"% unsigned64(lib.ext_get_dhash(lib_struct, image)))
	# print("phash: %i"% unsigned64(lib.ext_get_phash(lib_struct, image)))

# Do cleanup
# Makes sure that the heap is cleaned up
lib.ext_free(lib_struct)

print("ffi test finished")
