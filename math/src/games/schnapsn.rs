// tuple: (color, value)
// colors: 
// heart: 0
// shell: 1
// leaf : 2
// acorn: 3 
//
// values: 
// 2 | 3 | 4 | 10 | 11
// u | o | k | 10 | ass
//
pub fn solve(color: u8, mut player: [(u8, u8, u8); 5], mut enemy: [(u8, u8, u8); 5]) -> Vec<[u8; 2]> {
    let mut sols = Vec::new();

    player.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    enemy.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    // ((0, 3), (0, 4)), ((0, 10), (0, 11)), ((1, 3), (1, 2)), ((3, 10), (1, 4)), 
    // herz ober - herz könig -> p2 +  7
    // herz 10   - herz ass   -> p1 + 21
    // shell o   - shell u    -> p1 +  5
    // acorn 10  - shell könig-> p1 + 14
    // ---------------------------------
    //                                47
    // [67, 0]

    for i in 0..player.len() {
        player[i].2 = 1;

        let found = duel(color, player[i], 1, [player, enemy], [20, 0], Vec::new());
        let has_lost = found.iter().find(|v| v.1[1] >= 66).is_some();

        if !has_lost {
            sols.extend_from_slice(&found);
        }

        player[i].2 = 0;
    }

    for sol in sols {
        println!("{sol:?}");
    }

    vec![]
}

fn duel(
    color: u8,
    (cur_color, value, _): (u8, u8, u8),

    current: usize,
    mut players: [[(u8, u8, u8); 5]; 2],
    mut points: [u8; 2],
    mut path: Vec<(((u8, u8), (u8, u8)))>,
) -> Vec<(Vec<Vec<(((u8, u8), (u8, u8)))>>, [u8; 2])> {
    if points[0] >= 65 || points[1] >= 65 {
        return vec![(vec![path], points)];
    }

    let mut sols = Vec::new();
    let mut len = players[current].len();

    let mut matches = Vec::new();
    let mut trumpfs = Vec::new();

    for i in 0..players[current].len() {
        if players[current][i].2 == 1 { len -= 1; continue; }

        if players[current][i].0 == cur_color {
            matches.push(i);
        } else if players[current][i].0 == color {
            trumpfs.push(i);
        }
    }

    if len == 0 {
        return vec![(vec![path], points)];
    }

    if !matches.is_empty() { trumpfs.clear(); }

    let mut nxt = Vec::new();
    
    for &match_idx in &matches {
        let winner = if players[current][match_idx].0 > value { current } else { 1 - current };
        let gain = (match_idx, winner, players[current][match_idx].1 + value);

        nxt.push(gain);
    }
    for &trmpf_idx in &trumpfs {
        nxt.push((trmpf_idx, current, players[current][trmpf_idx].1 + value))
    }
    if matches.is_empty() && trumpfs.is_empty() {
        for i in 0..players[current].len() {
            if players[current][i].2 != 1 {
                nxt.push((i, 1 - current, players[current][i].1 + value))
            }
        }
    }

    for (card, pidx, gain) in nxt {
        players[current][card].2 = 1;
        points[pidx] += gain;
        let mut cp = path.clone();
        cp.push(((cur_color, value), (players[current][card].0, players[current][card].1)));

        for i in 0..players[pidx].len() {
            if players[pidx][i].2 == 1 { continue; }
            players[pidx][i].2 = 1;

            let found = duel(color, players[pidx][i], 1 - pidx, players, points, cp.clone());
            let has_lost = found.iter().find(|v| v.1[1] >= 66).is_some();

            println!("{found:?}");
            if !has_lost {
                sols.extend_from_slice(&found);
            }

            players[pidx][i].2 = 0;
        }

        points[pidx] -= gain;
        players[current][card].2 = 0;
    }

    sols
}

