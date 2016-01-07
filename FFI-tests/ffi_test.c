#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <dlfcn.h>

int main() {
    void *lib;
    uint64_t (*ext_get_ahash)(const char *);
    uint64_t (*ext_get_dhash)(const char *);
    uint64_t (*ext_get_phash)(const char *);

    static const char largePath[] = u8"test_images/sample_01_large.jpg";
    static const char mediumPath[] = u8"test_images/sample_01_medium.jpg";
    static const char smallPath[] = u8"test_images/sample_01_small.jpg";

    lib = dlopen("./libpihash.so", RTLD_LAZY);

    uint64_t largeA = ext_get_ahash(*largePath);
    uint64_t largeD = ext_get_dhash(*largePath);
    uint64_t largeP = ext_get_phash(*largePath);

    uint64_t mediumA = ext_get_ahash(*mediumPath);
    uint64_t mediumD = ext_get_dhash(*mediumPath);
    uint64_t mediumP = ext_get_phash(*mediumPath);

    uint64_t smallA = ext_get_ahash(*smallPath);
    uint64_t smallD = ext_get_dhash(*smallPath);
    uint64_t smallP = ext_get_phash(*smallPath);

    printf("Large_Test_AHash: ", largeA);
    printf("Large_Test_DHash: ", largeD);
    printf("Large_Test_PHash: ", largeP);
    printf("Medium_Test_AHash: ", mediumA);
    printf("Medium_Test_DHash: ", mediumD);
    printf("Medium_Test_PHash: ", mediumP);
    printf("Small_Test_AHash: ", smallA);
    printf("Small_Test_DHash: ", smallD);
    printf("Small_Test_PHash: ", smallP);

    if (lib != NULL ) dlclose(lib);
    return EXIT_SUCCESS;
}
