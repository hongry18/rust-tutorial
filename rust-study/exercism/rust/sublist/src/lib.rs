#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> Comparison {
    let l1_len = l1.len();
    let l2_len = l2.len();

    if l1_len == l2_len {
        return if l1 == l2 {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    }

    if l1_len < l2_len {
        if l1_len == 0 {
            return Comparison::Sublist;
        }
        return match l2.windows(l1_len).find(|i| i == &l1) {
            Some(_) => Comparison::Sublist,
            None => Comparison::Unequal,
        };
    }

    if l1_len > l2_len {
        if l2_len == 0 {
            return Comparison::Superlist;
        }
        return match l1.windows(l2_len).find(|i| i == &l2) {
            Some(_) => Comparison::Superlist,
            None => Comparison::Unequal,
        };
    }

    Comparison::Unequal
}

#[test]
fn recurring_values_sublist() {
    assert_eq!(
        Comparison::Sublist,
        sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
    );
}
