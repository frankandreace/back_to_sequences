//! Define Command Line Interface

/* std use */

/* crates use */
use clap::Parser;

/* project use */

/// Extract sequences that contain some kmers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input fasta or fastq [.gz] file containing the original sequences (eg. reads). 
    /// The stdin is used if not provided
    #[arg(long, default_value_t = String::from(""), verbatim_doc_comment)]
    pub in_sequences: String,

    /// Input txt file containing in each line a path to a fasta or fastq [.gz] file 
    /// containing the original sequences (eg. reads). 
    /// The stdin is used if not provided
    #[arg(long, default_value_t = String::from(""), verbatim_doc_comment)]
    pub in_filelist: String,

    /// Input fasta file containing the original kmers
    #[arg(long)]
    pub in_kmers: String,

    /// Output file containing the filtered original sequences (eg. reads). 
    /// It will be automatically in fasta or fastq format depending on the input file.
    /// If not provided, only the in_kmers with their count is output
    #[arg(long, default_value_t = String::from(""), verbatim_doc_comment)]
    pub out_sequences: String,

    /// Output txt file containing in each line a path to a fasta or fastq [.gz] file 
    /// that will contain the related output file from the input files list 
    /// The stdin is used if not provided
    #[arg(long, default_value_t = String::from(""), verbatim_doc_comment)]
    pub out_filelist: String,


    /// If provided, output a text file containing the kmers that occur in the reads with their number of occurrences
    #[arg(long, default_value_t = String::from(""))]
    pub out_kmers: String,
    
    /// If out_kmers is provided, output only reference kmers whose number of occurrences is at least equal to this value\n
    /// If out_kmers is not provided, this option is ignored
    #[arg(long, default_value_t = 0, verbatim_doc_comment)]
    pub counted_kmer_threshold: usize,

    /// Size of the kmers to index and search
    #[arg(short, long, default_value_t = 31)]
    pub kmer_size: usize,

    /// Output sequences are those whose ratio of indexed kmers is in ]min_threshold; max_threshold]
    /// Minimal threshold of the ratio  (%) of kmers that must be found in a sequence to keep it (default 0%).
    /// Thus by default, if no kmer is found in a sequence, it is not output.
    #[arg(short, long, default_value_t = 0.0, verbatim_doc_comment)]
    pub min_threshold: f32,

    /// Output sequences are those whose ratio of indexed kmers is in ]min_threshold; max_threshold]
    /// Maximal threshold of the ratio (%) of kmers that must be found in a sequence to keep it (default 100%).
    /// Thus by default, there is no limitation on the maximal number of kmers found in a sequence.
    #[arg(long, default_value_t = 100.0, verbatim_doc_comment)]
    pub max_threshold: f32,

    /// Used original kmer strand (else canonical kmers are considered)
    #[arg(long, default_value_t = false)]
    pub stranded: bool,

    /// Query the reverse complement of reads. Useless without the --stranded option
    #[arg(long, default_value_t = false)]
    pub query_reverse: bool,

    /// Do not index low complexity kmers (ie. with a Shannon entropy < 1.0)
    #[arg(long, default_value_t = false)]
    pub no_low_complexity: bool,
}

/// check that a file name corresponds to a non empty file:
pub fn validate_non_empty_file(in_file: String) -> Result<(), ()> {
    if let Ok(metadata) = std::fs::metadata(in_file.clone()) {
        // Check if the file exists
        if !metadata.is_file() {
            return Err(eprintln!("{:#} exists, but it's not a file.", in_file));
        }
    } else {
        return Err(eprintln!(
            "The {} file does not exist or there was an error checking its existence.",
            in_file
        ));
    }
    Ok(())
}
