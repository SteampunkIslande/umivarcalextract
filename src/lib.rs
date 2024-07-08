use error::UMIVarCalExtractError;
use noodles::bam;

use noodles::sam::alignment::io::Write;
use noodles::sam::alignment::record::Name;

pub mod error;

pub fn add_mbc_from_tag(
    tag_name: &[u8; 2],
    input_path: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<(), UMIVarCalExtractError> {
    let mut reader = bam::io::reader::Builder.build_from_path(input_path)?;
    let mut writer = bam::io::writer::Builder.build_from_path(output_path)?;
    let header = reader.read_header()?;

    writer.write_header(&header)?;

    for result in reader.record_bufs(&header) {
        let mut record = result?;
        let tag = record.data().get(tag_name);
        let mbc = match tag {
            Some(noodles::sam::alignment::record_buf::data::field::Value::Character(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::Int8(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::UInt8(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::Int16(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::UInt16(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::Int32(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::UInt32(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::Float(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::String(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::Hex(val)) => {
                format!("{val}")
            }
            Some(noodles::sam::alignment::record_buf::data::field::Value::Array(val)) => {
                format!("{val:?}")
            }
            None => String::from("NNN-NNN"),
        };
        *record.name_mut() = Some(noodles::sam::alignment::record_buf::Name::from(
            [record.name().unwrap().as_bytes(), &[b'_'], mbc.as_bytes()].concat(),
        ));
        writer.write_alignment_record(&header, &record)?;
    }
    Ok(())
}

pub fn add_mbc_from_umi_length(
    umi_length: usize,
    input_path: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<(), UMIVarCalExtractError> {
    let mut reader = bam::io::reader::Builder.build_from_path(input_path)?;
    let mut writer = bam::io::writer::Builder.build_from_path(output_path)?;
    let header = reader.read_header()?;

    writer.write_header(&header)?;
    for result in reader.record_bufs(&header) {
        let mut record = result?;
        let mbc = record
            .sequence()
            .as_ref()
            .get(..umi_length)
            .ok_or(error::UMIVarCalExtractError::UMIExtractionError(umi_length))?;
        *record.name_mut() = Some(noodles::sam::alignment::record_buf::Name::from(
            [record.name().unwrap().as_bytes(), &[b'_'], mbc].concat(),
        ));
        *record.sequence_mut() = record
            .sequence()
            .as_ref()
            .get(umi_length..)
            .ok_or(error::UMIVarCalExtractError::UMIExtractionError(umi_length))?
            .into();
        writer.write_alignment_record(&header, &record)?;
    }

    Ok(())
}
