use crate::{Orange, OrangeRef};
use quickcheck_macros::quickcheck;
use static_assertions::{assert_eq_align, assert_eq_size, assert_eq_size_ptr, assert_eq_size_val};
use std::collections::HashSet;

#[test]
pub fn equality_tests() {
    let x = Orange::new("One");
    let y = OrangeRef::from_str("One");

    assert_eq!(x, y);
    assert_eq!(x, *y);
    assert_eq!(&x, y);
    assert_eq!(y, x);
    assert_eq!(y, &x);
    assert_eq!(*y, x);

    assert_eq!("One", x.clone().into_string());
    let z = x.clone().into_boxed_ref();
    assert_eq!(y, z);
    assert_eq!(z, y);
    assert_eq!(x, z);
    assert_eq!(z, x);

    assert_eq!(x, z.into_owned());
}

#[test]
pub fn debug_and_display_tests() {
    let x = Orange::new("One");
    let y = OrangeRef::from_str("One");

    assert_eq!("One", x.to_string());
    assert_eq!("One", y.to_string());
    assert_eq!("Orange(\"One\")", format!("{:?}", x));
    assert_eq!("OrangeRef(\"One\")", format!("{:?}", y));
}

#[quickcheck]
fn owned_and_borrowed_hashes_are_equivalent(s: String) -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let owned = Orange::new(s.clone());

    let owned_hash = {
        let mut hasher = DefaultHasher::new();
        owned.hash(&mut hasher);
        hasher.finish()
    };

    let borrowed = OrangeRef::from_str(&s);

    let borrowed_hash = {
        let mut hasher = DefaultHasher::new();
        borrowed.hash(&mut hasher);
        hasher.finish()
    };

    owned_hash == borrowed_hash
}

#[test]
fn can_use_as_hash_keys() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashSet::new();

    assert!(map.insert(Orange::new("One")));
    assert!(map.insert(Orange::new("Seven")));

    assert!(map.contains(OrangeRef::from_str("One")));
    assert!(map.contains(&Orange::new("One")));
    assert!(!map.contains(OrangeRef::from_str("Two")));

    assert!(!map.remove(OrangeRef::from_str("Two")));
    assert!(map.remove(OrangeRef::from_str("One")));
    assert!(!map.remove(OrangeRef::from_str("One")));

    assert!(map.remove(&Orange::new("Seven")));
    assert!(!map.remove(OrangeRef::from_str("Seven")));

    assert!(map.is_empty());

    Ok(())
}

#[test]
fn can_use_refs_as_hash_keys() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashSet::new();

    assert!(map.insert(OrangeRef::from_str("One")));
    assert!(map.insert(OrangeRef::from_str("Seven")));

    assert!(map.contains(OrangeRef::from_str("One")));
    assert!(map.contains(&*Orange::new("One")));
    assert!(!map.contains(OrangeRef::from_str("Two")));

    assert!(!map.remove(OrangeRef::from_str("Two")));
    assert!(map.remove(OrangeRef::from_str("One")));
    assert!(!map.remove(OrangeRef::from_str("One")));

    assert!(map.remove(&*Orange::new("Seven")));
    assert!(!map.remove(OrangeRef::from_str("Seven")));

    assert!(map.is_empty());

    Ok(())
}

#[test]
fn verify_serialization_non_validated() -> Result<(), Box<dyn std::error::Error>> {
    const SOURCE: &str = "Test 🏗";
    const EXPECTED_SERIALIZATION: &str = "\"Test 🏗\"";

    let start = Orange::new(SOURCE);

    let own_serialized = serde_json::to_string(&start)?;
    assert_eq!(EXPECTED_SERIALIZATION, own_serialized);
    let borrow: &OrangeRef = serde_json::from_str(&own_serialized)?;
    assert_eq!(start, borrow);
    let borrow_serialized = serde_json::to_string(borrow)?;
    assert_eq!(EXPECTED_SERIALIZATION, borrow_serialized);
    let boxed: Box<OrangeRef> = serde_json::from_str(&borrow_serialized)?;
    assert_eq!(borrow, boxed);
    let box_serialized = serde_json::to_string(&boxed)?;
    assert_eq!(EXPECTED_SERIALIZATION, box_serialized);
    let owned: Orange = serde_json::from_str(&box_serialized)?;
    assert_eq!(boxed, owned);

    assert_eq!(owned, start);
    Ok(())
}

#[test]
fn check_reference_alignment() {
    dbg!(std::mem::align_of::<&str>());
    dbg!(std::mem::align_of::<&OrangeRef>());
    assert_eq_align!(&OrangeRef, &str);
}

#[test]
fn check_reference_size() {
    dbg!(std::mem::size_of::<&str>());
    dbg!(std::mem::size_of::<&OrangeRef>());
    assert_eq_size!(&OrangeRef, &str);
}

#[test]
fn check_reference_size_ptr() {
    let s = "source";
    let y: &OrangeRef = OrangeRef::from_str(s);
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_reference_size_val() {
    let s = "source";
    let y: &OrangeRef = OrangeRef::from_str(s);
    dbg!(std::mem::size_of_val(s));
    dbg!(std::mem::size_of_val(y));
    assert_eq_size_val!(s, y);
}

#[test]
fn check_boxed_ref_alignment() {
    dbg!(std::mem::align_of::<Box<str>>());
    dbg!(std::mem::align_of::<Box<OrangeRef>>());
    assert_eq_align!(Box<OrangeRef>, Box<str>);
}

#[test]
fn check_boxed_ref_size() {
    dbg!(std::mem::size_of::<Box<str>>());
    dbg!(std::mem::size_of::<Box<OrangeRef>>());
    assert_eq_size!(Box<OrangeRef>, Box<str>);
}

#[test]
fn check_boxed_ref_size_ptr() {
    let source = String::from("source");
    let s = source.clone().into_boxed_str();
    let y: Box<OrangeRef> = Orange::new(source).into_boxed_ref();
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_boxed_ref_size_val() {
    let source = String::from("source");
    let s = source.clone().into_boxed_str();
    let y: Box<OrangeRef> = Orange::new(source).into_boxed_ref();
    dbg!(std::mem::size_of_val(&s));
    dbg!(std::mem::size_of_val(&y));
    assert_eq_size_val!(s, y);
}

#[test]
fn check_owned_alignment() {
    dbg!(std::mem::align_of::<String>());
    dbg!(std::mem::align_of::<Orange>());
    assert_eq_align!(Orange, String);
}

#[test]
fn check_owned_size() {
    dbg!(std::mem::size_of::<String>());
    dbg!(std::mem::size_of::<Orange>());
    assert_eq_size!(Orange, String);
}

#[test]
fn check_owned_size_ptr() {
    let s = String::from("source");
    let y: Orange = Orange::new(s.clone());
    assert_eq_size_ptr!(&s, &y);
}

#[test]
fn check_owned_size_val() {
    let s = String::from("source");
    let y: Orange = Orange::new(s.clone());
    dbg!(std::mem::size_of_val(&s));
    dbg!(std::mem::size_of_val(&y));
    assert_eq_size_val!(s, y);
}