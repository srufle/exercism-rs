#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

//  Inspired by
// https://exercism.org/tracks/rust/exercises/sublist/solutions/rsalmei

// https://doc.rust-lang.org/std/primitive.slice.html#method.windows
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
pub fn sublist<T: PartialEq + std::cmp::Ord + std::fmt::Debug>(
    first_list: &[T],
    second_list: &[T],
) -> Comparison {
    // - If both lists are empty = Equal
    // - If the list has length, use it to create
    //   a window into the "other" list checking that
    //   subslice exists "any"where in the other list
    //   do it in both directions to get the superlist case
    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|list| list == first_list);
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|list| list == second_list);

    use Comparison::*;
    // liked that the match uses a tuple of bools
    match (sublist, superlist) {
        (true, true) => Equal,
        (true, false) => Sublist,
        (false, true) => Superlist,
        (false, false) => Unequal,
    }
}
