
#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;
    use std::sync::{Arc, Mutex};
    use log::info;
    use crate::db;

    lazy_static! {
        static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    }

    pub fn init() {
        let mut initiated = INITIATED.lock().unwrap();
        if *initiated == false {
            info!("Initiating app for intergation test");
            db::init();
            *initiated = true;
        }
    }

    pub fn before_db_test() {
        info!("Before DB test");
        db::before_test();
    }
}

// #[cfg(test)]
// #[ctor::ctor]
// fn init_test() {
//     info!("Before any test code");
//     // tests::init();
// }