#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>

#define MEGBYTE 1 << 20

int main() {
  int file = open("/tmp/byte_write.txt", O_WRONLY | O_TRUNC);

  struct stat st;
  int blocks = -1;

  for (int i = 0; i < MEGBYTE; i++) {
    write(file, "a", 1);
    fstat(file, &st);
    if (st.st_blocks > blocks) {
      printf("Size: %lld, blocks: %lld, on disk %lld\n", st.st_size, st.st_blocks, st.st_blocks * 512);
      blocks = st.st_blocks;
    }
  }
}

