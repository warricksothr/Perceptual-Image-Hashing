#!/usr/bin/env python3

from ctypes import *

large_image1_path = "test_images/sample_01_large.jpg".encode(encoding="utf-8")
medium_image1_path = "test_images/sample_01_medium.jpg".encode(encoding="utf-8")
small_image1_path = "test_images/sample_01_small.jpg".encode(encoding="utf-8")

large_image2_path = "test_images/sample_02_large.jpg".encode(encoding="utf-8")
medium_image2_path = "test_images/sample_02_medium.jpg".encode(encoding="utf-8")
small_image2_path = "test_images/sample_02_small.jpg".encode(encoding="utf-8")

large_image3_path = "test_images/sample_03_large.jpg".encode(encoding="utf-8")
medium_image3_path = "test_images/sample_03_medium.jpg".encode(encoding="utf-8")
small_image3_path = "test_images/sample_03_small.jpg".encode(encoding="utf-8")

test_images=[large_image1_path, medium_image1_path, small_image1_path,large_image2_path, medium_image2_path, small_image2_path,large_image3_path, medium_image3_path, small_image3_path]

def unsigned64(number):
	return c_ulonglong(number).value

print("starting ffi test")

# Load the shared library
lib = cdll.LoadLibrary("libpihash.so")

# Setting the ctypes return type references for the foreign functions
lib.ext_get_ahash.restype = c_ulonglong
lib.ext_get_dhash.restype = c_ulonglong
lib.ext_get_phash.restype = c_ulonglong

#initialize the library
lib.init()

# Print the path and the bytes as hex for debugging
#print(large_image_path)
#print('\\x'+'\\x'.join('{:02x}'.format(x) for x in large_image_path))

for image in test_images:
    print("Requesting hashes for: %s"% image)
    print("ahash: %i"% unsigned64(lib.ext_get_ahash(image)))
    print("dhash: %i"% unsigned64(lib.ext_get_dhash(image)))
    print("phash: %i"% unsigned64(lib.ext_get_phash(image)))

# Do cleanup
#lib.teardown()

print("ffi test finished")
