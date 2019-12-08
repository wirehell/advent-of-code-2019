mod mutable_union_set {
    use std::rc::Rc;
    use std::borrow::Borrow;
    use std::cell::{RefCell, Ref};
    use std::ops::Deref;


    /*
    #[derive(Eq,PartialEq,Debug)]
    pub struct UnionSet<T :Eq> {
        elem: T,
        parent: Rc<RefCell<Option<UnionSet<T>>>>,
    }


    impl<K: Eq> UnionSet<K> {
        pub fn of(object: K) -> UnionSet<K> {
            UnionSet { elem: object, parent: Rc::new(RefCell::new(None) )}
        }
    }

    pub fn find_root<K :Eq>(s :&UnionSet<K>) -> &UnionSet<K> {
        let val ;
        {
            let cell :&RefCell<Option<UnionSet<K>>> = s.parent.borrow();
            let borrow :Ref<Option<UnionSet<K>>> = cell.borrow();
            match borrow.as_ref() {
                None => val = s,
                Some(parent) => {
                    val = find_root(parent).clone()
                },
            };
        }
        return val;
    }



    pub fn identity<K: Eq>(s: Rc<UnionSet<K>>) -> Rc<UnionSet<K>> {
    }
        return match s.parent.borrow() {
            Some(n) => identity(n),
            None => s,
        }
    }

    pub fn union<K: Eq>(s1: UnionSet<K>, s2: UnionSet<K>) {
        let r1 = identity(Rc::new(s1));
        let r2 = identity(Rc::new(s2));
        if r1 == r2 {
            return;
        }

        if r1.rank > r2.rank {
            r2.parent.replace(Some(r1));
        }
    }

    pub fn is_same_component<K: Eq>(s1: UnionSet<K>, s2: UnionSet<K>) -> bool {
        return identity(Rc::new(s1)) == identity(Rc::new(s2));
    }
*/

}
#[cfg(test)]
mod tests {
    /*
    use crate::mutable_union_set::mutable_union_set::{UnionSet, find_root};

    #[test]
    fn test_simple() {
        let s1 = UnionSet::of(32);
        let s2 = UnionSet::of(27);

        let b = find_root(&s1);

//        assert_eq!(is_same_component(s1, s2), false);
        //union(s1,s2);
//        assert_eq!(is_same_component(s1, s2), true);

    }

    #[test]
    fn it_doesnt_find_anything_in_empty() {
    }
    */
}
