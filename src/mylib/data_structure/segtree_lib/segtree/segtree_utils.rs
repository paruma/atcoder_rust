use cargo_snippet::snippet;

/// セグメント木を根から葉の順に、区切り付きで標準エラーへ出力する。
///
/// `len` にはセグメント木の要素数を指定する。
#[snippet(include = "segtree_debug_string")]
pub fn segtree_debug<M: ac_library::Monoid>(seg: &ac_library::Segtree<M>, len: usize)
where
    M::S: std::fmt::Debug,
{
    eprint!("{}", segtree_debug_string(seg, len));
}

/// セグメント木を根から葉の順に、区切り付きの文字列にする。
///
/// `len` にはセグメント木の要素数を指定する。
///
/// 葉の数は2の冪に切り上げ、実データを含まない区間は `*` と表示する。
///
/// 値は `Debug` の整形表示を使い、構造体はフィールドごとに改行する。
/// インデントを保ち、表示ブロック全体を各区間の中央に配置する。
#[snippet]
pub fn segtree_debug_string<M: ac_library::Monoid>(
    seg: &ac_library::Segtree<M>,
    len: usize,
) -> String
where
    M::S: std::fmt::Debug,
{
    if len == 0 {
        return String::new();
    }
    #[derive(Clone, Debug)]
    struct DisplayBlock<'a> {
        lines: Vec<&'a str>,
        width: usize,
    }

    impl<'a> DisplayBlock<'a> {
        fn new(label: &'a str) -> Self {
            let lines: Vec<_> = label.lines().collect();
            let width = lines
                .iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0);
            Self { lines, width }
        }

        fn write_row(&self, output: &mut String, row: usize, interval_width: usize) {
            let line = self.lines.get(row).copied().unwrap_or("");
            let left_padding = (interval_width - self.width) / 2;
            let remaining_width = interval_width - left_padding;
            output.push_str(&" ".repeat(left_padding));
            output.push_str(&format!("{line:<remaining_width$}|"));
        }
    }

    let leaf_count = len.next_power_of_two();

    // 集約値と実データの有無から、各ノードの表示内容を決める。
    let labels: Vec<_> = {
        let mut node_values = vec![M::identity(); 2 * leaf_count];
        let mut has_value = vec![false; 2 * leaf_count];
        for i in 0..len {
            node_values[leaf_count + i] = seg.get(i);
            has_value[leaf_count + i] = true;
        }
        for i in (1..leaf_count).rev() {
            node_values[i] = M::binary_operation(&node_values[2 * i], &node_values[2 * i + 1]);
            has_value[i] = has_value[2 * i] || has_value[2 * i + 1];
        }
        node_values
            .iter()
            .zip(&has_value)
            .map(|(value, &present)| {
                if present {
                    format!("{value:#?}")
                } else {
                    "*".to_string()
                }
            })
            .collect()
    };

    // 行と幅をまとめて扱い、複数行のインデントを保つ。
    let blocks: Vec<_> = labels
        .iter()
        .map(|label| DisplayBlock::new(label))
        .collect();
    let leaf_cell_width = blocks[1..].iter().map(|block| block.width).max().unwrap() + 2;

    let mut output = String::new();
    let mut nodes_in_level = 1;
    while nodes_in_level <= leaf_count {
        let level_blocks = &blocks[nodes_in_level..2 * nodes_in_level];
        // 親の表示幅には、子の間にある区切りの幅も含める。
        let interval_width = (leaf_cell_width + 1) * (leaf_count / nodes_in_level) - 1;
        let level_height = level_blocks
            .iter()
            .map(|block| block.lines.len())
            .max()
            .unwrap();
        for row in 0..level_height {
            output.push('|');
            for block in level_blocks {
                block.write_row(&mut output, row, interval_width);
            }
            output.push('\n');
        }
        nodes_in_level *= 2;
    }
    output
}

#[snippet]
pub fn segtree_to_vec<M: ac_library::Monoid>(
    seg: &ac_library::Segtree<M>,
    len: usize,
) -> Vec<M::S> {
    (0..len).map(|i| seg.get(i)).collect()
}

// セグ木用の Monoidテンプレート
#[allow(unused_variables)]
#[snippet(prefix = "use monoid_template::*;")]
pub mod monoid_template {
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub len: usize,
    }

    impl RangeXxx {
        pub fn unit(x: i64) -> Self {
            Self { len: 1 }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;

        fn identity() -> Self::S {
            RangeXxx { len: 0 }
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx { len: a.len + b.len }
        }
    }
}

