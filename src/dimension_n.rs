use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Subspace;
use Of;
use ToIndex;
use ToPos;

/// Dimension is a list of numbers, position is a list of numbers.
pub struct DimensionN<T>(PhantomData<T>);

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

impl<'a, T, U>
Count<(&'a [usize], U)> for DimensionN<Subspace<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, (a, b): (&'a [usize], U)) -> usize {
        let subspace: T = Construct::new();
        let data: DimensionN<Data> = Construct::new();
        data.count(a) * subspace.count(b)
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
        for i in (0..dim.len()) {
            prod *= of.count(dim[i]);
        }
        prod
    }
}

impl<'a> ToIndex<&'a [usize], &'a [usize]> for DimensionN<Data> {
    fn to_index(&self, dim: &'a [usize], pos: &'a [usize]) -> usize {
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * dim[i] + pos[i];
        }
        dim_index
    }
}

impl<'a, T, U: Copy, V>
ToIndex<(&'a [usize], U), (&'a [usize], V)> for DimensionN<Subspace<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>
{
    fn to_index(
        &self,
        (a, b): (&'a [usize], U),
        (pa, pb): (&'a [usize], V)
    ) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: DimensionN<Data> = Construct::new();
        data.to_index(a, pa) * count + subspace.to_index(b, pb)
    }
}

impl<'a, T, U: Copy, V: Copy>
ToIndex<&'a [U], &'a [V]> for DimensionN<Of<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>
{
    fn to_index(
        &self,
        dim: &'a [U],
        pos: &'a [V]
    ) -> usize {
        let of: T = Construct::new();
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * of.count(dim[i])
                      + of.to_index(dim[i], pos[i]);
        }
        dim_index
    }
}

impl<'a> ToPos<&'a [usize], &'a mut [usize]> for DimensionN<Data> {
    fn to_pos(&self, dim: &'a [usize], index: usize, pos: &'a mut [usize]) {
        let mut prod = self.count(dim);
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
ToPos<(&'a [usize], U), &'a mut (&'a mut [usize], V)>
for DimensionN<Subspace<T>>
    where
        T: Construct + Count<U> + ToPos<U, &'a mut V>
{
    fn to_pos(
        &self,
        (a, b): (&'a [usize], U),
        index: usize,
        &mut (ref mut head, ref mut tail): &'a mut (&'a mut [usize], V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: DimensionN<Data> = Construct::new();
        let x = index / count;
        data.to_pos(a, index / count, *head);
        subspace.to_pos(b, index - x * count, tail)
    }
}

impl<'a, T, U: Copy, V>
ToPos<&'a [U], &'a mut [&'a mut V]>
for DimensionN<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, &'a mut V>
{
    fn to_pos(
        &self,
        dim: &'a [U],
        index: usize,
        pos: &'a mut [&'a mut V]
    ) {
        let of: T = Construct::new();
        let mut prod = self.count(dim);
        let mut dim_index = index;
        for (i, p) in pos.iter_mut().enumerate().rev() {
            prod /= of.count(dim[i]);
            let p_i = dim_index / prod;
            of.to_pos(dim[i], p_i, *p);
            dim_index -= p_i * prod;
        }
    }
}