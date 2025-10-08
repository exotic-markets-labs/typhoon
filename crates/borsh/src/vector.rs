use {crate::BorshAccessor, core::ops::RangeBounds, std::marker::PhantomData};

pub struct BorshVector<'a, T>
where
    T: BorshAccessor<'a>,
{
    inner: &'a [u8],
    _phantom: PhantomData<T>,
}

impl<'a, T> BorshVector<'a, T>
where
    T: BorshAccessor<'a>,
{
    pub fn new(inner: &'a [u8]) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn get(&'a self, range: impl RangeBounds<usize> + 'a) -> impl Iterator<Item = T> + 'a {
        let elements_nb = u32::convert(&self.inner[..4]) as usize;
        let mut offset = 4;
        (0..elements_nb).filter_map(move |i| {
            let len = T::len(&self.inner[offset..]);
            if range.contains(&i) {
                let el = T::convert(&self.inner[offset..(offset + len)]);
                offset += len;
                Some(el)
            } else {
                offset += len;
                None
            }
        })
    }

    pub fn at(&'a self, index: usize) -> Option<T> {
        let elements_nb = u32::convert(&self.inner[..4]) as usize;
        if index >= elements_nb {
            return None;
        }

        let mut offset = 4;
        for i in 0..=index {
            let len = T::len(&self.inner[offset..]);
            if i == index {
                return Some(T::convert(&self.inner[offset..(offset + len)]));
            }
            offset += len;
        }
        None
    }
}

// impl<'a, T> BorshVector<'a, T>
// where
//     T: BorshAccessor<'a> + BorshWriter,
// {
//     pub fn push(&'a mut self, element: T) {
//         let elements_nb = u32::convert(&self.inner[..4]) as usize;
//         // self
//     }
// }

impl<'a, T> BorshAccessor<'a> for BorshVector<'a, T>
where
    T: BorshAccessor<'a>,
{
    #[inline(always)]
    fn len(data: &'a [u8]) -> usize {
        let mut offset = 4;
        let el_nb = u32::convert(&data[..offset]) as usize;
        for _ in 0..el_nb {
            offset += T::len(&data[offset..]);
        }
        offset
    }

    #[inline(always)]
    fn convert(data: &'a [u8]) -> Self {
        BorshVector::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_vector() {
        let data = borsh::to_vec(&vec!["j'aime", "les", "pâtes"]).unwrap();
        let vector: BorshVector<'_, &str> = BorshVector::new(&data);

        assert_eq!(
            vector.get(..).collect::<Vec<_>>(),
            &["j'aime", "les", "pâtes"]
        );

        assert_eq!(vector.at(0).unwrap(), "j'aime");
        assert_eq!(vector.at(1).unwrap(), "les");
        assert_eq!(vector.at(2).unwrap(), "pâtes");
        assert!(vector.at(3).is_none());
    }

    #[test]
    fn test_simple_vector() {
        let data = borsh::to_vec(&vec![1, 2, 3]).unwrap();
        let vector: BorshVector<'_, u32> = BorshVector::new(&data);

        let mut el = 1;
        for inner_el in vector.get(..) {
            assert_eq!(el, inner_el);
            el += 1;
        }
    }

    #[test]
    fn test_complex_vector() {
        let data = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let serialized = borsh::to_vec(&data).unwrap();

        let vector: BorshVector<'_, BorshVector<'_, u32>> = BorshVector::new(&serialized);

        let mut result = 2;
        let el = vector.at(1).unwrap();

        for inner_el in el.get(..) {
            assert_eq!(result, inner_el);
            result += 1;
        }
    }
}
