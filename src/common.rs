use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

pub type ForeignKey = (String, String);

pub trait Links<Config>: Debug {
    fn convert_fks_to_objs(&mut self, config: &Config);
}

pub trait Linkable<Key, Obj>: KeyLink<Key> {
    fn get_fake(key: String) -> Obj;
}

pub trait KeyLink<Key>
where
    Key: ?Sized,
{
    fn get_key(&self) -> Key;
}

impl<T, K> KeyLink<K> for Rc<T>
where
    T: KeyLink<K>,
{
    fn get_key(&self) -> K {
        self.as_ref().get_key()
    }
}

impl<T, K> KeyLink<K> for RefCell<T>
where
    T: KeyLink<K>,
{
    fn get_key(&self) -> K {
        self.borrow().get_key()
    }
}

impl<T, K, O> Linkable<K, RefCell<O>> for RefCell<T>
where
    T: Linkable<K, O>,
{
    fn get_fake(key: String) -> RefCell<O> {
        RefCell::new(T::get_fake(key))
    }
}

impl<T, K, O> Linkable<K, Rc<O>> for Rc<T>
where
    T: Linkable<K, O>,
{
    fn get_fake(key: String) -> Rc<O> {
        Rc::new(T::get_fake(key))
    }
}

impl<T, K, O> Linkable<K, Weak<O>> for Weak<T>
where
    T: Linkable<K, O>,
{
    fn get_fake(key: String) -> Weak<O> {
        let fake = T::get_fake(key);
        Rc::downgrade(&Rc::new(fake))
    }
}

impl<T, K> KeyLink<K> for Weak<T>
where
    T: KeyLink<K>,
{
    fn get_key(&self) -> K {
        self.upgrade().unwrap().get_key()
    }
}
