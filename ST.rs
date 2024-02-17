macro_rules! array_2d {
    ($n: expr, $m: expr) => {
        vec![vec![0; $m]; $n]
    }
}

struct ST {
    mx: Vec<Vec<i32>>,
    mi: Vec<Vec<i32>>
}

impl ST {
    fn build(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut mx = array_2d!(n+1, 33);
        let mut mi = array_2d!(n+1, 33);

        for i in 1..n {
            mx[i][0] = nums[i];
            mi[i][0] = nums[i];
        }

        for j in 1..=((31 - (n as i32).leading_zeros()) as usize) {
            for i in 1..=(n - (1 << j) + 1) {
                mx[i][j] = i32::max(mx[i][j - 1], mx[i + (1 << (j - 1))][j - 1]);
                mi[i][j] = i32::min(mi[i][j - 1], mi[i + (1 << (j - 1))][j - 1]);
            }
        }

        ST {mx, mi}
    }

    fn query_min(&self, l: usize, r: usize) -> i32 {
        let x = ((r - l + 1) as f64).log2() as usize;
        i32::min(self.mi[l][x], self.mi[r - (1 << x) + 1][x])
    }

    fn query_max(&self, l: usize, r: usize) -> i32 {
        let x = ((r - l + 1) as f64).log2() as usize;
        i32::max(self.mx[l][x], self.mx[r - (1 << x) + 1][x])
    }
}

