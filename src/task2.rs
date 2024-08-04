fn magnitude(s:&[f64]) -> f64 {
    let mut sum:f64=0.0;
    for i in 0..s.len(){
        sum=sum+s[i]*s[i];
    }
    let mag=sum.sqrt();
    return mag;
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(s: &mut[f64]) {
    let mag=magnitude(s);
    if mag!=0.0{
        for i in 0..s.len(){
        s[i]=s[i]/mag;
        }
    }
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
