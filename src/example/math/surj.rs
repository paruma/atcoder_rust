#![allow(dead_code)]

// [n_balls] → [n_boxes] の全射全列挙
// 残りのボールの数と残りの箱の数が同じになったら、ボールが入ってない箱にボールを入れるようにすれば良い？
// 必要になったら、一旦スターリング数全探索をして、箱の部分を K! 全部試す形で対応する
// fn surj(n_balls: usize, n_boxes: usize) -> Vec<Vec<usize>> {
//     todo!()
// }
