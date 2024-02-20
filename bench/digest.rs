// Copyright 2023 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ring::digest;

static ALGORITHMS: &[(&str, &digest::Algorithm)] = &[
    ("sha256", &digest::SHA256),
    ("sha384", &digest::SHA384),
    ("sha512", &digest::SHA512),
];

const INPUT_LENGTHS: &[usize] = &[
    // Benchmark that emphasizes overhead.
    0,
    31,
    48,
    63,
];

fn oneshot(c: &mut Criterion) {
    for &(alg_name, algorithm) in ALGORITHMS {
        for input_len in INPUT_LENGTHS {
            c.bench_with_input(
                BenchmarkId::new(format!("digest::oneshot::{alg_name}"), input_len),
                input_len,
                |b, &input_len| {
                    let input = vec![0u8; input_len];
                    b.iter(|| -> usize {
                        let digest = digest::digest(algorithm, black_box(&input));
                        black_box(digest.as_ref().len())
                    })
                },
            );
            c.bench_with_input(
                BenchmarkId::new(format!("digest::oneshot::{alg_name}_align"), input_len),
                input_len,
                |b, &input_len| {
                    let align_len = digest::aligned_len(algorithm, input_len);
                    let mut input = vec![0u8; align_len];
                    b.iter(|| -> usize {
                        let digest = digest::digest_aligned(algorithm, black_box(&mut input), black_box(input_len));
                        black_box(digest.as_ref().len())
                    })
                },
            );
        }
    }
}

criterion_group!(digest, oneshot);
criterion_main!(digest);
