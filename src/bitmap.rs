use bit_set::BitSet;

// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct Bitmap {
    width: u32,
    height: u32,
    bits: BitSet,
}

impl Bitmap {
    pub fn new(width: u32, height: u32, bits: BitSet) -> Self {
        Self {
            width,
            height,
            bits,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn is_set(&self, x: u32, y: u32) -> Option<bool> {
        if y >= self.height || x >= self.width {
            None
        } else {
            Some(self.bits.contains((y * self.width + x) as usize))
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        if y >= self.height || x >= self.width {
            return;
        }

        if value {
            self.bits.insert((y * self.width + x) as usize);
        } else {
            self.bits.remove((y * self.width + x) as usize);
        }
    }

    pub fn iter(&self) -> PixelIter {
        PixelIter::new(self)
    }
}

pub struct PixelIter<'a> {
    x: u32,
    y: u32,
    bitmap: &'a Bitmap,
}

impl<'a> PixelIter<'a> {
    pub fn new(bitmap: &'a Bitmap) -> Self {
        Self {
            x: 0,
            y: 0,
            bitmap,
        }
    }
}

impl<'a> Iterator for PixelIter<'a> {
    type Item = ((u32, u32), bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.bitmap.width() {
            self.x  = 0;
            self.y += 1;
        }

        if self.y >= self.bitmap.height() {
            return None;
        }

        let x = self.x;
        let y = self.y;

        self.x += 1;

        Some(((x, y), self.bitmap.is_set(x, y).unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let width   = self.bitmap.width();
        let height  = self.bitmap.height();
        let current = (self.y * height) + self.x;

        (current as usize, Some(((width * height) - current) as usize))
    }
}

impl<'a> ExactSizeIterator for PixelIter<'a> { }
