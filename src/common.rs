use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type ForeignKey = (String, String);

pub trait Links<Config>: Debug {
    fn get_foreign_keys(&self) -> Vec<ForeignKey>;
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
