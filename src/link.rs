pub trait Link <V> where V: Copy {
    fn value(&self) -> V;
    fn set_value(&mut self, value: V);
}

pub struct Var <V> {
    _value: V,
}

impl <V> Link <V> for Var <V> where V: Copy {
    fn value(&self) -> V {
        self._value
    }
    fn set_value(&mut self, value: V) {
        self._value = value;
    }
}

impl <V> Var <V> where V: Copy {
    pub fn new(value: V) -> Var <V> {
        Var {
            _value: value,
        }
    }
}

pub struct Func <V> {
    getter: Box <dyn Fn() -> V>,
    setter: Box <dyn Fn(V)>,
}

impl <V> Link <V> for Func <V> where V: Copy {
    fn value(&self) -> V {
        let rv = (self.getter)();
        rv
    }
    fn set_value(&mut self, value: V) {
        (self.setter)(value);
    }
}

impl <V> Func <V> {
    pub fn new(getter: Option<Box <dyn Fn() -> V>>, setter: Option<Box <dyn Fn(V)>>) -> Func <V> {
        Func {
            getter: getter.unwrap_or(Box::new(|| {
                panic!("getter is not set");
            })),
            setter: setter.unwrap_or(Box::new(|_| {
                panic!("setter is not set");
            })),
        }
    }
}