#[cfg(test)]
mod test_segtree_debug {
    use super::{segtree_debug, segtree_debug_string};
    use ac_library::{Additive, Segtree};

    #[test]
    fn test_tree() {
        let seg = Segtree::<Additive<i64>>::from(vec![1, 2, 3, 4]);
        assert_eq!(
            segtree_debug_string(&seg, 4),
            concat!(
                "|        10         |\n",
                "|    3    |    7    |\n",
                "| 1  | 2  | 3  | 4  |\n",
            )
        );
        segtree_debug(&seg, 4);
    }

    #[test]
    fn test_wide_integers_centered() {
        let seg = Segtree::<Additive<i64>>::from(vec![1, 100000000]);
        assert_eq!(
            segtree_debug_string(&seg, 2),
            concat!("|       100000001       |\n", "|     1     | 100000000 |\n",)
        );
    }

    #[test]
    fn test_empty_and_single() {
        let empty = Segtree::<Additive<i64>>::new(0);
        assert_eq!(segtree_debug_string(&empty, 0), "");
        let single = Segtree::<Additive<i64>>::from(vec![-12]);
        assert_eq!(segtree_debug_string(&single, 1), "| -12 |\n");
    }

    #[test]
    fn test_padding_and_update() {
        let mut seg = Segtree::<Additive<i64>>::from(vec![1, 2, 3]);
        seg.set(1, 5);
        assert_eq!(
            segtree_debug_string(&seg, 3),
            concat!(
                "|       9       |\n",
                "|   6   |   3   |\n",
                "| 1 | 5 | 3 | * |\n",
            )
        );
    }

    #[test]
    fn test_padding_and_real_identity() {
        let seg = Segtree::<Additive<i64>>::from(vec![0; 13]);
        let output = segtree_debug_string(&seg, 13);
        assert_eq!(
            output,
            concat!(
                "|                               0                               |\n",
                "|               0               |               0               |\n",
                "|       0       |       0       |       0       |       0       |\n",
                "|   0   |   0   |   0   |   0   |   0   |   0   |   0   |   *   |\n",
                "| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | * | * | * |\n",
            )
        );
    }

    #[test]
    fn test_struct_values() {
        #[derive(Clone, Debug)]
        struct Node {
            sum: i64,
            len: usize,
        }

        enum NodeMonoid {}

        impl ac_library::Monoid for NodeMonoid {
            type S = Node;

            fn identity() -> Node {
                Node { sum: 0, len: 0 }
            }

            fn binary_operation(a: &Node, b: &Node) -> Node {
                Node {
                    sum: a.sum + b.sum,
                    len: a.len + b.len,
                }
            }
        }

        let seg =
            Segtree::<NodeMonoid>::from([1, 2, 3, 4000].map(|sum| Node { sum, len: 1 }).to_vec());
        let output = segtree_debug_string(&seg, 4);
        assert_eq!(
            output,
            concat!(
                "|                          Node {                                   |\n",
                "|                              sum: 4006,                           |\n",
                "|                              len: 4,                              |\n",
                "|                          }                                        |\n",
                "|           Node {                |         Node {                  |\n",
                "|               sum: 3,           |             sum: 4003,          |\n",
                "|               len: 2,           |             len: 2,             |\n",
                "|           }                     |         }                       |\n",
                "|  Node {        |  Node {        |  Node {        | Node {         |\n",
                "|      sum: 1,   |      sum: 2,   |      sum: 3,   |     sum: 4000, |\n",
                "|      len: 1,   |      len: 1,   |      len: 1,   |     len: 1,    |\n",
                "|  }             |  }             |  }             | }              |\n",
            )
        );
        segtree_debug(&seg, 4);
    }
}

#[cfg(test)]
mod test_segtree_to_vec {
    use super::segtree_to_vec;
    use ac_library::{Additive, Segtree};

    #[test]
    fn test_segtree_to_vec() {
        let seg = Segtree::<Additive<i64>>::from(vec![1, 2, 3]);
        assert_eq!(segtree_to_vec(&seg, 3), vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod test_monoid_template {
    use super::monoid_template::*;
    use ac_library::Monoid;

    #[test]
    fn test_map_monoid_template() {
        let x1 = RangeXxx::unit(2);
        let x2 = RangeXxx::unit(3);

        assert_eq!(
            RangeXxxMonoid::binary_operation(&x1, &x2),
            RangeXxx { len: 2 }
        );
    }
}
