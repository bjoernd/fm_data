use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fm_data::attributes::{
    AttributeSet, GoalkeepingAttribute, MentalAttribute, PhysicalAttribute, TechnicalAttribute,
};
use fm_data::types::PlayerType;
use std::collections::HashMap;

fn create_large_attribute_dataset() -> Vec<AttributeSet> {
    let mut attributes = Vec::with_capacity(1000);

    for i in 0..1000 {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // Set all technical attributes
        attr_set.set_technical(TechnicalAttribute::Corners, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Crossing, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Dribbling, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Finishing, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::FirstTouch, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::FreeKickTaking, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Heading, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::LongShots, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::LongThrows, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Marking, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Passing, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::PenaltyTaking, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Tackling, (i % 20 + 1) as u8);
        attr_set.set_technical(TechnicalAttribute::Technique, (i % 20 + 1) as u8);

        // Set mental attributes
        attr_set.set_mental(MentalAttribute::Aggression, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Anticipation, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Bravery, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Composure, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Concentration, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Decisions, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Determination, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Flair, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Leadership, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::OffTheBall, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Positioning, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Teamwork, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::Vision, (i % 20 + 1) as u8);
        attr_set.set_mental(MentalAttribute::WorkRate, (i % 20 + 1) as u8);

        // Set physical attributes
        attr_set.set_physical(PhysicalAttribute::Acceleration, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::Agility, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::Balance, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::JumpingReach, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::NaturalFitness, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::Pace, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::Stamina, (i % 20 + 1) as u8);
        attr_set.set_physical(PhysicalAttribute::Strength, (i % 20 + 1) as u8);

        attributes.push(attr_set);
    }

    attributes
}

fn create_hashmap_based_attributes() -> Vec<HashMap<String, u8>> {
    let mut attributes = Vec::with_capacity(1000);

    for i in 0..1000 {
        let mut attr_map = HashMap::new();

        // Technical attributes
        attr_map.insert("Corners".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Crossing".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Dribbling".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Finishing".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("First Touch".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Free Kick Taking".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Heading".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Long Shots".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Long Throws".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Marking".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Passing".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Penalty Taking".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Tackling".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Technique".to_string(), (i % 20 + 1) as u8);

        // Mental attributes
        attr_map.insert("Aggression".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Anticipation".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Bravery".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Composure".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Concentration".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Decisions".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Determination".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Flair".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Leadership".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Off the Ball".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Positioning".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Teamwork".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Vision".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Work Rate".to_string(), (i % 20 + 1) as u8);

        // Physical attributes
        attr_map.insert("Acceleration".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Agility".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Balance".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Jumping Reach".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Natural Fitness".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Pace".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Stamina".to_string(), (i % 20 + 1) as u8);
        attr_map.insert("Strength".to_string(), (i % 20 + 1) as u8);

        attributes.push(attr_map);
    }

    attributes
}

fn bench_attribute_set_access(c: &mut Criterion) {
    let attributes = create_large_attribute_dataset();

    c.bench_function("attribute_set_technical_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get_technical(black_box(TechnicalAttribute::Passing)) as u32;
                sum += attr_set.get_technical(black_box(TechnicalAttribute::Finishing)) as u32;
                sum += attr_set.get_technical(black_box(TechnicalAttribute::Dribbling)) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("attribute_set_mental_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get_mental(black_box(MentalAttribute::Vision)) as u32;
                sum += attr_set.get_mental(black_box(MentalAttribute::Decisions)) as u32;
                sum += attr_set.get_mental(black_box(MentalAttribute::Composure)) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("attribute_set_physical_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get_physical(black_box(PhysicalAttribute::Pace)) as u32;
                sum += attr_set.get_physical(black_box(PhysicalAttribute::Acceleration)) as u32;
                sum += attr_set.get_physical(black_box(PhysicalAttribute::Stamina)) as u32;
            }
            black_box(sum)
        })
    });
}

fn bench_hashmap_access(c: &mut Criterion) {
    let attributes = create_hashmap_based_attributes();

    c.bench_function("hashmap_technical_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_map in &attributes {
                sum += *attr_map.get(black_box("Passing")).unwrap_or(&0) as u32;
                sum += *attr_map.get(black_box("Finishing")).unwrap_or(&0) as u32;
                sum += *attr_map.get(black_box("Dribbling")).unwrap_or(&0) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("hashmap_mental_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_map in &attributes {
                sum += *attr_map.get(black_box("Vision")).unwrap_or(&0) as u32;
                sum += *attr_map.get(black_box("Decisions")).unwrap_or(&0) as u32;
                sum += *attr_map.get(black_box("Composure")).unwrap_or(&0) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("hashmap_physical_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_map in &attributes {
                sum += *attr_map.get(black_box("Pace")).unwrap_or(&0) as u32;
                sum += *attr_map.get(black_box("Acceleration")).unwrap_or(&0) as u32;
                sum += *attr_map.get(black_box("Stamina")).unwrap_or(&0) as u32;
            }
            black_box(sum)
        })
    });
}

fn bench_conversion_overhead(c: &mut Criterion) {
    let attributes = create_large_attribute_dataset();

    c.bench_function("attribute_set_to_hashmap_conversion", |b| {
        b.iter(|| {
            let mut converted = Vec::new();
            for attr_set in &attributes {
                converted.push(black_box(attr_set.to_hashmap()));
            }
            black_box(converted)
        })
    });

    c.bench_function("hashmap_to_attribute_set_conversion", |b| {
        let hashmaps = create_hashmap_based_attributes();
        b.iter(|| {
            let mut converted = Vec::new();
            for attr_map in &hashmaps {
                let attr_set =
                    AttributeSet::from_hashmap(black_box(attr_map), &PlayerType::FieldPlayer);
                converted.push(attr_set);
            }
            black_box(converted)
        })
    });
}

fn bench_goalkeeper_attributes(c: &mut Criterion) {
    let mut gk_attributes = Vec::with_capacity(1000);

    for i in 0..1000 {
        let mut attr_set = AttributeSet::new(PlayerType::Goalkeeper);

        // Set goalkeeper-specific attributes
        attr_set.set_goalkeeping(GoalkeepingAttribute::AerialReach, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::CommandOfArea, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Communication, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Eccentricity, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::FirstTouch, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Handling, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Kicking, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::OneOnOnes, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Passing, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::PunchingTendency, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Reflexes, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::RushingOutTendency, (i % 20 + 1) as u8);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Throwing, (i % 20 + 1) as u8);

        gk_attributes.push(attr_set);
    }

    c.bench_function("goalkeeper_attribute_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &gk_attributes {
                sum += attr_set.get_goalkeeping(black_box(GoalkeepingAttribute::Reflexes)) as u32;
                sum += attr_set.get_goalkeeping(black_box(GoalkeepingAttribute::Handling)) as u32;
                sum += attr_set.get_goalkeeping(black_box(GoalkeepingAttribute::OneOnOnes)) as u32;
            }
            black_box(sum)
        })
    });
}

criterion_group!(
    benches,
    bench_attribute_set_access,
    bench_hashmap_access,
    bench_conversion_overhead,
    bench_goalkeeper_attributes
);
criterion_main!(benches);
