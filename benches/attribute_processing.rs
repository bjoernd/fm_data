use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fm_data::attributes::{Attribute, PlayerAttributes};
use std::collections::HashMap;

fn create_large_attribute_dataset() -> Vec<PlayerAttributes> {
    let mut attributes = Vec::with_capacity(1000);

    for i in 0..1000 {
        let mut attr_set = PlayerAttributes::new();

        // Set all technical attributes
        attr_set.set(Attribute::Corners, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Crossing, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Dribbling, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Finishing, (i % 20 + 1) as u8);
        attr_set.set(Attribute::FirstTouch, (i % 20 + 1) as u8);
        attr_set.set(Attribute::FreeKickTaking, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Heading, (i % 20 + 1) as u8);
        attr_set.set(Attribute::LongShots, (i % 20 + 1) as u8);
        attr_set.set(Attribute::LongThrows, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Marking, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Passing, (i % 20 + 1) as u8);
        attr_set.set(Attribute::PenaltyTaking, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Tackling, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Technique, (i % 20 + 1) as u8);

        // Set mental attributes
        attr_set.set(Attribute::Aggression, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Anticipation, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Bravery, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Composure, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Concentration, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Decisions, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Determination, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Flair, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Leadership, (i % 20 + 1) as u8);
        attr_set.set(Attribute::OffTheBall, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Positioning, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Teamwork, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Vision, (i % 20 + 1) as u8);
        attr_set.set(Attribute::WorkRate, (i % 20 + 1) as u8);

        // Set physical attributes
        attr_set.set(Attribute::Acceleration, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Agility, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Balance, (i % 20 + 1) as u8);
        attr_set.set(Attribute::JumpingReach, (i % 20 + 1) as u8);
        attr_set.set(Attribute::NaturalFitness, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Pace, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Stamina, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Strength, (i % 20 + 1) as u8);

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

fn bench_unified_attribute_access(c: &mut Criterion) {
    let attributes = create_large_attribute_dataset();

    c.bench_function("unified_technical_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get(black_box(Attribute::Passing)) as u32;
                sum += attr_set.get(black_box(Attribute::Finishing)) as u32;
                sum += attr_set.get(black_box(Attribute::Dribbling)) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("unified_mental_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get(black_box(Attribute::Vision)) as u32;
                sum += attr_set.get(black_box(Attribute::Decisions)) as u32;
                sum += attr_set.get(black_box(Attribute::Composure)) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("unified_physical_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get(black_box(Attribute::Pace)) as u32;
                sum += attr_set.get(black_box(Attribute::Acceleration)) as u32;
                sum += attr_set.get(black_box(Attribute::Stamina)) as u32;
            }
            black_box(sum)
        })
    });

    c.bench_function("unified_name_based_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &attributes {
                sum += attr_set.get_by_name(black_box("Passing")).unwrap_or(0) as u32;
                sum += attr_set.get_by_name(black_box("Vision")).unwrap_or(0) as u32;
                sum += attr_set.get_by_name(black_box("Pace")).unwrap_or(0) as u32;
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

    c.bench_function("unified_to_hashmap_conversion", |b| {
        b.iter(|| {
            let mut converted = Vec::new();
            for attr_set in &attributes {
                converted.push(black_box(attr_set.to_hashmap()));
            }
            black_box(converted)
        })
    });

    c.bench_function("hashmap_to_unified_conversion", |b| {
        let hashmaps = create_hashmap_based_attributes();
        b.iter(|| {
            let mut converted = Vec::new();
            for attr_map in &hashmaps {
                let attr_set = PlayerAttributes::from_hashmap(black_box(attr_map));
                converted.push(attr_set);
            }
            black_box(converted)
        })
    });
}

fn bench_goalkeeper_attributes(c: &mut Criterion) {
    let mut gk_attributes = Vec::with_capacity(1000);

    for i in 0..1000 {
        let mut attr_set = PlayerAttributes::new();

        // Set goalkeeper-specific attributes using unified system
        attr_set.set(Attribute::AerialReach, (i % 20 + 1) as u8);
        attr_set.set(Attribute::CommandOfArea, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Communication, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Eccentricity, (i % 20 + 1) as u8);
        attr_set.set(Attribute::GoalkeepingFirstTouch, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Handling, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Kicking, (i % 20 + 1) as u8);
        attr_set.set(Attribute::OneOnOnes, (i % 20 + 1) as u8);
        attr_set.set(Attribute::GoalkeepingPassing, (i % 20 + 1) as u8);
        attr_set.set(Attribute::PunchingTendency, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Reflexes, (i % 20 + 1) as u8);
        attr_set.set(Attribute::RushingOutTendency, (i % 20 + 1) as u8);
        attr_set.set(Attribute::Throwing, (i % 20 + 1) as u8);

        gk_attributes.push(attr_set);
    }

    c.bench_function("unified_goalkeeper_attribute_access", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for attr_set in &gk_attributes {
                sum += attr_set.get(black_box(Attribute::Reflexes)) as u32;
                sum += attr_set.get(black_box(Attribute::Handling)) as u32;
                sum += attr_set.get(black_box(Attribute::OneOnOnes)) as u32;
            }
            black_box(sum)
        })
    });
}

criterion_group!(
    benches,
    bench_unified_attribute_access,
    bench_hashmap_access,
    bench_conversion_overhead,
    bench_goalkeeper_attributes
);
criterion_main!(benches);
