//! | Case                  | Tested by          |
//! |-----------------------|--------------------|
//! | class (no subclass)      | reckless-attack |
//! | class and subclass    | natural recover         |
//! | with prereq (spell)   | eldritch-invocation-agonizing-blast  |
//! | with prereq (feature)   | Eldritch Invocation: Book of Ancient Secrets  |
//! | with prereq (levle)   | ldritch Invocation: Sign of Ill Omen  |
//add the "feature_specific" - Meta magic

use initiative_reference::srd_5e::feautres;

#[test]
fn recklessattack() {
    let class_feature = features().unwrap();
    let class_feature = class_features
        .iter()
        .find(|i| i.name == "Reckless Attack")
        .unwrap();

    assert_eq!("`Reckless Attack`", class_feature.display_summary().to_string());

    assert_eq!(
        "\
# Reckless Attack

**Class:** Barbarian
**Level:** 2

Starting at 2nd level, you can throw aside all concern for defense to attack with fierce desperation. When you make your first attack on your turn, you can decide to attack recklessly. Doing so gives you advantage on melee weapon attack rolls using Strength during this turn, but attack rolls against you have advantage until your next turn.",
        class_feature.display_details().to_string(),
    );
}

// #[test]
// fn dwarven_resilience() {
//     let species_traits = traits().unwrap();
//     let species_trait = species_traits
//         .iter()
//         .find(|i| i.name == "Dwarven Resilience")
//         .unwrap();

//     assert_eq!(
//         "`Dwarven Resilience`",
//         species_trait.display_summary().to_string(),
//     );

//     assert_eq!(
//         "\
// # Dwarven Resilience

// **Species:** Dwarf

// You have advantage on saving throws against poison, and you have resistance against poison damage.",
//         species_trait.display_details().to_string(),
//     );
// }

// #[test]
// fn dwarven_toughness() {
//     let species_traits = traits().unwrap();
//     let species_trait = species_traits
//         .iter()
//         .find(|i| i.name == "Dwarven Toughness")
//         .unwrap();

//     assert_eq!(
//         "`Dwarven Toughness`",
//         species_trait.display_summary().to_string(),
//     );

//     assert_eq!(
//         "\
// # Dwarven Toughness

// **Subspecies:** Hill Dwarf

// Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.",
//         species_trait.display_details().to_string(),
//     );
// }
