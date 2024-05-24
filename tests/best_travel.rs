use rust_katas::best_travel::choose_best_sum;

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

#[test]
fn basics_choose_best_sum() {
    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);
}

#[test]
#[ignore]
fn performance_test() {
    let ts = vec![
        931, 744, 763, 825, 713, 851, 867, 435, 726, 112, 847, 697, 547, 384, 897, 455, 242, 564, 873,
        586, 336, 934, 208, 557, 876, 644, 934, 753, 664, 725, 941, 754, 563, 972, 683, 636, 428, 943,
        653, 862, 929, 645, 251, 665, 120, 750, 420, 340, 931, 744, 763, 825, 713, 851, 867, 435, 726,
        112, 847, 697, 547, 384, 897, 455, 242, 564, 873, 586, 336, 934, 208, 557, 876, 644, 934, 753,
        664, 725, 941, 754, 563, 972, 683, 636, 428, 943, 653, 862, 929, 645, 251, 665, 120, 750, 420,
        340, 931, 744, 763, 825, 713, 851, 867, 435, 726, 112, 847, 697, 547, 384, 897, 455, 242, 564,
        873, 586, 336, 934, 208, 557, 876, 644, 934, 753, 664, 725, 941, 754, 563, 972, 683, 636, 428,
        943, 653, 862, 929, 645, 251, 665, 120, 750, 420, 340, 931, 744, 763,
    ];
    testing(3198, 5, &ts, 3198);
}
