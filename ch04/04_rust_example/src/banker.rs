use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],         // 利用可能なリソース
    allocation: [[usize; NRES]; NTH], // スレッドiが確保中のリソース
    max: [[usize; NRES]; NTH],        // スレッドiが必要とするリソースの最大値
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    // 現在の状態がデッドロックに陥らないかを検査 1
    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH]; // スレッドiはリソース取得と解放に成功？ 2
        let mut work = self.available.clone(); // 利用可能なリソースのシミュレート値 3

        loop {
            // すべてのスレッドiとリソースjにおいて、 4
            // finish[i] == false && work[j] >= (self.max[i][j] - self.allocation[i][j])
            // を満たすようなスレッドを見つける。
            let mut found = false;
            let mut num_true = 0;
            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                // need[j] = self.max[i][j] - self.allocation[i][j] を計算し、 5
                // すべてのリソースjにおいて、work[j] >= need[j] かを判定
                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                if is_avail {
                    // スレッドiがリソース確保可能
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a // スレッドiの現在確保しているリソースを返却 6
                    }
                    break;
                }
            }

            if num_true == NTH {
                // すべてのスレッドがリソース確保可能なら安全 7
                return true;
            }

            if !found {
                // スレッドがリソースを確保できずに 8
                break;
            }
        }

        false
    }

    // id番目のスレッドが、resourceを1つ取得 9
    fn take(&mut self, id: usize, resource: usize) -> bool {
        // スレッド番号、リソース番号を検査
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        // リソースを確保試みる 10
        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        // 11
        if self.is_safe() {
            true // リソース確保成功
        } else {
            // リソース確保に失敗したため、状態を復元
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    // id番目のスレッドが、resourceを1つ解放 12
    fn release(&mut self, id: usize, resource: usize) {
        // スレッド番号、リソース番号を検査
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(available, max))),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource)
    }
}

const NUM_LOOP: usize = 100000;

pub fn main() {
    // 利用可能な箸の数と、哲学者の利用する最大の箸の数を設定
    let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
    let banker0 = banker.clone();

    let philosopher0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            // 箸0と1を確保
            while !banker0.take(0, 0) {}
            while !banker0.take(0, 1) {}

            println!("0: eating");

            // 箸0と1を解放
            banker0.release(0, 0);
            banker0.release(0, 1);
        }
    });

    let philosopher1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            // 箸1と0を確保
            while !banker.take(1, 1) {}
            while !banker.take(1, 0) {}

            println!("1: eating");

            // 箸1と0を解放
            banker.release(1, 1);
            banker.release(1, 0);
        }
    });

    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}
