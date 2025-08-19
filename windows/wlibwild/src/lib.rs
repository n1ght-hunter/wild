use object::LittleEndian as LE;
use object::coff::CoffHeader;
use object::coff::SectionTable;
use object::coff::SymbolTable;
use object::pe::ImageFileHeader;
use std::io::Write;
use std::path::PathBuf;
use tracing::debug;
use tracing::info;
use tracing::warn;

pub fn run() {
    let files = vec![
        PathBuf::from("basic_objs/function.o"),
        PathBuf::from("basic_objs/main.o"),
    ];
    link_files(files).unwrap();
}

fn link_files(files: Vec<PathBuf>) -> anyhow::Result<()> {
    info!("Starting to link files: {:?}", files);
    for file in files {
        let debug_output = std::fs::File::create(file.with_extension("debug"))?;
        let mut writer = std::io::BufWriter::new(debug_output);
        let data = std::fs::read(file)?;
        parse_object(&mut writer, &data)?;
    }

    Ok(())
}

fn parse_object(debug_output: &mut impl Write, data: &[u8]) -> anyhow::Result<()> {
    let kind = object::FileKind::parse(data)?;

    match kind {
        object::FileKind::Coff => {
            debug!("COFF file detected");
            writeln!(debug_output, "COFF file detected")?;
            print_coff(debug_output, data)?;
        }
        object::FileKind::Pe32 => {
            info!("PE32 file detected");
        }
        object::FileKind::Pe64 => {
            info!("PE64 file detected");
        }
        kind => {
            warn!("Unsupported file kind: {:?}", kind);
            return Err(anyhow::anyhow!("Unsupported file kind: {:?}", kind));
        }
    }

    // If parsing is successful
    Ok(())
}

pub fn print_coff(debug_output: &mut impl Write, data: &[u8]) -> anyhow::Result<()> {
    let mut offset = 0;
    let header = ImageFileHeader::parse(data, &mut offset)?;

    let sections = header.sections(data, offset)?;

    let symbols = header.symbols(data);
    print_sections(
        debug_output,
        data,
        header.machine.get(LE),
        symbols.as_ref().ok(),
        &sections,
    )?;
    let symbols = symbols?;
    // print_symbols(p, sections.as_ref(), symbols);

    Ok(())
}

fn print_sections<'data, Coff: CoffHeader>(
    debug_output: &mut impl Write,
    data: &[u8],
    machine: u16,
    symbols: Option<&SymbolTable<'data, &'data [u8], Coff>>,
    sections: &SectionTable,
) -> anyhow::Result<()> {
    for (index, section) in sections.iter().enumerate() {
        if let Some(name) = symbols.and_then(|symbols| section.name(symbols.strings()).ok()) {
            writeln!(
                debug_output,
                "Section {}: {}",
                index,
                str::from_utf8(name).unwrap_or(&format!("{:x?}", name))
            )?;
        } else {
            writeln!(debug_output, "Section {}: {:?}", index, section.raw_name())?;
        }
        writeln!(
            debug_output,
            "  Virtual Size: {:#x}",
            section.virtual_size.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Virtual Address: {:#x}",
            section.virtual_address.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Size Of Raw Data: {:#x}",
            section.size_of_raw_data.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Pointer To Raw Data: {:#x}",
            section.pointer_to_raw_data.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Pointer To Relocations: {:#x}",
            section.pointer_to_relocations.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Pointer To Linenumbers: {:#x}",
            section.pointer_to_linenumbers.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Number Of Relocations: {}",
            section.number_of_relocations.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Number Of Linenumbers: {}",
            section.number_of_linenumbers.get(LE)
        )?;
        writeln!(
            debug_output,
            "  Characteristics: {:#x}",
            section.characteristics.get(LE)
        )?;
    }

    Ok(())
}
