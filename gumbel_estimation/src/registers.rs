#[derive(Debug, Clone)]
pub struct Registers {
    buf: Vec<u32>,
    count: usize,
    // zeros: usize,
}

impl Registers {
    pub const MIN_VAL: f32 = -16.0;
    pub const MAX_VAL: f32 = 15.0;
    pub const BIAS: i32 = 16;

    pub const SIZE: usize = 5;
    const COUNT_PER_WORD: usize = 32 / Self::SIZE;
    const MASK: u32 = (1 << Self::SIZE) - 1;

    /// Encodes a floating point value with a shift into 5-bit representation
    pub fn encode(value: f32, shift: f32) -> u32 {
        ((f32::floor(value + shift).clamp(Self::MIN_VAL, Self::MAX_VAL) as i32) + Self::BIAS) as u32
    }

    /// Decodes a register value using a shift as a float
    pub fn decode(reg_bits: u32, shift: f32) -> f64 {
        (reg_bits as i32 - Self::BIAS) as f64 - shift as f64 + 0.5
    }
    
    pub fn new(count: usize) -> Self {
        Self {
            buf: vec![0; ceil(count, Self::COUNT_PER_WORD)],
            count,
            // zeros: count,
        }
    }

/*
 *    #[inline]
 *    pub fn get(&self, index: usize) -> u32 {
 *        let (quot, rem) = (
 *            index / Self::COUNT_PER_WORD,
 *            index % Self::COUNT_PER_WORD,
 *        );
 *
 *        (self.buf[quot] >> (rem * Self::COUNT_PER_WORD)) & Self::MASK
 *    }
 */

    pub fn set(&mut self, index: usize, mut value: u32) {
        value = u32::min(value, (1 << 5) - 1);

        let (quot, rem) = (
            index / Self::COUNT_PER_WORD,
            index % Self::COUNT_PER_WORD,
        );

        let mask = Self::MASK << (rem * Self::SIZE);

        self.buf[quot] = (self.buf[quot] & !mask)
                | (value << (rem * Self::SIZE));
    }

    pub fn set_greater(&mut self, index: usize, mut value: u32) {
        value = u32::min(value, (1 << 5) - 1);

        let (quot, rem) = (
            index / Self::COUNT_PER_WORD,
            index % Self::COUNT_PER_WORD,
        );

        let curr = (self.buf[quot] >> (rem * Self::SIZE)) & Self::MASK;

        if value > curr {
            let mask = Self::MASK << (rem * Self::SIZE);

            self.buf[quot] = (self.buf[quot] & !mask)
                | (value << (rem * Self::SIZE));
        }
    }

    /*
     *#[inline]
     *pub fn iter(&self) -> RegistersIterator {
     *    RegistersIterator{registers: self, index: 0}
     *}
     */

    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.buf.iter()
            .flat_map(|val| {
                (0..Self::COUNT_PER_WORD).map(move |i| {
                    (val >> (i * Self::SIZE)) & Self::MASK
                })
            })
            .take(self.count)
    }
}

/*
 *pub struct RegistersIterator<'a> {
 *    registers: &'a Registers,
 *    index: usize,
 *}
 *
 *impl<'a> Iterator for RegistersIterator<'a> {
 *    type Item = u32;
 *
 *    fn next(&mut self) -> Option<Self::Item> {
 *        if self.index < self.registers.count {
 *            let value = self.registers.get(self.index);
 *            self.index += 1;
 *            Some(value)
 *        } else {
 *            None
 *        }
 *    }
 *}
 */

#[inline(always)]
pub fn ceil(num: usize, denom: usize) -> usize {
    (num + denom - 1) / denom
}

