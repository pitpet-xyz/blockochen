use blockochen::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test() {
    let mut chen = Blockochen::new();

    let mut mosquito_pet = b"".to_vec();
    let mut profile_data = "I love my mosquito pet.".as_bytes().to_vec();
    let birth_hash = chen.add(mosquito_pet, profile_data);

    assert_eq!(chen.get_ttl(&birth_hash), Some(TTL - 1));

    let event_hash = chen.add(
        birth_hash.clone(),
        "I brought my mosquito for a walk in the park"
            .as_bytes()
            .to_vec(),
    );

    assert_eq!(chen.get_ttl(&birth_hash), Some(TTL - 1));

    let mut cat_pet = b"".to_vec();
    let mut cat_profile_data = "I love my cat pet.".as_bytes().to_vec();
    let cat_birth_hash = chen.add(cat_pet, cat_profile_data);

    assert_eq!(chen.get_ttl(&birth_hash), Some(TTL - 2));
    assert_eq!(chen.get_ttl(&cat_birth_hash), Some(TTL - 1));
}
