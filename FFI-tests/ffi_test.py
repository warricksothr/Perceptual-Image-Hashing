#!/usr/bin/env python3

from ctypes import *
from _ffi_test_py import ffi, lib

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

print("starting ffi test")

#initialize the library
lib.init()

# Print the path and the bytes as hex for debugging
#print(large_image_path)
#print('\\x'+'\\x'.join('{:02x}'.format(x) for x in large_image_path))

for image in test_images:
    print("Get hashes for {}", image)
    print("AHash: {}",lib.ext_get_ahash(image) & 0xffffffffffffffff)
    print("DHash: {}",lib.ext_get_dhash(image) & 0xffffffffffffffff)
    print("PHash: {}",lib.ext_get_phash(image) & 0xffffffffffffffff)

# Do cleanup
#lib.teardown()

print("ffi test finished")
