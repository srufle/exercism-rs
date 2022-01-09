#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_list_in<T: PartialEq + std::cmp::Ord + std::fmt::Debug>(
    first_list: &[T],
    second_list: &[T],
) -> bool {
    let mut start = 0;
    let mut end = first_list.len();
    while end <= second_list.len() {
        let work_list = &second_list[start..end];
        if first_list.eq(work_list) {
            return true;
        }
        start += 1;
        end += 1;
    }
    false
}

pub fn sublist<T: PartialEq + std::cmp::Ord + std::fmt::Debug>(
    first_list: &[T],
    second_list: &[T],
) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }

    if is_list_in(first_list, second_list) {
        return Comparison::Sublist;
    } else if is_list_in(second_list, first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

pub fn sublist_v1<T: PartialEq + std::cmp::Ord + std::fmt::Debug>(
    first_list: &[T],
    second_list: &[T],
) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }

    let first_len = first_list.len();
    let second_len = second_list.len();

    if first_len < second_len {
        let mut start = 0;
        let mut end = first_len;
        while end <= second_len {
            let work_list = &second_list[start..end];
            if first_list.eq(work_list) {
                return Comparison::Sublist;
            }
            start += 1;
            end += 1;
        }
    } else if first_len > second_len {
        let mut start = 0;
        let mut end = second_len;
        while end <= first_len {
            let work_list = &first_list[start..end];
            if second_list.eq(work_list) {
                return Comparison::Superlist;
            }
            start += 1;
            end += 1;
        }
    }

    Comparison::Unequal
}
