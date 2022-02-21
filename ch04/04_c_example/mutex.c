#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

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

// 再入可能ミューテックス用の型 <1>
struct reent_lock {
    bool lock; // ロック用共有変数
    int id;    // 現在ロックを獲得中のスレッドID、非ゼロの場合ロック獲得中
    int cnt;   // 再帰ロックのカウント
};

// 再帰ロック獲得関数
void reentlock_acquire(struct reent_lock *lock, int id) {
    // ロック獲得中でかつ自分が獲得中か判定 <2>
    if (lock->lock && __atomic_load_n(&lock->id, __ATOMIC_RELAXED) == id) {
        // 自分が獲得中ならカウントをインクリメント
        lock->cnt++;
    } else {
        // 誰もロックを獲得していないか、
        // 他のスレッドがロック獲得中ならロック獲得
        spinlock_acquire(&lock->lock);
        // ロックを獲得したら、自身のスレッドIDを設定し、
        // カウントをインクリメント
        __atomic_store_n(&lock->id, id, __ATOMIC_RELAXED);
        lock->cnt++;
    }
}

// 再帰ロック解放関数
void reentlock_release(struct reent_lock *lock) {
    // カウントをデクリメントし、
    // そのカウントが0になったらロック解放 <3>
    lock->cnt--;
    if (lock->cnt == 0) {
        __atomic_store_n(&lock->id, 0, __ATOMIC_RELAXED);
        spinlock_release(&lock->lock);
    }
}

struct reent_lock lock_var; // ロック用の共有変数

// n回再帰的に呼び出してロックするテスト関数
void reent_lock_test(int id, int n) {
    if (n == 0)
        return;

    // 再帰ロック
    reentlock_acquire(&lock_var, id);
    reent_lock_test(id, n - 1);
    reentlock_release(&lock_var);
}

// スレッド用関数
void *thread_func(void *arg) {
    int id = (int)arg;
    assert(id != 0);
    for (int i = 0; i < 10000; i++) {
        reent_lock_test(id, 10);
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS];
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&v[i], NULL, thread_func, (void *)(i + 1));
    }
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(v[i], NULL);
    }
    return 0;
}