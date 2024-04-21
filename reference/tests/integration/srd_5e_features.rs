//! | Case                  | Tested by          |
//! |-----------------------|--------------------|
//! | class (no subclass)   | reckless-attack    |
//! | class and subclass    | natural-recovery   |

use initiative_reference::srd_5e::features;

#[test]
fn recklessattack() {
    let class_features = features().unwrap();
    let class_feature = class_features
        .iter()
        .find(|i| i.name == "Reckless Attack")
        .unwrap();

    assert_eq!(
        "`Reckless Attack`",
        class_feature.display_summary().to_string()
    );

    assert_eq!(
        "\
# Reckless Attack

**Class:** Barbarian\\\n**Level:** 2

Starting at 2nd level, you can throw aside all concern for defense to attack with fierce desperation. When you make your first attack on your turn, you can decide to attack recklessly. Doing so gives you advantage on melee weapon attack rolls using Strength during this turn, but attack rolls against you have advantage until your next turn.",
        class_feature.display_details().to_string(),
    );
}

#[test]
fn naturalrecovery() {
    let class_features = features().unwrap();
    let class_feature = class_features
        .iter()
        .find(|i| i.name == "Natural Recovery")
        .unwrap();

    assert_eq!(
        "`Natural Recovery`",
        class_feature.display_summary().to_string()
    );

    assert_eq!(
        "\
# Natural Recovery

**Class:** Druid\\\n**Subclass:** Land\\\n**Level:** 2

Starting at 2nd level, you can regain some of your magical energy by sitting in meditation and communing with nature. During a short rest, you choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your druid level (rounded up), and none of the slots can be 6th level or higher. You can't use this feature again until you finish a long rest.

For example, when you are a 4th-level druid, you can recover up to two levels worth of spell slots. You can recover either a 2nd-level slot or two 1st-level slots.",
        class_feature.display_details().to_string(),
    );
}
