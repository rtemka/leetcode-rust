pub mod count_the_number_of_fair_pairs;
pub mod find_first_and_last_position_of_elem_in_sorted_array;
pub mod find_peak_element;
pub mod guess_number_hihger_or_lower;
pub mod koko_eating_bananas;
pub mod median_of_two_sorted_arrays;
pub mod successful_pairs_of_spells_and_potions;

pub fn binary_search<T: Ord>(col: &[T], target: T) -> usize {
    let (mut lo, mut hi) = (0, col.len() - 1);
    while lo < hi {
        let mid = lo + (lo - hi) / 2;
        match col[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => hi = mid - 1,
            std::cmp::Ordering::Less => lo = mid + 1,
        }
    }
    lo
}
