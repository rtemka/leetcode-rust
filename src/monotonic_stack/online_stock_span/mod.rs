// https://leetcode.com/problems/online-stock-span/description
struct StockSpanner {
    stocks: Vec<(i32, Option<usize>)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { stocks: Vec::new() }
    }

    // not monotonic stack solution actually
    fn next(&mut self, price: i32) -> i32 {
        let l = self.stocks.len();
        let res = if l == 0 {
            (1, None)
        } else if self.stocks[l - 1].0 > price {
            (1, Some(l - 1))
        } else {
            let mut i = l - 1;
            let mut greater = None;
            while let Some(p) = self.stocks[i].1 {
                i = p;
                if self.stocks[p].0 > price {
                    greater = Some(p);
                    break;
                }
            }
            if greater.is_none() {
                (l + 1, None)
            } else {
                (l - i, greater)
            }
        };
        self.stocks.push((price, res.1));
        res.0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_spanner() {
        let mut spanner = StockSpanner::new();
        assert_eq!(1, spanner.next(100));
        assert_eq!(1, spanner.next(80));
        assert_eq!(1, spanner.next(60));
        assert_eq!(2, spanner.next(70));
        assert_eq!(1, spanner.next(60));
        assert_eq!(4, spanner.next(75));
        assert_eq!(6, spanner.next(80));

        let mut spanner = StockSpanner::new();
        assert_eq!(1, spanner.next(80));
        assert_eq!(2, spanner.next(85));

        let mut spanner = StockSpanner::new();
        assert_eq!(1, spanner.next(31));
        assert_eq!(2, spanner.next(41));
        assert_eq!(3, spanner.next(48));
        assert_eq!(4, spanner.next(59));
        assert_eq!(5, spanner.next(79));
    }
}
