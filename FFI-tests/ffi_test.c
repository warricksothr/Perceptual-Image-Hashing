#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <dlfcn.h>
#include <string.h>

/* function declaration */
void print_str_bytes(const char []);

int main() {
    void *lib;
    uint64_t (*get_ahash)(const char *);
    uint64_t (*get_dhash)(const char *);
    uint64_t (*get_phash)(const char *);

    static const char largePathStr[] = { 0x74,0x65,0x73,0x74,0x5F,0x69,0x6D,0x61,0x67,0x65,0x73,0x2F,0x73,0x61,0x6D,0x70,0x6C,0x65,0x5F,0x30,0x31,0x5F,0x6C,0x61,0x72,0x67,0x65,0x2E,0x6A,0x70,0x67,0x00 };
    //print_str_bytes(largePathStr);
    static const char mediumPathStr[] = { 0x74,0x65,0x73,0x74,0x5F,0x69,0x6D,0x61,0x67,0x65,0x73,0x2F,0x73,0x61,0x6D,0x70,0x6C,0x65,0x5F,0x30,0x31,0x5F,0x6D,0x65,0x64,0x69,0x75,0x6D,0x2E,0x6A,0x70,0x67,0x00 };
    //print_str_bytes(mediumPathStr);
    static const char smallPathStr[] = { 0x74,0x65,0x73,0x74,0x5F,0x69,0x6D,0x61,0x67,0x65,0x73,0x2F,0x73,0x61,0x6D,0x70,0x6C,0x65,0x5F,0x30,0x31,0x5F,0x73,0x6D,0x61,0x6C,0x6C,0x2E,0x6A,0x70,0x67,0x00 };
    //print_str_bytes(smallPathStr);

    const char *largePathPtr = &largePathStr[0];
    const char *mediumPathPtr = &mediumPathStr[0];
    const char *smallPathPtr = &smallPathStr[0];

    // Loading the external library
    lib = dlopen("./libpihash.so", RTLD_LAZY);

    //Registering the external functions
    *(void **)(&get_ahash) = dlsym(lib,"ext_get_ahash");
    *(void **)(&get_dhash) = dlsym(lib,"ext_get_dhash");
    *(void **)(&get_phash) = dlsym(lib,"ext_get_phash");


    uint64_t largeA = get_ahash(largePathPtr);
    uint64_t largeD = get_dhash(largePathPtr);
    uint64_t largeP = get_phash(largePathPtr);

    uint64_t mediumA = get_ahash(mediumPathPtr);
    uint64_t mediumD = get_dhash(mediumPathPtr);
    uint64_t mediumP = get_phash(mediumPathPtr);

    uint64_t smallA = get_ahash(smallPathPtr);
    uint64_t smallD = get_dhash(smallPathPtr);
    uint64_t smallP = get_phash(smallPathPtr);

    printf("Large_Test_AHash: %llu \n", largeA);
    printf("Large_Test_DHash: %llu \n", largeD);
    printf("Large_Test_PHash: %llu \n", largeP);
    printf("Medium_Test_AHash: %llu \n", mediumA);
    printf("Medium_Test_DHash: %llu \n", mediumD);
    printf("Medium_Test_PHash: %llu \n", mediumP);
    printf("Small_Test_AHash: %llu \n", smallA);
    printf("Small_Test_DHash: %llu \n", smallD);
    printf("Small_Test_PHash: %llu \n", smallP);

    if (lib != NULL ) dlclose(lib);
    return EXIT_SUCCESS;
}

void print_str_bytes(const char str[]) {
    int strLen = strlen(str);
    printf("Length: %u \n",strLen*2);
    char* strBuf =  (char*) malloc(strLen);
    for(int i = 0; i <= strLen; i++) {
        int j = i * 2;
        printf("%c",str[i]);
        sprintf(&strBuf[j], "%02X", str[i]);
    }
    printf("\nBytes: %s \n" , strBuf);
}
