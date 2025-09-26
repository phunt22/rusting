/// PART 1: Implement Gale-Shapley stable matching
/// 
/// # Arguments
/// * `h_prefs` - Each h_prefs[h] is hospital h's preference list (most --> least preferred)
/// * `r_prefs` - Each r_prefs[r] is resident r's preference list (most --> least preferred)
/// 
/// # Returns
/// * `match_for_hospital` - Where match_for_hospital[h] = r means hospital h is matched with resident r.
/// * The result should be a stable matching 
/// 
/// # Errors
/// * If the number of hospitals and residents are not the same
pub fn stable_matching(h_prefs: &[Vec<usize>], r_prefs: &[Vec<usize>]) -> Result<Vec<usize>, &str> {
    todo!("Your solution here!")
}







/// PART 2: Stable Matching Reduction
/// 
/// This problem asks you to solve many-to-one stable matching (hospitals with capacity)
/// by reducing it to one-to-one stable matching using the stable_matching function above.
/// 
/// Goal: Find a stable assignment where:
/// - Each resident assigned to at most one hospital
/// - Each hospital assigned at most s_i residents  
/// - No unstable pairs exist
///
/// # Arguments
/// * `hospital_prefs` - hospital_prefs[i] is hospital i's preference list over residents
/// * `resident_prefs` - resident_prefs[j] is resident j's preference list over hospitals  
/// * `hospital_capacities` - hospital_capacities[i] is the capacity of hospital i
/// 
/// # Returns
/// A vector where result[j] = Some(i) if resident j is matched to hospital i,
/// or result[j] = None if resident j is unmatched.
///
/// # Errors
/// * If the number of hospitals and residents are not the same
pub fn many_to_one_stable_matching(
    hospital_prefs: &[Vec<usize>],
    resident_prefs: &[Vec<usize>], 
    hospital_capacities: &[usize]
) -> Result<Vec<Option<usize>>, &str> {
    todo!("Your solution here!")
}