#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <dlfcn.h>
#include <string.h>

/* function declaration */
void print_ustr_bytes(const char []);

int main() {
    void *lib;
    void *lib_struct;
    
    // declaration for the external functions used
    void *(*init)(const char *);
    void (*free)();
    uint64_t (*get_ahash)(void *, const char *);
    uint64_t (*get_dhash)(void *, const char *);
    uint64_t (*get_phash)(void *, const char *);

    //test image locations
    static const char image1PathStr[] = u8"../test_images/sample_01_";
    static const char image2PathStr[] = u8"../test_images/sample_02_";
    static const char image3PathStr[] = u8"../test_images/sample_03_";
    static const char image4PathStr[] = u8"../test_images/sample_04_";
    
    // Array of pointers to the base image paths to test
    //static const char *imagesSet[] = { image1PathStr, image2PathStr, image3PathStr };
    static const char *imagesSet[] = { image1PathStr, image2PathStr, image3PathStr, image4PathStr };
    static const int imageSetSize = 4;

    // designators for the image sizes
    static const char largeImageSizeStr[] = u8"large";
    static const char mediumImageSizeStr[] = u8"medium";
    static const char smallImageSizeStr[] = u8"small";

    // Array of pointers to the images sizes
    static const char *imageSizesSet[] = { largeImageSizeStr, mediumImageSizeStr, smallImageSizeStr };
    static int imageSizesSetSize = 3;

    // Image extension
    static const char imageExtensionStr[] = u8".jpg";

    // Loading the external library
    lib = dlopen("./libpihash.so", RTLD_LAZY);

    //Registering the external functions
    *(void **)(&init) = dlsym(lib,"ext_init");
    *(void **)(&free) = dlsym(lib,"ext_free");
    *(void **)(&get_ahash) = dlsym(lib,"ext_get_ahash");
    *(void **)(&get_dhash) = dlsym(lib,"ext_get_dhash");
    *(void **)(&get_phash) = dlsym(lib,"ext_get_phash");

    // Init the shared library
    lib_struct = init(u8"./.hash_cache");

    //temp buffer for the path
    char *imagePathBuffer = malloc(100);
    // loop over the images and sizes to test
    for (int i = 0; i < imageSetSize; i++) {
        for (int j = 0; j < imageSizesSetSize; j++) {
            // Make sure the buffer is clean before using it
            memset(imagePathBuffer,0,100);
            // Getting the correct path
            strcat(imagePathBuffer, imagesSet[i]);
            strcat(imagePathBuffer, imageSizesSet[j]);
            strcat(imagePathBuffer, imageExtensionStr);
            //printf("Path: %s\n", imagePath);
            
            // Visually proving that the bytes stored are the correct representation
            //print_ustr_bytes(imagePath);
            printf("Image: %s\n",imagePathBuffer);

            // Printing information about the hashes of the provided images
            uint64_t imageAhash = get_ahash(lib_struct, imagePathBuffer);
            uint64_t imageDhash = get_dhash(lib_struct, imagePathBuffer);
            uint64_t imagePhash = get_phash(lib_struct, imagePathBuffer);
            
            printf("ahash: %llu \n", imageAhash);
            printf("dhash: %llu \n", imageDhash);
            printf("phash: %llu \n", imagePhash);
        }
    } 
    //cleanup and close the buffer
    memset(imagePathBuffer,0,100);
    free(imagePathBuffer);

    // Closing the shared library reference
    if (lib != NULL ) dlclose(lib);
    return EXIT_SUCCESS;
}

void print_ustr_bytes(const char str[]) {
    int strLen = strlen(str);
    //printf("Length: %u \n",strLen*2);
    char *strBuf = malloc(strLen*4);
    for(int i = 0; i <= strLen; i++) {
        sprintf(&strBuf[i*4], "\\x%02X", str[i]);
    }
    printf("String: '%s' -> Bytes: '%s'\n" , str, strBuf);
    memset(strBuf,0,strLen*4);
    free(strBuf);
}
