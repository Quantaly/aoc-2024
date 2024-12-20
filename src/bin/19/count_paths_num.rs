//! This is copy-pasted from pathfinding/directed/count_paths.rs,
//! but with usage of `usize` replaced with a type parameter based on `num_traits`
//! and also just using the standard library HashMap because I can't be bothered to import FxHashMap

use num_traits::One;
use std::{collections::HashMap, hash::Hash, iter::Sum};

fn cached_count_paths_num<T, FN, IN, FS, C>(
    start: T,
    successors: &mut FN,
    success: &mut FS,
    cache: &mut HashMap<T, C>,
) -> C
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
    C: Copy + Sum + One,
{
    if let Some(&n) = cache.get(&start) {
        return n;
    }

    let count = if success(&start) {
        C::one()
    } else {
        successors(&start)
            .into_iter()
            .map(|successor| cached_count_paths_num(successor, successors, success, cache))
            .sum()
    };

    cache.insert(start, count);

    count
}

/// Count the total number of possible paths to reach a destination. There must be no loops
/// in the graph, or the function will overflow its stack.
pub fn count_paths_num<T, FN, IN, FS, C>(start: T, mut successors: FN, mut success: FS) -> C
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
    C: Copy + Sum + One,
{
    cached_count_paths_num(
        start,
        &mut successors,
        &mut success,
        &mut HashMap::default(),
    )
}
