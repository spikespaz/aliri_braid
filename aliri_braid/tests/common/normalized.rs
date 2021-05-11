use crate::{Normalized, NormalizedBuf};
use quickcheck_macros::quickcheck;
use static_assertions::{assert_eq_align, assert_eq_size, assert_eq_size_ptr, assert_eq_size_val};
use std::collections::HashSet;

#[test]
pub fn equality_tests() -> Result<(), Box<dyn std::error::Error>> {
    let x = NormalizedBuf::new("One Two")?;
    let y = &*Normalized::from_str("One Two")?;
    assert_eq!(x, y);
    assert_eq!(x, *y);
    assert_eq!(&x, y);
    assert_eq!(y, x);
    assert_eq!(y, &x);
    assert_eq!(*y, x);

    assert_eq!("OneTwo", x.clone().into_string());
    let z = x.clone().into_boxed_ref();
    assert_eq!(y, z);
    assert_eq!(z, y);
    assert_eq!(x, z);
    assert_eq!(z, x);

    assert_eq!(x, z.into_owned());

    Ok(())
}

#[test]
fn debug_and_display_tests() -> Result<(), Box<dyn std::error::Error>> {
    let x = NormalizedBuf::new("One Two")?;
    let y = Normalized::from_str("One Two")?;
    let z = Normalized::from_str("OneTwo")?;

    assert_eq!("OneTwo", x.to_string());
    assert_eq!("OneTwo", y.to_string());
    assert_eq!("OneTwo", z.to_string());
    assert_eq!("NormalizedBuf(\"OneTwo\")", format!("{:?}", x));
    assert_eq!("NormalizedBuf(\"OneTwo\")", format!("{:?}", y));
    assert_eq!("Normalized(\"OneTwo\")", format!("{:?}", z));

    Ok(())
}

#[quickcheck]
fn owned_and_borrowed_hashes_are_equivalent(s: String) -> quickcheck::TestResult {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let owned = if let Ok(x) = NormalizedBuf::new(s.clone()) {
        x
    } else {
        return quickcheck::TestResult::discard();
    };

    let owned_hash = {
        let mut hasher = DefaultHasher::new();
        owned.hash(&mut hasher);
        hasher.finish()
    };

    let borrowed = Normalized::from_str(&s).unwrap();

    let borrowed_hash = {
        let mut hasher = DefaultHasher::new();
        borrowed.hash(&mut hasher);
        hasher.finish()
    };

    if owned_hash == borrowed_hash {
        quickcheck::TestResult::passed()
    } else {
        quickcheck::TestResult::failed()
    }
}

#[test]
fn can_use_as_hash_keys() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashSet::new();

    assert!(map.insert(NormalizedBuf::new("One Two")?));
    assert!(map.insert(NormalizedBuf::new("SevenEight")?));

    assert!(map.contains(&*Normalized::from_str("One Two")?));
    assert!(map.contains(&NormalizedBuf::new("One Two")?));
    assert!(!map.contains(&*Normalized::from_str("Two Three")?));

    assert!(!map.remove(&*Normalized::from_str("Two Three")?));
    assert!(map.remove(&*Normalized::from_str("OneTwo")?));
    assert!(!map.remove(&*Normalized::from_str("One Two")?));

    assert!(map.remove(&NormalizedBuf::new("Seven Eight")?));
    assert!(!map.remove(&*Normalized::from_str("SevenEight")?));

    assert!(map.is_empty());

    Ok(())
}

#[test]
fn can_use_refs_as_hash_keys() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashSet::new();

    assert!(map.insert(Normalized::from_str("One Two")?));
    assert!(map.insert(Normalized::from_str("SevenEight")?));

    assert!(map.contains(&*Normalized::from_str("One Two")?));
    assert!(map.contains(&*NormalizedBuf::new("One Two")?));
    assert!(!map.contains(&*Normalized::from_str("Two Three")?));

    assert!(!map.remove(&*Normalized::from_str("Two Three")?));
    assert!(map.remove(&*Normalized::from_str("OneTwo")?));
    assert!(!map.remove(&*Normalized::from_str("One Two")?));

    assert!(map.remove(&*NormalizedBuf::new("Seven Eight")?));
    assert!(!map.remove(&*Normalized::from_str("SevenEight")?));

    assert!(map.is_empty());

    Ok(())
}

#[test]
#[should_panic]
fn verify_serialization_fail_borrow() {
    const SERIALIZATION: &str = "\"Test 🏗\"";
    dbg!(SERIALIZATION.as_bytes());
    let _: &Normalized = serde_json::from_str(&SERIALIZATION).unwrap();
}

#[test]
#[should_panic]
fn verify_serialization_fail_boxed() {
    const SERIALIZATION: &str = "\"Test 🏗\"";
    let _: Box<Normalized> = serde_json::from_str(&SERIALIZATION).unwrap();
}

#[test]
#[should_panic]
fn verify_serialization_fail_owned() {
    const SERIALIZATION: &str = "\"Test 🏗\"";
    let _: NormalizedBuf = serde_json::from_str(&SERIALIZATION).unwrap();
}

