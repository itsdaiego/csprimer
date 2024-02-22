#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define STARTING_CAPACITY 8

typedef struct DA {
  int length; 
  int capacity;
  int* items;
} DA;


DA* DA_new (void) {
  DA* da = malloc(sizeof(DA));

  da->length = 0;
  da->capacity = STARTING_CAPACITY;
  da->items = malloc(sizeof(int) * STARTING_CAPACITY);

  return da;
}

int DA_size(DA *da) {
  return da->length;
}

void DA_push (DA* da, int* x) {
  if (da->length == da->capacity) {
    void *temp_items = realloc(da->items, da->capacity * 2 * sizeof(int));

    if (temp_items == NULL) {
      perror("somehting went south, oh no");

      free(da->items);
      free(da);

      return;
    }

    da->capacity *= 2;

    da->items = temp_items;
  }

  da->items[da->length] = *x;
  
  da->length++;
}

int DA_pop(DA *da) {
  if (da->length == 0) {
    return -1;
  }

  da->length--;

  return da->items[da->length];
}

void DA_set(DA *da, int *x, int i) {
  if (i < 0 || i >= da->length) {
    perror("index out of bounds");
  }

  da->items[i] = *x;
}

int DA_get(DA *da, int i) {
  if (i < 0 || i > da->length) {
    perror("index out of bounds");

    return -1;
  }

  return  da->items[i];
}


void DA_free(DA *da) {
  free(da->items);
}

int main() {
    DA* da = DA_new();

    assert(DA_size(da) == 0);

    // basic push and pop test
    int x = 5;
    // TOOD: float y = 12.4;
    int y = 12;
    DA_push(da, &x);
    DA_push(da, &y);
    assert(DA_size(da) == 2);

    assert(DA_pop(da) == y);
    assert(DA_size(da) == 1);

    assert(DA_pop(da) == x);
    assert(DA_size(da) == 0);
    assert(DA_pop(da) == -1);

    // basic set/get test
    DA_push(da, &x);
    DA_set(da, &y, 0);
    assert(DA_get(da, 0) == y);
    DA_pop(da);
    assert(DA_size(da) == 0);

    // expansion test
    DA *da2 = DA_new(); // use another DA to show it doesn't get overriden
    DA_push(da2, &x);
    int i, n = 100 * STARTING_CAPACITY, arr[n];
    for (i = 0; i < n; i++) {
      arr[i] = i;
      DA_push(da, &arr[i]);
    }
    assert(DA_size(da) == n);
    for (i = 0; i < n; i++) {
      assert(DA_get(da, i) == arr[i]);
    }
    for (; n; n--)
      DA_pop(da);
    assert(DA_size(da) == 0);
    assert(DA_pop(da2) == x); // this will fail if da doesn't expand

    DA_free(da);
    DA_free(da2);
    printf("OK\n");
}
