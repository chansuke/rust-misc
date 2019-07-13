pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

//implブロック内に関連関数やメソッドを定義。
impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)           // T型のデフォルト値をsize個作る
            .collect::<Vec<_>>()  // Vec<T> 型に変換
            .into_boxed_slice()   // Box<[T]>に変換
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        //match self.get(index) {
        //    Some(v) => v,
        //    None => default,
        //}
        self.get(index).unwrap_or(default)
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heal(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            for (i, elem) in old_elments.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }
}
