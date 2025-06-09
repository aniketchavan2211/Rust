#include <string.h>
#include <stdio.h>
#include <openssl/sha.h>

void c_hash_string(const char* input, char* output_hex) {
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256((const unsigned char*)input, strlen(input), hash);

    for (int i = 0; i < SHA256_DIGEST_LENGTH; ++i) {
        sprintf(output_hex + (i * 2), "%02x", hash[i]);
    }
}
