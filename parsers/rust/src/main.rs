use dnd_document_parser::models::common::*;
use dnd_document_parser::models::items::*;
use dnd_document_parser::models::spells::*;
use dnd_document_parser::parsers::spells::*;
use dnd_document_parser::traits::To5etools;

fn main() {
    let source = format!(
        "{}/resources/test/spells/gm_binder_input.html",
        env!("CARGO_MANIFEST_DIR")
    );
    parse_gm_binder(source.as_str());
    let s = Spell {
        source: Source {
            source_book: "PHB",
            page: 0,
        },
        name: "Blade of Disaster",
        level: 9,
        school: MagicSchool::Conjuration,
        casting_time: CastingTime {
            number: 1,
            unit: CastingTimeUnit::Action(ActionType::Action),
        },
        ritual: false,
        duration: Duration {
            number: 1,
            unit: DurationUnit::Time(TimeUnit::Minute),
            concentration: true,
        },
        range: Range::Ranged {
            type_: TargetType::Point,
            range: 60,
            unit: RangeUnit::Feet,
        },
        components: Components {
            verbal: true,
            somatic: false,
            material: Some(MaterialComponent {
                component: "Comp".into(),
                value: Some(ItemValue {
                    value: 10,
                    unit: Currency::Gold,
                }),
                consumed: false,
            }),
        },
        damage_type: Some(vec![DamageType::Force]),
        description: vec!["line1", "line2"],
        at_higher_levels: None,
        classes: vec![Classes::Wizard, Classes::Sorcerer],
    };
    println!("{}", s.to_5etools_spell());
}
