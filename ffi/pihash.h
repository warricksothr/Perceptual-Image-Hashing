#include <stdint.h>

void init();
void teardown();
uint64_t ext_get_ahash(const char *);
uint64_t ext_get_dhash(const char *);
uint64_t ext_get_phash(const char *);