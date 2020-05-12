use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T: Clone> List<T> {
    pub fn empty() -> List<T> {
        List::Nil
    }

    pub fn is_empty(&self) -> bool {
        match self {
            List::Nil => true,
            _ => false,
        }
    }

    pub fn cons(&self, x: T) -> List<T> {
        List::Cons(x, Rc::new(self.clone()))
    }

    pub fn head(&self) -> Option<T> {
        match self {
            List::Cons(h, _) => Some(h.clone()),
            List::Nil => None,
        }
    }

    pub fn tail(&self) -> List<T> {
        match self {
            List::Nil => List::Nil,
            List::Cons(_, t) => (**t).clone(),
        }
    }
}

#[test]
fn list() {
    //     let l1: List<usize> = Stack::new();
    //     let l2 = l1.cons(3).cons(2).cons(1);
    //     let l3 = l1.cons(5).cons(4);
    //
    //     assert!(l1.is_empty());
    //     assert!(!l2.is_empty());
    //     assert_eq!(
    //         l2,
    //         Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))))
    //     );
    //     assert_eq!(l2.head(), 1);
    //     assert_eq!(l2.tail(), Cons(2, Rc::new(Cons(3, Rc::new(Nil)))));
    //
    //     assert_eq!(l1.append(&l2), l2);
    //     assert_eq!(l2.append(&l1), l2);
    //
    //     assert_eq!(
    //         l2.append(&l3),
    //         Cons(
    //             1,
    //             Rc::new(Cons(
    //                 2,
    //                 Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Cons(5, Rc::new(Nil)))))))
    //             ))
    //         )
    //     );
    //
    //     assert_eq!(
    //         l3.append(&l2),
    //         Cons(
    //             4,
    //             Rc::new(Cons(
    //                 5,
    //                 Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))))
    //             ))
    //         )
    //     );
    //
    //     assert_eq!(l3.update(0, 0), Cons(0, Rc::new(Cons(5, Rc::new(Nil)))));
    //
    //     assert_eq!(l3.update(1, 0), Cons(4, Rc::new(Cons(0, Rc::new(Nil)))));
}
