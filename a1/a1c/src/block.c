
#include <stdbool.h>
#include <stdio.h>

typedef enum
{
    false,
    true
} bool;

struct Id
{
    int val;
    bool is_some;
} id;

typedef struct Id ID;

struct
{
    ID id;
    int size;
    int start_addr;
    int end_addr;
} Block;
