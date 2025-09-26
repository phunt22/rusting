use algos::gale_shapley::{stable_matching, many_to_one_stable_matching};

#[test]
fn test_basic_stable_matching() {
    // basic test for compilation
    let h_prefs = vec![vec![0, 1], vec![1, 0]];
    let r_prefs = vec![vec![0, 1], vec![1, 0]];
    
    let _matching = stable_matching(&h_prefs, &r_prefs);

    assert_eq!(matching, Ok(vec![0, 1]));
    
    // TODO add tests
    // - errors on different input sizes
    // - perfect matching (implement is_stable)
    // - edge cases (empty inputs, zero capacity, etc.)
    // - different sizes, etc. grab examples from the slides
}

#[test]
fn test_many_to_one_matching() {
    // basic test for compilation, same as above
    let hospital_prefs = vec![vec![0, 1], vec![1, 0]];
    let resident_prefs = vec![vec![0, 1], vec![1, 0]];
    let hospital_capacities = vec![1, 1]; 

    let _matching = many_to_one_stable_matching(&hospital_prefs, &resident_prefs, &hospital_capacities);

    assert_eq!(matching, Ok(vec![Some(0), Some(1)]));
    
    // TODO add tests
    // - okay on diff input sizes
    // - diff capacities
    // - perfect matching (is_stable)
    // - edge cases (no capacity, residents vs. capacity, etc.)
    // - different sizes, etc. grab examples from the slides
}
