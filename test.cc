#include "rustlib.h"
#include <cassert>

int main(int argc, char ** argv)
{
    // Get a pointer to a rust type 
    // that C/C++ cannot ever understand
    auto *x = new_opaque();

    // Try to get an internal value stored
    // in a rust Option. It is initialized to None,
    // and the FFI bindings return NULL for that case.
    assert(get_value(x) == nullptr);

    // Set the internal Option to hold a value
    init(5, x);

    // Get a pointer to that stored value
    auto *y = get_value(x);
    // Yay, it is not null!
    assert(*y == 5);

    // Free it up.
    // Returns -1 if unsafe behavior 
    // like free(null) or double free is detected.
    auto rv = free_opaque(x);
    assert (rv == 0);

    // We can catch the double-free.
    rv = free_opaque(x);
    assert (rv < 0);
}
