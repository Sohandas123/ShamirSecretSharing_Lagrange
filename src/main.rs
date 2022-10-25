// I followed an example but from this point generalization is not so hard.
// Sohan Das, CrS 2119, Oct,24,2022, ISI Kokata
extern crate rand;
pub use num_bigint;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use rand::Rng;

fn main() {
    let n = 6; // Number of shares
    let k = 3; //Threshold
    let prime = 1631; // We have to choose prime > secret
    let secret = 1324;
    println!(
        " n is : {}, k is : {},\n prime is : {},\n secret is : {} \n",
        n, k, prime, secret
    );

    //---------------- Polynomial Creation Starts ---------------------
    let mut rng = rand::thread_rng();
    // f(x) = 1631 + a1*x + a2*x . Here coefficients are craeted for the function f(x).
    let mut coefficients = vec![secret];
    for _i in 1..k {
        let random_number = rng.gen_range(0..prime);
        coefficients.push(random_number);
    }

    print!("f(x) = {}", coefficients[0]);
    for i in 1..coefficients.len() {
        print!(" +{}*x^{} ", &coefficients[i], i);
    }
    println!("\n");
    // --------- ---- Polynomial Creation is Complete ---------------


    //--------------- Share Creation Start -----------------
    // n Shares S_1...S_n will be created by computing f(1)...f(n)
    // f(0) = secret , so we have to avoid it as a share
    let mut shares = vec![];
    for i in 1..(n+1) {
        let f_i = evaluate_at(i, &prime, &coefficients);
        shares.push((i, f_i));
    }
    for share in shares.iter() {
        println!("\n {}th Share is :({},{})", share.0, share.0, share.1);
    }
    //-------------- Share Creation is Complete --------------

    //--------- Recover secret from threshold number of ordered-pairs---------------
    let secret_value = recover(k, &[shares[2], shares[5], shares[4]], &prime);
    // Here we can change the above shares (here I have used share 1, share 2, and share 3)
    println!("\n Secret value is : {}\n", &secret_value);
}

//--------------------------------Function Definitions ----------------------------------------------

// This function evaluates f(x) at x with (mod prime) and returns that value
// where x = 1,2,....,n for n shares.
fn evaluate_at(x: u32, prime: &u32, polynomial: &Vec<u32>) -> u32 {
    let mut sum = 0;
    let x_value = u32::from(x);
    for coeffi in polynomial.iter().rev() {
        sum = (&x_value * sum + coeffi) % prime
    }
    return sum;
}

//This function will recover the secret value from theshold number
//of shares using Lagrange Interpolation
fn recover(threshold: u32, shares: &[(u32, u32)], p: &u32) -> BigInt {
    let thd = usize::try_from(threshold).unwrap();
    // checking whether the number of shares is greater than or equals to the number
    // of threshold value or not. If it fails then simply returns 0 as secret.
    // let len = filter_uniq(shares).len();  // trying to count unique elements in shares vector
    // println!("{}",len);
    if shares.len() < thd {
        println!("Number of shares {}, is less than threshold {}",shares.len(), threshold);
        return Zero::zero();
    }
    let (xs, f_xs): (Vec<u32>, Vec<u32>) = shares.iter().cloned().unzip();
    // println!("\np :{}\n",*p);
    let f_0 = lagrange_interpolation(0, xs, f_xs, *p);

    return f_0; // since secret = f(0) = 1234(here)
}

//Trying to count unique elements in a vector, k number of shares should be distict.
// fn filter_uniq(vec: Vec<u32>) -> Vec<String> {
//     vec.into_iter()
//         .map(|course| course.CRS_PREREQ)
//         .collect::<HashSet<_>>()
//         .into_iter()
//         .collect()
// }

//Lagrange Interpolation
fn lagrange_interpolation(x: u32, xs: Vec<u32>, ys: Vec<u32>, p: u32) -> BigInt {
    let len = xs.len();
    let xs_bigint: Vec<BigInt> = xs.iter().map(|q| BigInt::from(*q as i64)).collect();
    let mut sum: BigInt = Zero::zero();

    for i in 0..len {
        let mut neumerator: BigInt = One::one();
        for m in 0..len {
            if m == i {
                neumerator *= 1; //do nothing, since we dont want to multiply (xi - x)
            } else {
                neumerator = (neumerator * (&xs_bigint[m] - x)) % p; // product over m [(xm - x)], where m != i
            }
        }

        let mut denominator: BigInt = One::one();
        for m in 0..len {
            if m == i {
                denominator *= 1; // do nothing, since we dont want to multiply (xi - xi) which is 0
            } else {
                denominator = (denominator * (&xs_bigint[m] - &xs_bigint[i])) % p;
                // product over m [(xm-xi)] ,where m != i
            }
        }
        sum = sum + ys[i] * neumerator * inverse_mod_p(denominator, p);
    }
    sum = sum % p; //since our computation is on the field Zp, p is a prime
    if sum < Zero::zero() {
        sum = sum + p; // To avoid negative value as it belongs to Zp so both are equivalent
    }
    return sum;
}

// The following function returns the modulus inverse of an element in Zp
// Let q is in Zp. Then it finds some d in Zp such that q*d = 1 (mod p)
fn inverse_mod_p(denom: BigInt, p: u32) -> BigInt {
    let denom_modp = if denom < Zero::zero() {
        denom + p
    } else {
        denom
    };
    let inv = extend_euclid_algo(denom_modp, p);
    return inv;
}

// d is found by extended Euclid Algorithm as follows. This function actually returns d, the inverse of q in Zp
fn extend_euclid_algo(num: BigInt, p: u32) -> BigInt {
    let (mut r, mut next_r, mut s, mut next_s, mut t, mut next_t) = (
        BigInt::from(p).clone(),
        num.clone(),
        BigInt::from(1),
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(1),
    );
    let mut quotient;
    let mut tmp;
    while next_r > Zero::zero() {
        quotient = r.clone() / next_r.clone();
        tmp = next_r.clone();
        next_r = r.clone() - next_r.clone()*quotient.clone();
        r = tmp.clone();
        tmp = next_s.clone();
        next_s = s - next_s.clone()*quotient.clone();
        s = tmp;
        tmp = next_t.clone();
        next_t = t - next_t*quotient;
        t = tmp;
    }
    return t;
}
