use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

fn main() {
    let debug_abbrev = DebugAbbrev::new(&[
        10,
        39,
        0,
        10,
        11,
        1,
        22,
        0,
        0,
        0,
        0,
        0,
        255,
        255,
        255,
        255,
        255,
    ], LittleEndian);
    let debug_info = DebugInfo::new(&[
        10,
        0,
        0,
        0,
        4,
        0,
        0,
        0,
        0,
        0,
        101,
        10,
        255,
        33,
        0,
        1,
    ], LittleEndian);

    let mut units = debug_info.units();
    while let Ok(Some(unit)) = units.next() {
        if let Ok(abbrevs) = unit.abbreviations(&debug_abbrev) {
            let mut cursor = unit.entries(&abbrevs);
            while let Ok(Some((_delta, entry))) = cursor.next_dfs() {
                let mut attrs = entry.attrs();
                while let Ok(Some(_attr)) = attrs.next() {
                    continue;
                }
            }
        }
    }
}
