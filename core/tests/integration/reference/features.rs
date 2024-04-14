use crate::common::sync_app;
use initiative_core::app::AutocompleteSuggestion;

#[test]
fn fastmovement() {
    assert_eq!(
        "\
# Fast Movement

**Class:** Barbarian

Starting at 5th level, your speed increases by 10 feet while you aren't wearing heavy armor.

*Fast Movement is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("Stonecunning").unwrap(),
    );

    assert_eq!(
        vec![AutocompleteSuggestion::new("fast movement", "SRD feature")],
        sync_app().autocomplete("fast movement"),
    );
}

// #[test]
// fn darkvision() {
//     assert_eq!(
//         "\
// There are several possible interpretations of this command. Did you mean:

// * `srd spell Darkvision`
// * `srd trait Darkvision`",
//         sync_app().command("Darkvision").unwrap_err(),
//     );

//     assert_eq!(
//         "\
// # Darkvision

// **Species:** Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Tiefling

// You have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You cannot discern color in darkness, only shades of gray.

// *Darkvision is Open Game Content subject to the `Open Game License`.*",
//         sync_app().command("srd trait Darkvision").unwrap(),
//     );

//     assert_eq!(2, sync_app().autocomplete("darkvision").len());
//     assert!(sync_app()
//         .autocomplete("darkvision")
//         .iter()
//         .any(|suggestion| suggestion.term == "Darkvision" && suggestion.summary == "SRD trait"));
// }

// #[test]
// fn draconic_ancestry() {
//     assert_eq!(
//         vec![AutocompleteSuggestion::new(
//             "Draconic Ancestry",
//             "SRD trait",
//         )],
//         sync_app().autocomplete("draconic ancestry"),
//     );
// }
