#[macro_use]

macro_rules! count_exprs {
    () => { 0 };
    ($e:expr) => { 1 };
    ($e:expr, $($es:expr),+) => { 1 + count_exprs!($($es),*) };
}

/**

    This macro provides a way to initialise any container for which there is a FromIterator implementation.  It allows for both sequence and map syntax to be used, as well as inline type ascription for the result.
    
    For example:
    ```
    # #[macro_use] extern crate grabbag_macros;
    # use std::collections::{HashMap, VecMap};
    # fn main() {
    // Initialise an empty collection.
    let a: Vec<int> = collect![];
    let b: HashMap<String, bool> = collect![];
    // Initialise a sequence.
    let c: String = collect!['a', 'b', 'c'];
    // Initialise a sequence with a type constraint.
    let d = collect![into Vec<_>: 0, 1, 2];
    // Initialise a map collection.
    let e: VecMap<&str> = collect![1 => "one", 2 => "two", 3 => "many", 4 => "lots"];
    // Initialise a map with a type constraint.
    let f: HashMap<_, u8> = collect![into HashMap<int, _>: 42 => 0, -11 => 2];
    # }
    ```
*/

macro_rules! collect {
    // Short-hands for initialising an empty collection.
    [] => { collect![into _] };
    [into $col_ty:ty] => { collect![into $col_ty:] };
    [into $col_ty:ty:] => {
        {
            let col: $col_ty = ::std::default::Default::default();
            col
        }
    };

    // Initialise a sequence with a constrained container type.
    [into $col_ty:ty: $v0:expr] => { collect![into $col_ty: $v0,] };

    [into $col_ty:ty: $v0:expr, $($vs:expr),* $(,)*] => {
        {
            use std::marker::PhantomData;

            const NUM_ELEMS: usize = count_exprs!($v0 $(, $vs)*);

            // This trick is stolen from std::iter, and *should* serve to give the container enough information to pre-allocate sufficient storage for all the elements.
            struct SizeHint<E>(PhantomData<E>);

            impl<E> SizeHint<E> {
                // This method is needed to help the compiler work out which `Extend` impl to use in cases where there is more than one (e.g. `String`).
                #[inline(always)]
                fn type_hint(_: &E) -> SizeHint<E> { SizeHint(PhantomData) }
            }

            impl<E> Iterator for SizeHint<E> {
                type Item = E;

                #[inline(always)]
                fn next(&mut self) -> Option<E> {
                    None
                }

                #[inline(always)]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    (NUM_ELEMS, Some(NUM_ELEMS))
                }
            }

            let mut col: $col_ty = ::std::default::Default::default();
            let v0 = $v0;

            Extend::extend(&mut col, SizeHint::type_hint(&v0));

            Extend::extend(&mut col, Some(v0).into_iter());
            $(Extend::extend(&mut col, Some($vs).into_iter());)*

            col
        }
    };

    // Initialise a sequence with a fully inferred contained type.
    [$($vs:expr),+] => { collect![into _: $($vs),+] };

    // Initialise a map with a constrained container type.
    [into $col_ty:ty: $($ks:expr => $vs:expr),+] => {
        // Maps implement FromIterator by taking tuples, so we just need to rewrite each `a:b` as `(a,b)`.
        collect![into $col_ty: $(($ks, $vs)),+]
    };

    // Initialise a map with a fully inferred contained type.
    [$($ks:expr => $vs:expr),+] => { collect![into _: $($ks => $vs),+] };
}



