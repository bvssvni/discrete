use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Dimension is a list of numbers, position is a list of numbers.
pub struct DimensionN<T = Data>(PhantomData<T>);

impl<T> Construct for DimensionN<T> {
    fn new() -> DimensionN<T> { DimensionN(PhantomData) }
}

impl<'a> Count<&'a [usize]> for DimensionN<Data> {
    fn count(&self, dim: &'a [usize]) -> usize {
        let mut prod = 1;
        for i in 0..dim.len() {
            prod *= dim[i];
        }
        prod
    }
}

impl<'a, T, U: Copy>
Count<&'a [U]> for DimensionN<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: &'a [U]) -> usize {
        let of: T = Construct::new();
        let mut prod = 1;
        for i in 0..dim.len() {
            prod *= of.count(dim[i]);
        }
        prod
    }
}

impl<'a> Zero<&'a [usize], Vec<usize>> for DimensionN<Data> {
    fn zero(&self, dim: &'a [usize]) -> Vec<usize> {
        vec![0, dim.len()]
    }
}

impl<'a, T, U, V>
Zero<&'a [U], Vec<V>>
for DimensionN<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>,
        U: Copy
{
    fn zero(&self, dim: &'a [U]) -> Vec<V> {
        let of: T = Construct::new();
        let mut v = Vec::with_capacity(dim.len());
        for i in 0..dim.len() {
            v.push(of.zero(dim[i]));
        }
        v
    }
}

impl<'a> ToIndex<&'a [usize], Vec<usize>> for DimensionN<Data> {
    fn to_index(&self, dim: &'a [usize], pos: &Vec<usize>) -> usize {
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * dim[i] + pos[i];
        }
        dim_index
    }
}

impl<'a, T, U: Copy, V: Copy>
ToIndex<&'a [U], Vec<V>> for DimensionN<Of<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>
{
    fn to_index(
        &self,
        dim: &'a [U],
        pos: &Vec<V>
    ) -> usize {
        let of: T = Construct::new();
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * of.count(dim[i])
                      + of.to_index(dim[i], &pos[i]);
        }
        dim_index
    }
}

impl<'a> ToPos<&'a [usize], Vec<usize>> for DimensionN<Data> {
    fn to_pos(&self, dim: &'a [usize], index: usize, pos: &mut Vec<usize>) {
        unsafe { pos.set_len(0); }
        let mut prod = self.count(dim);
        for _ in 0..dim.len() {
            pos.push(0);
        }
        let mut dim_index = index;
        for i in (0..dim.len()).rev() {
            prod /= dim[i];
            let p_i = dim_index / prod;
            *pos.get_mut(i).unwrap() = p_i;
            dim_index -= p_i * prod;
        }
    }
}

impl<'a, T, U: Copy, V>
ToPos<&'a [U], Vec<V>>
for DimensionN<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>
{
    fn to_pos(
        &self,
        dim: &'a [U],
        index: usize,
        pos: &mut Vec<V>
    ) {
        let of: T = Construct::new();
        let mut prod = self.count(dim);
        let mut dim_index = index;
        for (i, p) in pos.iter_mut().enumerate().rev() {
            prod /= of.count(dim[i]);
            let p_i = dim_index / prod;
            of.to_pos(dim[i], p_i, p);
            dim_index -= p_i * prod;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<DimensionN, &[usize], Vec<usize>>();
        does_zero::<DimensionN, &[usize], Vec<usize>>();
        does_zero::<DimensionN<Of<Pair>>, &[usize],
            Vec<(usize, usize)>>();
    }

    #[test]
    fn data() {
        let x: DimensionN = Construct::new();
        let dim = &[3, 3];
        assert_eq!(x.count(dim), 9);
        assert_eq!(x.to_index(dim, &vec![0, 0]), 0);
        assert_eq!(x.to_index(dim, &vec![1, 0]), 1);
        assert_eq!(x.to_index(dim, &vec![0, 1]), 3);
        let mut new_pos = vec![0, 0];
        x.to_pos(dim, 3, &mut new_pos);
        assert_eq!(&new_pos, &[0, 1]);
    }

    #[test]
    fn of() {
        let x: DimensionN<Of<Pair>> = Construct::new();
        let dim = [3, 4];
        assert_eq!(x.count(&dim), 18);
        assert_eq!(x.to_index(&dim, &vec![(0, 1), (0, 1)]), 0);
        assert_eq!(x.to_index(&dim, &vec![(0, 2), (0, 1)]), 1);
        assert_eq!(x.to_index(&dim, &vec![(1, 2), (0, 1)]), 2);
        assert_eq!(x.to_index(&dim, &vec![(0, 1), (0, 2)]), 3);
        let mut pos = vec![(0, 0), (0, 0)];
        x.to_pos(&dim, 3, &mut pos);
        assert_eq!(pos[0], (0, 1));
        assert_eq!(pos[1], (0, 2));
    }
}
