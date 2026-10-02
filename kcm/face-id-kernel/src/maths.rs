pub fn norm(vec: &[f32]) -> f32 {
    vec.iter().map(|x| x.powi(2)).sum::<f32>().sqrt()
}
