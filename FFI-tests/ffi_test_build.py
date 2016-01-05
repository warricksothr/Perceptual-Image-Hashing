#!/usr/bin/env python3

# A simple test script to confirm the C FFI for the rust library is working

from cffi import FFI
ffi = FFI()

ffi.set_source("_ffi_test_py",
        """
        #include <dlfcn.h>
        """,
        libraries=["pihash"],
        library_dirs=["."]
        )

ffi.cdef("""
    
    void init();
    void teardown();
    uint64_t ext_get_ahash(const char *);
    uint64_t ext_get_dhash(const char *);
    uint64_t ext_get_phash(const char *);
""")

if __name__ == "__main__":
    ffi.compile()
