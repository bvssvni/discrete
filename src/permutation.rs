
use std::marker::PhantomData;
use std::default::Default;

use Construct;
use Count;
use Data;
use Subspace;
use ToPos;
use ToIndex;
use Of;

/// Dimension is natural number, position is a list of numbers.
pub struct Permutation<T>(PhantomData<T>);

impl<T> Construct for Permutation<T> {
    fn new() -> Permutation<T> {
        Permutation(PhantomData)
    }
}

impl Count<usize> for Permutation<Data> {
    fn count(&self, dim: usize) -> usize {
        let mut res = 1;
        for x in 1..dim + 1 {
            res *= x;
        }
        res
    }
}

impl<T, U>
Count<(usize, U)> for Permutation<Subspace<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, (a, b): (usize, U)) -> usize {
        let subspace: T = Construct::new();
        let data: Permutation<Data> = Construct::new();
        data.count(a) * subspace.count(b)
    }
}

impl<T, U> Count<U> for Permutation<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        let mut res = 1;
        for x in 1..of.count(dim) + 1 {
            res *= x;
        }
        res
    }
}

impl<'a> ToIndex<usize, &'a [usize]> for Permutation<Data> {
    fn to_index(&self, dim: usize, pos: &'a [usize]) -> usize {
        let mut index = 0;
        let mut count = 1;
        for (i, &x) in pos.iter().enumerate().rev() {
            let lower = pos[..i].iter().filter(|&&y| y < x).count();
            index += count * (x - lower);
            count *= dim - i;
        }
        index
    }
}

impl<'a, T, U: Copy, V>
ToIndex<(usize, U), (&'a [usize], V)>
for Permutation<Subspace<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>
{
    fn to_index(&self, (a, b): (usize, U), (pa, pb): (&'a [usize], V)) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: Permutation<Data> = Construct::new();
        data.to_index(a, pa) * count + subspace.to_index(b, pb)
    }
}

impl<'a, T, U: Copy, V: Copy> ToIndex<U, &'a [V]> for Permutation<Of<T>>
    where
        T: Construct + ToIndex<U, V> + Count<U>
{
    fn to_index(&self, dim: U, pos: &'a [V]) -> usize {
        let of: T = Construct::new();
        let mut index = 0;
        let dim_count = of.count(dim);
        let mut count = 1;
        for (i, x) in pos.iter()
            .map(|&x| of.to_index(dim, x))
            .enumerate().rev() {
            let lower = pos[..i].iter()
                .map(|&y| of.to_index(dim, y))
                .filter(|&y| y < x).count();
            index += count * (x - lower);
            count *= dim_count - i;
        }
        index
    }
}

impl ToPos<usize, Vec<usize>> for Permutation<Data> {
    fn to_pos(&self, dim: usize, mut index: usize, pos: &mut Vec<usize>) {
        unsafe { pos.set_len(0); }

        let mut count = 1;
        for (j, x) in (1..dim + 1).enumerate() {
            count *= x;
            pos.push(j);
        }

        for i in 0..dim {
            let block = count / (dim - i);
            let ind = index / block;
            let item = pos.remove(ind);
            pos.push(item);
            count /= dim - i;
            index -= ind * block;
        }
    }
}

impl<T, U: Copy, V>
ToPos<(usize, U), (Vec<usize>, V)>
for Permutation<Subspace<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>
{
    fn to_pos(
        &self,
        (a, b): (usize, U),
        index: usize,
        &mut (ref mut head, ref mut tail): &mut (Vec<usize>, V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: Permutation<Data> = Construct::new();
        let x = index / count;
        data.to_pos(a, index / count, head);
        subspace.to_pos(b, index - x * count, tail)
    }
}

impl<T, U, V> ToPos<U, Vec<V>> for Permutation<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>,
        U: Copy,
        V: Default
{
    fn to_pos(&self, dim: U, mut index: usize, pos: &mut Vec<V>) {
        let of: T = Construct::new();
        let of_count = of.count(dim);
        pos.clear();

        let mut count = 1;
        for (j, x) in (1..of_count + 1).enumerate() {
            count *= x;
            let mut new_pos: V = Default::default();
            of.to_pos(dim, j, &mut new_pos);
            pos.push(new_pos);
        }

        for i in 0..of_count {
            let block = count / (of_count - i);
            let ind = index / block;
            let item = pos.remove(ind);
            pos.push(item);
            count /= of_count - i;
            index -= ind * block;
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn data() {
        let permutation: Permutation<Data> = Construct::new();
        assert_eq!(permutation.count(1), 1);
        assert_eq!(permutation.count(2), 2);
        assert_eq!(permutation.count(3), 6);
        assert_eq!(permutation.count(4), 24);

        let mut pos = Vec::new();
        let dim = 4;
        let count = permutation.count(dim);
        for i in 0..count {
            permutation.to_pos(dim, i, &mut pos);
            let index = permutation.to_index(dim, &pos);
            assert_eq!(index, i);
        }
    }

    #[test]
    fn of() {
        let space: Permutation<Of<Pair<Data>>> = Construct::new();
        let dim = 3;
        let count = space.count(dim);
        let mut pos = Vec::new();
        for i in 0..count {
            space.to_pos(dim, i, &mut pos);
            let index = space.to_index(dim, &pos);
            assert_eq!(index, i);
        }
    }
}
