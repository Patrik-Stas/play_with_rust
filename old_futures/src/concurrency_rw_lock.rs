use std::sync::RwLock;

pub fn run() {
    let lock = RwLock::new(5);
    {

        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
        println!("First Write lock written stuff");

    }

// many reader locks can be held at once
    {
        // if the write lock from above still existed here, we would get panic:
        // "thread 'main' panicked at 'rwlock read lock would result in deadlock', src/libstd/sys/unix/rwlock.rs:48:13"
        // that's because read() will block as far as there is write lock. But if we merge block above
        // with this one, write lock would still exist here and we would wait here forever
        println!("Going to create first read lock");
        let r1 = lock.read().unwrap();

        println!("Going to create second read lock");
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 6);
        assert_eq!(*r2, 6);
        println!("Two read locks verified value. Keep holding read lock");
    } // read locks are dropped at this point
//// only one write lock may be held, however
    {
        //
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 7);
        println!("Write locks written stuff");
    } // write lock is dropped

    {
        let mut w = lock.write().unwrap();
//        let mut w2 = lock.write().unwrap();
        // uncommenting live above, we get runtime panic:
        // "thread 'main' panicked at 'rwlock write lock would result in deadlock', src/libstd/sys/unix/rwlock.rs:79:13"
    }

}