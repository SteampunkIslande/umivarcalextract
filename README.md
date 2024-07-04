# umivarcalextract: a valid replacement for UMI-VarCal extract

[UMI-VarCal](https://gitlab.com/vincent-sater/umi-varcal) is a MBC-aware variant caller written in python.

The caller itself relies on a BAM file with some specific formatting. Basically, each record's header should end with `_NNN-NNN` where `NNN` are the MBC tag for the read. Depending on the technology, there might be one or two MBCs per read.

# Installation

For now, you will need to have cargo installed.
I may publish it if people are interested, feel free to file an issue or open a pull request in this regard.

```bash
cargo install --git https://github.com/SteampunkIslande/umivarcalextract.git
```

# Usage and description

```
Simple program that writes MBC to the end of the read name, either from a predefined UMI length or from a tag

Usage: umivarcalextract [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input BAM file
  <OUTPUT>  Output BAM file

Options:
  -t, --tag <TAG>                Tag name to extract MBC from. Mutually exclusive with -l/--umi-length option
  -l, --umi-length <UMI_LENGTH>  Length of UMI to extract into read name. Mutually exclusive with -t/--tag option
  -h, --help                     Print help
  -V, --version                  Print version
```

As you can see, there are two ways of extracting UMI tags from an input BAM file, both are mutually exclusive.

You can use either:

- a tag name containing the MBC (if you use Agilent AGeNT trimmer followed by `bwa mem -C`, the MBC tag will be `RX`)
- the UMI length. Enter the number of nucleotides on each read to trim off that will be put at the end of each read header in the output file.

