 #include <stdbool.h>

bool test_and_set(volatile bool *p) {
    return __sync_lock_test_and_set(p, 1);
}

void tas_release(volatile bool *p) {
    return __sync_lock_release(p);
}
void spinlock_acquire(volatile bool *lock) { // <1>
    for (;;) {
        while(*lock); // <2>
        if (!test_and_set(lock))
            break;
    }
}

void spinlock_release(bool *lock) {
    tas_release(lock);
}