fn play(
    tr: u8, 
    last: Option<(u8, u8)>,
    p1: &[(u8, u8)],
    p2: &[(u8, u8)],
    pts1: u8,
    pts2: u8,
) -> Vec<(u8, u8)> {
    if p1.is_empty() && p2.is_empty() {
        return vec![(pts1, pts2)];
    }

    let mut sols = Vec::new();

    if let Some((color, value)) = last {
        let turn = p1.len() > p2.len();
        let cur = if p1.len() > p2.len() { p1 } else { p2 };

        let mut mac = Vec::new();
        let mut trc = Vec::new();

        for i in 0..cur.len() {
            if cur[i].0 == color {
                mac.push(i);
            } else if cur[i].0 == tr {
                trc.push(i);
            }
        }

        if !mac.is_empty() {
            for idx in mac {
                let mut rm = Vec::new();
                rm.extend_from_slice(&cur[..idx]);
                rm.extend_from_slice(&cur[idx + 1..]);
                let gn = cur[idx].1 + value;

                if cur[idx].1 > value {
                    if turn {
                        sols.extend_from_slice(&(play(tr, Some(cur[idx]), &rm, p2, pts1 + gn, pts2)));
                    } else {
                        sols.extend_from_slice(&(play(tr, Some(cur[idx]), p1, &rm, pts1, pts2 + gn)));
                    }
                } else {
                    if turn {
                        sols.extend_from_slice(&(play(tr, Some(cur[idx]), &rm, p2, pts1, pts2 + gn)));
                    } else {
                        sols.extend_from_slice(&(play(tr, Some(cur[idx]), p1, &rm, pts1 + gn, pts2)));
                    }
                }
            }
        } else if !trc.is_empty() {
            for idx in trc {
                let mut rm = Vec::new();
                rm.extend_from_slice(&cur[..idx]);
                rm.extend_from_slice(&cur[idx + 1..]);
                let gn = cur[idx].1 + value;

                if turn {
                    sols.extend_from_slice(&(play(tr, Some(cur[idx]), &rm, p2, pts1 + gn, pts2)));
                } else {
                    sols.extend_from_slice(&(play(tr, Some(cur[idx]), p1, &rm, pts1, pts2 + gn)));
                }
            }
        } else {
            for i in 0..cur.len() {
                let mut rm = Vec::new();
                rm.extend_from_slice(&cur[..i]);
                rm.extend_from_slice(&cur[i + 1..]);
                let gn = cur[i].1 + value;

                if turn {
                    sols.extend_from_slice(&(play(tr, Some(cur[i]), &rm, p2, pts1, pts2 + gn)));
                } else {
                    sols.extend_from_slice(&(play(tr, Some(cur[i]), p1, &rm, pts1 + gn, pts2)));
                }
            }
        }
    } else {
        for i in 0..p1.len() {
            let mut rm = Vec::new();
            rm.extend_from_slice(&p1[..i]);
            rm.extend_from_slice(&p1[i + 1..]);

            sols.extend_from_slice(&(play(tr, Some(p1[i]), &rm, p2, pts1, pts2)));
        }
    }

    sols
}

//fn play(tr: u8, l: Option<(u8, u8)>, p1: &[(u8, u8)], p2: &[(u8, u8)], mut pts1: u8, mut pts2: u8) -> Vec<(u8, u8)> {
//    if pts1 >= 66 || pts2 >= 66 {
//        return vec![(pts1, pts2)];
//    }
//
//    let mut sols = Vec::new();
//
//    if let Some(last) = l {
//        for i in 0..p2.len() {
//
//        }
//    }
//
//    if p1.is_empty() {
//        return vec![(pts1, pts2)];
//    }
//
//    for i in 0..p1.len() {
//        if i + 1 < p1.len() && p1[i].0 == p1[i + 1].0 && p1[i].1 == 3 && p1[i + 1].1 == 4 {
//            let pts = if tr == p1[i].0 { 40 } else { 20 };
//
//            sols.extend_from_slice(&(play(tr, Some((p1[i])), &p1[1..], p2, pts1 + pts, pts2)));
//
//            let mr = Vec::new();
//            mr.extend_from_slice(&p1[..1]);
//            mr.extend_from_slice(&p1[2..]);
//            sols.extend_from_slice(&(play(tr, Some((p1[i + 1])), &mr, p2, pts1 + pts, pts2)));
//        }
//
//        sols.extend_from_slice(&(play(tr, Some((p1[i])), &p1[1..], p2, pts1, pts2)));
//    }
//
//    
//
//    vec![]
//}


