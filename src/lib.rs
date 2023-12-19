#![allow(dead_code)]

/// A fixed-sized static pool of items.
///
/// `StaticPool` manages `N` items of type `T`. The items
/// are stored in a static array, and are accessed via
/// `StaticPoolHandle`s.
///
/// ```
/// use static_pool::StaticPool;
///
/// let mut pool: StaticPool<u64, 128> = StaticPool::new();
/// let handle = pool.alloc().unwrap();
/// let num = pool.get_mut(handle).unwrap();
/// *num = 128;
/// assert_eq!(pool.get(handle), Some(&128));
/// ```
///
pub struct StaticPool<T, const N: usize> {
    items: [T; N],
    free: [bool; N],
    len: usize,
}

pub type StaticPoolHandle = usize;

impl<T, const N: usize> StaticPool<T, N>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            items: std::array::from_fn(|_| Default::default()),
            free: [true; N],
            len: 0,
        }
    }

    fn next_free_handle(&mut self) -> Option<StaticPoolHandle> {
        for i in 0..N {
            if self.free[i] {
                self.free[i] = false;
                let handle = i + 1;
                return Some(handle);
            }
        }

        None
    }

    pub fn alloc(&mut self) -> Option<StaticPoolHandle> {
        let handle = self.next_free_handle()?;
        self.items[handle - 1] = Default::default();
        Some(handle)
    }

    pub fn free(&mut self, handle: StaticPoolHandle) {
        if handle > 0 && handle <= N {
            if !self.free[handle - 1] {
                self.free[handle - 1] = true;
            }
        }
    }

    pub fn get(&self, handle: StaticPoolHandle) -> Option<&T> {
        if handle > 0 && handle <= N && !self.free[handle - 1] {
            Some(&self.items[handle - 1])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, handle: StaticPoolHandle) -> Option<&mut T> {
        if handle > 0 && handle <= N && !self.free[handle - 1] {
            Some(&mut self.items[handle - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut p: StaticPool<i32, 1024> = StaticPool::new();
        let handle = p.alloc().unwrap();
        assert_eq!(p.get(handle), Some(&0));

        *p.get_mut(handle).unwrap() = 100;
        assert_eq!(p.get(handle), Some(&100));

        p.free(handle);
        assert_eq!(p.get(handle), None);
    }

    #[derive(Debug, Default)]
    struct Data {
        x: usize,
        s: String,
    }

    #[test]
    fn test_with_data() {
        let mut p: StaticPool<Data, 1024> = StaticPool::new();
        let handle = p.alloc().unwrap();
        let data = p.get_mut(handle).unwrap();
        data.x = 128;
        data.s = "Some data".to_owned();

        assert_eq!(p.get(handle).unwrap().x, 128);
        assert_eq!(p.get(handle).unwrap().s, "Some data");
    }

    #[test]
    fn test_alloc_free() {
        let mut p: StaticPool<u64, 4> = StaticPool::new();
        let handle = p.alloc();
        assert_eq!(handle, Some(1));
        let handle = p.alloc();
        assert_eq!(handle, Some(2));
        let handle = p.alloc();
        assert_eq!(handle, Some(3));
        let handle = p.alloc();
        assert_eq!(handle, Some(4));
        let handle = p.alloc();
        assert_eq!(handle, None);
        let handle = p.alloc();
        assert_eq!(handle, None);

        p.free(2);
        let handle = p.alloc();
        assert_eq!(handle, Some(2));
        let handle = p.alloc();
        assert_eq!(handle, None);
    }
}