#[test]
#[should_panic]
fn verify_serialization_fail_borrow_valid_but_non_normal() {
    const SERIALIZATION: &str = "\"Test ;\"";
    dbg!(SERIALIZATION.as_bytes());
    let _: &Normalized = serde_json::from_str(&SERIALIZATION).unwrap();
}

#[test]
fn verify_serialization_pass_boxed_valid_but_non_normal() -> Result<(), Box<dyn std::error::Error>> {
    const SERIALIZATION: &str = "\"Test ;\"";
    let expected = &*Normalized::from_str("Test;")?;
    let actual: Box<Normalized> = serde_json::from_str(&SERIALIZATION)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn verify_serialization_pass_owned_valid_but_non_normal() -> Result<(), Box<dyn std::error::Error>> {
    const SERIALIZATION: &str = "\"Test ;\"";
    let expected = &*Normalized::from_str("Test;")?;
    let actual: NormalizedBuf = serde_json::from_str(&SERIALIZATION)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn verify_serialization_pass_borrow() -> Result<(), Box<dyn std::error::Error>> {
    const SERIALIZATION: &str = "\"Test;\"";
    let expected = &*Normalized::from_str("Test;")?;
    let actual: &Normalized = serde_json::from_str(&SERIALIZATION)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn verify_serialization_pass_boxed() -> Result<(), Box<dyn std::error::Error>> {
    const SERIALIZATION: &str = "\"Test;\"";
    let expected = &*Normalized::from_str("Test;")?;
    let actual: Box<Normalized> = serde_json::from_str(&SERIALIZATION)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn verify_serialization_pass_owned() -> Result<(), Box<dyn std::error::Error>> {
    const SERIALIZATION: &str = "\"Test;\"";
    let expected = &*Normalized::from_str("Test;")?;
    let actual: NormalizedBuf = serde_json::from_str(&SERIALIZATION)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn check_reference_alignment() {
    dbg!(std::mem::align_of::<&str>());
    dbg!(std::mem::align_of::<&Normalized>());
    assert_eq_align!(&Normalized, &str);
}

#[test]
fn check_reference_size() {
    dbg!(std::mem::size_of::<&str>());
    dbg!(std::mem::size_of::<&Normalized>());
    assert_eq_size!(&Normalized, &str);
}

#[test]
fn check_reference_size_ptr() {
    let s = "source";
    let y: &Normalized = &*Normalized::from_str(s).unwrap();
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_reference_size_ptr_normalized() {
    let s = "source five";
    let y: &Normalized = &*Normalized::from_str(s).unwrap();
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_reference_size_val() {
    let s = "source";
    let y: &Normalized = &*Normalized::from_str(s).unwrap();
    dbg!(std::mem::size_of_val(s));
    dbg!(std::mem::size_of_val(y));
    assert_eq_size_val!(s, y);
}

#[test]
fn check_reference_size_val_normalized() {
    let s = "source five";
    let y: &Normalized = &*Normalized::from_str(s).unwrap();
    dbg!(std::mem::size_of_val(s));
    dbg!(std::mem::size_of_val(y));
    assert_eq_size_val!(s, y);
}

#[test]
fn check_boxed_ref_alignment() {
    dbg!(std::mem::align_of::<Box<str>>());
    dbg!(std::mem::align_of::<Box<Normalized>>());
    assert_eq_align!(Box<Normalized>, Box<str>);
}

#[test]
fn check_boxed_ref_size() {
    dbg!(std::mem::size_of::<Box<str>>());
    dbg!(std::mem::size_of::<Box<Normalized>>());
    assert_eq_size!(Box<Normalized>, Box<str>);
}

#[test]
fn check_boxed_ref_size_ptr() {
    let source = String::from("source");
    let s = source.clone().into_boxed_str();
    let y: Box<Normalized> = NormalizedBuf::new(source).unwrap().into_boxed_ref();
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_boxed_ref_size_val() {
    let source = String::from("source");
    let s = source.clone().into_boxed_str();
    let y: Box<Normalized> = NormalizedBuf::new(source).unwrap().into_boxed_ref();
    dbg!(std::mem::size_of_val(&s));
    dbg!(std::mem::size_of_val(&y));
    assert_eq_size_val!(s, y);
}

#[test]
fn check_owned_alignment() {
    dbg!(std::mem::align_of::<String>());
    dbg!(std::mem::align_of::<NormalizedBuf>());
    assert_eq_align!(NormalizedBuf, String);
}

#[test]
fn check_owned_size() {
    dbg!(std::mem::size_of::<String>());
    dbg!(std::mem::size_of::<NormalizedBuf>());
    assert_eq_size!(NormalizedBuf, String);
}

#[test]
fn check_owned_size_ptr() {
    let s = String::from("source");
    let y: NormalizedBuf = NormalizedBuf::new(s.clone()).unwrap();
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_owned_size_val() {
    let s = String::from("source");
    let y: NormalizedBuf = NormalizedBuf::new(s.clone()).unwrap();
    dbg!(std::mem::size_of_val(&s));
    dbg!(std::mem::size_of_val(&y));
    assert_eq_size_val!(s, y);
}