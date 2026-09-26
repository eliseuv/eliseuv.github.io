//! Spectral summaries of correlation matrix ensembles, shared by the live page and the
//! precomputed reference scans so both are reduced the same way.

use std::io::Read;

use artificial_systems::analysis::{Histogram, Moments};
use flate2::read::GzDecoder;
use ndarray::ArrayView2;
use serde::{Deserialize, Serialize};

/// Bins of the eigenvalue and correlation histograms.
pub const N_BINS: usize = 200;

/// Accumulated spectra of the correlation matrices `G*` at one parameter point.
///
/// Only var(λ) and λ_max are summarised: the mean eigenvalue is `Tr G* / n_samples = 1` for every
/// matrix of standardised series, so it carries no information.
#[derive(Debug, Clone, Default)]
pub struct SpectrumAccumulator {
    eigenvalues: Vec<f64>,
    max_eigenvalue: Moments,
    correlations: Option<Histogram>,
}

impl SpectrumAccumulator {
    /// Record the ascending `eigenvalues` of the correlation matrix `g`.
    pub fn push(&mut self, g: ArrayView2<f64>, eigenvalues: &[f64]) {
        self.eigenvalues.extend_from_slice(eigenvalues);
        self.max_eigenvalue
            .push(*eigenvalues.last().expect("non-empty spectrum"));
        let correlations = self
            .correlations
            .get_or_insert_with(|| Histogram::new(-1.0, 1.0, N_BINS));
        let n = g.nrows();
        for i in 0..n {
            for j in i + 1..n {
                correlations.add(g[[i, j]]);
            }
        }
    }

    /// Number of matrices recorded.
    pub fn n_matrices(&self) -> u64 {
        self.max_eigenvalue.count()
    }

    /// All recorded eigenvalues, matrix after matrix.
    pub fn eigenvalues(&self) -> &[f64] {
        &self.eigenvalues
    }

    /// Variance of all recorded eigenvalues, `(n_samples - 1)` times the mean squared
    /// off-diagonal correlation.
    pub fn eigenvalue_variance(&self) -> f64 {
        let moments: Moments = self.eigenvalues.iter().copied().collect();
        moments.variance(0)
    }

    /// Mean and variance of the largest eigenvalue over the recorded matrices.
    pub fn max_eigenvalue(&self) -> &Moments {
        &self.max_eigenvalue
    }

    /// Histogram of the off-diagonal correlations `g*_ij`, `i < j`.
    pub fn correlations(&self) -> Option<&Histogram> {
        self.correlations.as_ref()
    }

    /// Summary at control parameter value `parameter`.
    ///
    /// # Panics
    /// If nothing was recorded.
    pub fn summarize(&self, parameter: f64) -> ReferencePoint {
        ReferencePoint {
            parameter,
            n_matrices: self.n_matrices(),
            eigenvalue_variance: self.eigenvalue_variance(),
            max_eigenvalue_mean: self.max_eigenvalue.mean(),
            max_eigenvalue_variance: self.max_eigenvalue.variance(0),
            eigenvalues: Histogram::from_data(&self.eigenvalues, N_BINS),
            correlations: self.correlations.clone().expect("recorded spectra"),
        }
    }
}

/// Spectral summary at one value of the control parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencePoint {
    pub parameter: f64,
    pub n_matrices: u64,
    pub eigenvalue_variance: f64,
    pub max_eigenvalue_mean: f64,
    pub max_eigenvalue_variance: f64,
    pub eigenvalues: Histogram,
    pub correlations: Histogram,
}

/// Summaries along a line of the control parameter, all other parameters fixed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceScan {
    /// Model identifier, as used by the page.
    pub model: String,
    /// Name of the control parameter (e.g. `T/T_c`, `alpha`).
    pub parameter: String,
    /// Fixed parameters of the scan (e.g. lattice length, diffusion).
    pub fixed: Vec<(String, f64)>,
    pub n_samples: usize,
    /// Measurements per series after the initial one (series length `n_steps + 1`).
    pub n_steps: usize,
    pub seed: u64,
    pub points: Vec<ReferencePoint>,
}

/// Decode gzipped CBOR reference scans.
///
/// Plain CBOR is accepted too: servers that send `.gz` files with `Content-Encoding: gzip` have
/// the browser decompress them before the page sees the bytes.
pub fn decode(gzipped_cbor: &[u8]) -> Result<Vec<ReferenceScan>, String> {
    const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];
    let mut cbor = Vec::new();
    let bytes = if gzipped_cbor.starts_with(&GZIP_MAGIC) {
        GzDecoder::new(gzipped_cbor)
            .read_to_end(&mut cbor)
            .map_err(|e| e.to_string())?;
        cbor.as_slice()
    } else {
        gzipped_cbor
    };
    ciborium::from_reader(bytes).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::{write::GzEncoder, Compression};
    use ndarray::array;

    use super::*;

    #[test]
    fn decodes_gzipped_and_plain_cbor() {
        let g = array![[1.0, 0.5], [0.5, 1.0]];
        let mut accumulator = SpectrumAccumulator::default();
        accumulator.push(g.view(), &[0.5, 1.5]);
        let point = accumulator.summarize(1.0);
        assert_eq!(point.max_eigenvalue_mean, 1.5);
        approx::assert_relative_eq!(point.eigenvalue_variance, 0.25);
        let scans = vec![ReferenceScan {
            model: "test".into(),
            parameter: "x".into(),
            fixed: vec![("L".into(), 2.0)],
            n_samples: 2,
            n_steps: 1,
            seed: 0,
            points: vec![point],
        }];
        let mut cbor = Vec::new();
        ciborium::into_writer(&scans, &mut cbor).unwrap();
        let mut gzip = GzEncoder::new(Vec::new(), Compression::default());
        gzip.write_all(&cbor).unwrap();
        let gzipped = gzip.finish().unwrap();
        for bytes in [gzipped.as_slice(), cbor.as_slice()] {
            let decoded = decode(bytes).unwrap();
            assert_eq!(
                decoded[0].points[0]
                    .correlations
                    .counts()
                    .iter()
                    .sum::<u64>(),
                1
            );
        }
    }
}
