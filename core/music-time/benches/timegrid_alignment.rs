use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use music_time::{GridConfig, Meter, TimeGrid, TimePoint};

fn bench_timegrid_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("timegrid_build");
    for &bars in &[32_u32, 128, 512, 2048] {
        group.throughput(Throughput::Elements(bars as u64));
        group.bench_with_input(BenchmarkId::from_parameter(bars), &bars, |b, &bars| {
            b.iter(|| {
                let grid = GridConfig::new(TimePoint::new(0.0), Meter::FOUR_FOUR)
                    .bars(bars)
                    .subdivisions_per_beat(4)
                    .build();
                black_box(grid);
            });
        });
    }
    group.finish();
}

fn bench_event_alignment(c: &mut Criterion) {
    let fixtures =
        [1024_usize, 4096, 16384].into_iter().map(AlignmentFixture::new).collect::<Vec<_>>();

    let mut group = c.benchmark_group("event_alignment");
    for fixture in &fixtures {
        let len = fixture.events.len() as u64;
        group.throughput(Throughput::Elements(len));
        group.bench_with_input(BenchmarkId::from_parameter(len), fixture, |b, fixture| {
            b.iter(|| {
                black_box(align_events_to_grid(&fixture.grid, &fixture.events));
            });
        });
    }
    group.finish();
}

struct AlignmentFixture {
    grid: TimeGrid,
    events: Vec<TimePoint>,
}

impl AlignmentFixture {
    fn new(event_count: usize) -> Self {
        let events = generate_events(event_count);
        let meter = Meter::FOUR_FOUR;
        let bars = bars_needed(&events, meter);
        let grid =
            GridConfig::new(TimePoint::new(0.0), meter).bars(bars).subdivisions_per_beat(4).build();
        Self { grid, events }
    }
}

fn align_events_to_grid(grid: &TimeGrid, events: &[TimePoint]) -> u64 {
    let subdivisions = grid.subdivisions();
    events.iter().fold(0_u64, |acc, event| {
        let idx = nearest_subdivision_index(subdivisions, *event) as u64;
        acc + idx
    })
}

fn nearest_subdivision_index(points: &[TimePoint], target: TimePoint) -> usize {
    match points.binary_search_by(|probe| probe.partial_cmp(&target).unwrap()) {
        Ok(idx) => idx,
        Err(idx) => {
            if idx == 0 {
                0
            } else if idx >= points.len() {
                points.len() - 1
            } else {
                let prev = points[idx - 1];
                let next = points[idx];
                let prev_dist = prev.distance_to(target).as_beats();
                let next_dist = next.distance_to(target).as_beats();
                if next_dist < prev_dist { idx } else { idx - 1 }
            }
        }
    }
}

fn generate_events(count: usize) -> Vec<TimePoint> {
    (0..count)
        .map(|idx| {
            let base = idx as f64 * 0.5;
            let jitter = (idx % 5) as f64 * 0.025;
            TimePoint::new(base + jitter)
        })
        .collect()
}

fn bars_needed(events: &[TimePoint], meter: Meter) -> u32 {
    if events.is_empty() {
        return 1;
    }
    let last = events.last().copied().unwrap();
    let beats_needed = last.as_beats() + 8.0; // buffer for alignment lookups
    let bars = (beats_needed / meter.beats_per_bar()).ceil() as u32;
    bars.max(1)
}

criterion_group!(timegrid_benches, bench_timegrid_build, bench_event_alignment);
criterion_main!(timegrid_benches);
