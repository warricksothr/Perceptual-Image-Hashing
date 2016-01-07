#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <dlfcn.h>
#include <string.h>

/* function declaration */
void print_ustr_bytes(const char []);

int main() {
    void *lib;
    
    // declaration for the external functions used
    void (*init)();
    void (*teardown)();
    uint64_t (*get_ahash)(const char *);
    uint64_t (*get_dhash)(const char *);
    uint64_t (*get_phash)(const char *);

    //test image locations
    static const char largePathStr[] = u8"test_images/sample_01_large.jpg";
    static const char mediumPathStr[] = u8"test_images/sample_01_medium.jpg";
    static const char smallPathStr[] = u8"test_images/sample_01_small.jpg";
  
    static const char image1PathStr[] = u8"test_images/sample_01_";
    static const char image2PathStr[] = u8"test_images/sample_02_";
    static const char image3PathStr[] = u8"test_images/sample_03_";
    
    // Array of pointers to the base image paths to test
    //static const char *imagesSet[] = { image1PathStr, image2PathStr, image3PathStr };
    static const char *imagesSet[] = { image1PathStr, image2PathStr, image3PathStr };
    static const int imageSetSize = 3;

    // designators for the image sizes
    static const char largeImageSizeStr[] = u8"large";
    static const char mediumImageSizeStr[] = u8"medium";
    static const char smallImageSizeStr[] = u8"small";

    // Array of pointers to the images sizes
    static const char *imageSizesSet[] = { largeImageSizeStr, mediumImageSizeStr, smallImageSizeStr };
    static int imageSizesSetSize = 3;

    // Image extension
    static const char imageExtensionStr[] = u8".jpg";

    // Pointers that the external function call need
    const char *largePathPtr = &largePathStr[0];
    const char *mediumPathPtr = &mediumPathStr[0];
    const char *smallPathPtr = &smallPathStr[0];

    // Loading the external library
    lib = dlopen("./libpihash.so", RTLD_LAZY);

    //Registering the external functions
    *(void **)(&init) = dlsym(lib,"init");
    *(void **)(&teardown) = dlsym(lib,"teardown");
    *(void **)(&get_ahash) = dlsym(lib,"ext_get_ahash");
    *(void **)(&get_dhash) = dlsym(lib,"ext_get_dhash");
    *(void **)(&get_phash) = dlsym(lib,"ext_get_phash");

    // Init the shared library
    init();

    // loop over the images and sizes to test
    for (int i = 0; i < imageSetSize; i++) {
        for (int j = 0; j < imageSizesSetSize; j++) {
            char *imagePath = malloc(100);
            // Getting the correct path
            strcat(imagePath, imagesSet[i]);
            strcat(imagePath, imageSizesSet[j]);
            strcat(imagePath, imageExtensionStr);
            //printf("Path: %s\n", imagePath);
            
            // Visually proving that the bytes stored are the correct representation
            print_ustr_bytes(imagePath);
            
            // Printing information about the hashes of the provided images
            uint64_t imageAhash = get_ahash(imagePath);
            uint64_t imageDhash = get_dhash(imagePath);
            uint64_t imagePhash = get_phash(imagePath);
            
            printf("ahash: %llu \n", imageAhash);
            printf("dhash: %llu \n", imageDhash);
            printf("phash: %llu \n", imagePhash);
            
            //cleanup
            free(imagePath);
        }
    }

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
    free(strBuf);
}
