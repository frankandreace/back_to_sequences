//! Back to sequences: find the origin of kmers

#![warn(missing_docs)]

/* std use */
use std::io::Write as _;

/* crates use */
use atomic_counter::{AtomicCounter, RelaxedCounter};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::Path;

/* mod declarations */
pub mod cli;
pub mod consts;
pub mod count;
pub mod kmer_hash;
pub mod sequence_normalizer;


/* project use */


/// Extract sequences that contain some kmers and
/// output the kmers that occur in the reads with their number of occurrences
pub fn back_to_sequences(
    in_fasta_reads: String,
    in_fasta_kmers: String,
    out_fasta_reads: String,
    out_txt_kmers: String,
    kmer_size: usize,
    counted_kmer_threshold: usize,
    min_threshold: f32,
    max_threshold: f32,
    stranded: bool,
    query_reverse: bool,
    no_low_complexity: bool,
) -> Result<(), ()> {
    // check that in_fasta_reads is a non empty file if it exists:
    if !in_fasta_reads.is_empty() {
        cli::validate_non_empty_file(in_fasta_reads.clone())?;
    }
    cli::validate_non_empty_file(in_fasta_kmers.clone())?;
    // check that in_fasta_kmers is a non empty file:

    let (kmer_set, kmer_size) =
        kmer_hash::index_kmers::<RelaxedCounter>(in_fasta_kmers, kmer_size, stranded, no_low_complexity)
            .map_err(|e| eprintln!("Error indexing kmers: {}", e))?;

    if out_fasta_reads.len() > 0 {

        count::kmers_in_fasta_file_par(
            in_fasta_reads,
            &kmer_set,
            kmer_size,
            out_fasta_reads.clone(),
            min_threshold,
            max_threshold,
            stranded,
            query_reverse,
        )?;
        println!(
            "Filtered sequences with exact kmer count are in file {}",
            out_fasta_reads
        );
    } else {
        println!("No output file provided, only the kmers with their count is output");
        count::only_kmers_in_fasta_file_par(
            in_fasta_reads,
            &kmer_set,
            kmer_size,
            stranded,
            query_reverse,
        );
    }
    // if the out_kmers_file is not empty, we output counted kmers in the out_kmers_file file
    if !out_txt_kmers.is_empty() {
        (|| -> std::io::Result<_> {
            // prints all kmers from kmer_set, whaterver their counts count
            let mut output = std::fs::File::create(&out_txt_kmers)?;
            for (kmer, count) in kmer_set.iter() {
                if count.get() >= counted_kmer_threshold {
                    output.write_all(kmer)?;
                    writeln!(output, " {}", count.get())?;
                }
            }
            Ok(())
        })()
        .map_err(|e| eprintln!("Error writing the kmers file: {}", e))?;

        println!(
            "kmers with their number of occurrences in the original sequences are in file {}",
            out_txt_kmers
        );
    }
    Ok(())
}

/// Extract sequences that contain some kmers and
/// output the kmers that occur in the reads with their number of occurrences
/// but instead of using a single input file takes a txt file with the path
/// of multiple files
pub fn back_to_multiple_sequences(
    in_fasta_filenames: String,
    in_fasta_kmers: String,
    out_fasta_filenames: String,
    out_txt_kmers: String,
    kmer_size: usize,
    counted_kmer_threshold: usize,
    min_threshold: f32,
    max_threshold: f32,
    stranded: bool,
    query_reverse: bool,
    no_low_complexity: bool,
) -> Result<(), ()> {
    // check that in_fasta_reads is a non empty file if it exists:
    if !in_fasta_filenames.is_empty() {
        cli::validate_non_empty_file(in_fasta_filenames.clone())?;
    }
    cli::validate_non_empty_file(in_fasta_kmers.clone())?;
    // check that in_fasta_kmers is a non empty file:

    let (kmer_set, kmer_size) =
        kmer_hash::index_kmers::<RelaxedCounter>(in_fasta_kmers, kmer_size, stranded, no_low_complexity)
            .map_err(|e| eprintln!("Error indexing kmers: {}", e))?;

    if out_fasta_filenames.len() > 0 {

        let input_files = read_file_lines(in_fasta_filenames.as_str())
        .map_err(|e| eprintln!("Error reading file: {}", e)).unwrap();
        let output_files = read_file_lines(out_fasta_filenames.as_str())
        .map_err(|e| eprintln!("Error reading file: {}", e)).unwrap();

        for (in_f, out_f) in input_files.iter().zip(output_files.iter()){
            count::kmers_in_fasta_file_par(
                in_f.to_string(),
                &kmer_set,
                kmer_size,
                out_f.clone().to_string(),
                min_threshold,
                max_threshold,
                stranded,
                query_reverse,
            )?;
            println!(
                "Filtered sequences with exact kmer count are in files specified at {}",
                out_fasta_filenames
            );
        }
    } else {
        eprintln!("No output file-list provided.");
    }
    // if the out_kmers_file is not empty, we output counted kmers in the out_kmers_file file
    if !out_txt_kmers.is_empty() {
        (|| -> std::io::Result<_> {
            // prints all kmers from kmer_set, whaterver their counts count
            let mut output = std::fs::File::create(&out_txt_kmers)?;
            for (kmer, count) in kmer_set.iter() {
                if count.get() >= counted_kmer_threshold {
                    output.write_all(kmer)?;
                    writeln!(output, " {}", count.get())?;
                }
            }
            Ok(())
        })()
        .map_err(|e| eprintln!("Error writing the kmers file: {}", e))?;

        println!(
            "kmers with their number of occurrences in the original sequences are in file {}",
            out_txt_kmers
        );
    }
    Ok(())
}

fn read_file_lines(file_path: &str) -> Result<Vec<String>, Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}