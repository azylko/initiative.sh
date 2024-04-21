use crate::common::sync_app;
use initiative_core::app::AutocompleteSuggestion;

#[test]
fn fastmovement() {
    assert_eq!(
        "\
# Fast Movement

**Class:** Barbarian\\\n**Level:** 5

Starting at 5th level, your speed increases by 10 feet while you aren't wearing heavy armor.

*Fast Movement is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("Fast Movement").unwrap(),
    );

    assert_eq!(
        vec![AutocompleteSuggestion::new("Fast Movement", "SRD feature")],
        sync_app().autocomplete("fast movement"),
    );
}
