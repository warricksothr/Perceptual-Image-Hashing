#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <dlfcn.h>

int main() {
    uint64_t (*ext_get_ahash)(const char *);
    uint64_t (*ext_get_dhash)(const char *);
    uint64_t (*ext_get_phash)(const char *);
    
    imglib = dlopen("./libpihash.so", RTLD_LAZY);
    if ( imglib != NULL ) {
    
    }




    printf("Large_Test_AHash: ", largeA);
    printf("Large_Test_DHash: ", largeD);
    printf("Large_Test_PHash: ", largeP);
    printf("Medium_Test_AHash: ", mediumA);
    printf("Medium_Test_DHash: ", mediumD);
    printf("Medium_Test_PHash: ", mediumP);
    printf("Small_Test_AHash: ", smallA);
    printf("Small_Test_DHash: ", smallD);
    printf("Small_Test_PHash: ", smallP);
}
