#!/usr/bin/env python3

from _ffi_test_py import ffi, lib

large_image_path = "test_images/sample_01_large.jpg".encode(encoding="utf-8")
medium_image_path = "test_images/sample_01_medium.jpg".encode(encoding="utf-8")
small_image_path = "test_images/sample_01_small.jpg".encode(encoding="utf-8")

print("starting test")

#initialize the library
lib.init()

# Print the path and the bytes as hex for debugging
#print(large_image_path)
#print('\\x'+'\\x'.join('{:02x}'.format(x) for x in large_image_path))

print("Get hashes for {}", large_image_path)
print("AHash: {}",lib.ext_get_ahash(large_image_path))
print("DHash: {}",lib.ext_get_dhash(large_image_path))
print("PHash: {}",lib.ext_get_phash(large_image_path))

print("Get hashes for {}", medium_image_path)
print("AHash: {}",lib.ext_get_ahash(medium_image_path))
print("DHash: {}",lib.ext_get_dhash(medium_image_path))
print("PHash: {}",lib.ext_get_phash(medium_image_path))

print("Get hashes for {}", small_image_path)
print("AHash: {}",lib.ext_get_ahash(small_image_path))
print("DHash: {}",lib.ext_get_dhash(small_image_path))
print("PHash: {}",lib.ext_get_phash(small_image_path))

# Do cleanup
#lib.teardown()
