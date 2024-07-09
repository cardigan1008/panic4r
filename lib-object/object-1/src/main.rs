fn main() {
    let mut obj = object::write::Object::new(
        object::BinaryFormat::MachO,
        object::Architecture::X86_64,
        object::Endianness::Little,
    );

    let sect = obj.add_section(vec![], vec![], object::SectionKind::ReadOnlyDataWithRel);
    obj.append_section_data(sect, &vec![0u8; 32], 8);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::ReadOnlyData);
    obj.append_section_data(sect, &vec![0u8; 1], 32);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::Text);
    obj.append_section_data(sect, &vec![0u8; 1], 8);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::Debug);
    obj.append_section_data(sect, &vec![0u8; 1], 1);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::Debug);
    obj.append_section_data(sect, &vec![0u8; 1], 1);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::Debug);
    obj.append_section_data(sect, &vec![0u8; 1], 1);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::Debug);
    obj.append_section_data(sect, &vec![0u8; 1], 1);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::Debug);
    obj.append_section_data(sect, &vec![0u8; 1], 1);

    let sect = obj.add_section(vec![], vec![], object::SectionKind::ReadOnlyData);
    obj.append_section_data(sect, &vec![0u8; 1688], 8);

    obj.write().unwrap();
}