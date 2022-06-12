use std::cmp::Ordering;

use crate::orderbook_aggregator::{Level, Summary};

pub enum QuoteType {
    ASKS,
    BIDS,
}


pub struct Aggregator {}

impl Aggregator {
    pub fn aggregate_top(
        n: usize,
        summary_a: Option<Summary>,
        summary_b: Option<Summary>,
        quote_type: QuoteType,
    ) -> Vec<Level> {

        let top_n_levels = |summary: Option<Summary>| -> Vec<Level> {
            match summary {
                Some(summary) => {
                    match quote_type {
                        QuoteType::ASKS => Aggregator::top(n, summary.asks),
                        QuoteType::BIDS => Aggregator::top(n, summary.bids)
                    }
                }
                _ => Vec::new()
            }
        };

        let a_top_n_levels = top_n_levels(summary_a);
        let b_top_n_levels = top_n_levels(summary_b);

        let combined_levels = Aggregator::combine_and_sort(
            a_top_n_levels,
            b_top_n_levels,
            quote_type
        );

        Aggregator::top(n, combined_levels)
    }

    /// Combine the arguments `levels_a` and `level_b` and sort them such that
    /// the best level is at the top. The switch `order` may be used to sort
    /// ascending (placing highest prices first as required for bids) or
    /// descending (placing lowest prices first as required for asks) order.
    /// When sorting larger amount for a price quote is placed before the
    /// smaller amount such that best quote is at the top.
    fn combine_and_sort(
        mut levels_a: Vec<Level>,
        mut levels_b: Vec<Level>,
        quote_type: QuoteType,
    ) -> Vec<Level> {
        levels_a.append(&mut levels_b);

        let compare = match quote_type {
            QuoteType::ASKS => |a: &Level, b: &Level| -> Ordering {
                if a.price == b.price {
                    return a.amount.partial_cmp(&b.amount).unwrap();
                }
                a.price.partial_cmp(&b.price).unwrap()
            },
            QuoteType::BIDS => |a: &Level, b: &Level| -> Ordering {
                if a.price == b.price {
                    return b.amount.partial_cmp(&a.amount).unwrap();
                }
                b.price.partial_cmp(&a.price).unwrap()
            },
        };

        levels_a.sort_by(compare);

        levels_a
    }

    /// Gets the top `n` elements from `levels`. This function expects `levels`
    /// to be sorted in the order expected by the consumer of the function.
    pub fn top(n: usize, mut levels: Vec<Level>) -> Vec<Level> {
        if levels.len() > n {
            let _ = levels.split_off(n);
        }

        levels
    }
}
