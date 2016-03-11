#include <stdint.h>

void *ext_init(const char *);
void ext_free();
uint64_t ext_get_ahash(void *, const char *);
uint64_t ext_get_dhash(void *, const char *);
uint64_t ext_get_phash(void *, const char *);