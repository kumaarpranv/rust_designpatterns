use crate::creational::singleton::Singleton;

#[test]
fn singleton_test() {
    let singleton = Singleton::instance();
    let data = singleton.lock().unwrap().get_data();
    let singleton1 = Singleton::instance();
    let data1 = singleton1.lock().unwrap().get_data();
    assert_eq!(data, data1);
}
