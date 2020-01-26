#![deny(warnings)]

#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate macro_attr;
#[macro_use]
extern crate bitflags;

use either::{Either};
use std::fmt::{self, Display, Debug};
//use std::str::{FromStr};

mod tag;

pub use tag::*;

include!(concat!(env!("OUT_DIR"), "/tags.rs"));

macro_attr! {
    #[derive(Primitive)]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
    #[derive(Debug, EnumDisplay!, EnumFromStr!)]
    #[repr(u32)]
    pub enum FileType {
        ESP = 0,
        ESM = 1,
        ESS = 32
    }
}

macro_attr! {
    #[derive(Primitive)]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
    #[derive(Debug, EnumDisplay!, EnumFromStr!)]
    #[repr(u8)]
    pub enum DialogType {
        Topic = 0,
        Voice = 1,
        Greeting = 2,
        Persuasion = 3,
        Journal = 4
    }
}

macro_attr! {
    #[derive(Primitive)]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
    #[derive(Debug, EnumDisplay!, EnumFromStr!)]
    #[repr(i32)]
    pub enum EffectRange {
        Oneself = 0,
        Touch = 1,
        Target = 2,
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum StringCoerce {
    None,
    CutTailZeros,
    CutTailZerosExceptOne,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum LinebreakStyle {
    Unix,
    Dos
}

#[derive(Copy, Clone, Debug)]
pub enum FieldType {
    Binary,
    String(StringCoerce),
    Multiline(LinebreakStyle, StringCoerce),
    MultiString,
    Reference,
    FixedString(u32),
    Float,
    Int,
    Short,
    Long,
    Byte,
    Compressed,
    Ingredient,
    Script,
    Dialog,
    None,
    Header,
    Npc,
    SavedNpc,
    Effect,
}

impl FieldType {
    pub fn from_tags(record_tag: Tag, field_tag: Tag) -> FieldType {
        match (record_tag, field_tag) {
            (INFO, ACDT) => FieldType::String(StringCoerce::None),
            (CELL, ACTN) => FieldType::Int,
            (NPC_, ANAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (_, ANAM) => FieldType::String(StringCoerce::None),
            (_, ASND) => FieldType::String(StringCoerce::None),
            (_, AVFX) => FieldType::String(StringCoerce::None),
            (ARMO, BNAM) => FieldType::String(StringCoerce::CutTailZeros),
            (BODY, BNAM) => FieldType::String(StringCoerce::CutTailZeros),
            (CELL, BNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (CLOT, BNAM) => FieldType::String(StringCoerce::CutTailZeros),
            (CONT, BNAM) => FieldType::Multiline(LinebreakStyle::Dos, StringCoerce::CutTailZeros),
            (INFO, BNAM) => FieldType::Multiline(LinebreakStyle::Dos, StringCoerce::CutTailZeros),
            (NPC_, BNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (PCDT, BNAM) => FieldType::String(StringCoerce::None),
            (REGN, BNAM) => FieldType::String(StringCoerce::None),
            (_, BNAM) => FieldType::Multiline(LinebreakStyle::Dos, StringCoerce::None),
            (_, BSND) => FieldType::String(StringCoerce::None),
            (_, BVFX) => FieldType::String(StringCoerce::None),
            (ARMO, CNAM) => FieldType::String(StringCoerce::CutTailZeros),
            (KLST, CNAM) => FieldType::Int,
            (NPC_, CNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (REGN, CNAM) => FieldType::Int,
            (_, CNAM) => FieldType::String(StringCoerce::None),
            (_, CSND) => FieldType::String(StringCoerce::None),
            (_, CVFX) => FieldType::String(StringCoerce::None),
            (DIAL, DATA) => FieldType::Dialog,
            (LAND, DATA) => FieldType::Int,
            (LEVC, DATA) => FieldType::Int,
            (LEVI, DATA) => FieldType::Int,
            (LTEX, DATA) => FieldType::String(StringCoerce::None),
            (SSCR, DATA) => FieldType::String(StringCoerce::CutTailZeros),
            (TES3, DATA) => FieldType::Long,
            (QUES, DATA) => FieldType::String(StringCoerce::None),
            (DIAL, DELE) => FieldType::None,
            (_, DESC) => FieldType::String(StringCoerce::None),
            (_, DNAM) => FieldType::String(StringCoerce::None),
            (ALCH, ENAM) => FieldType::Effect,
            (ARMO, ENAM) => FieldType::String(StringCoerce::None),
            (ENCH, ENAM) => FieldType::Effect,
            (PCDT, ENAM) => FieldType::Long,
            (SPEL, ENAM) => FieldType::Effect,
            (CELL, FGTN) => FieldType::String(StringCoerce::None),
            (_, FLAG) => FieldType::Int,
            (_, FLTV) => FieldType::Float,
            (ACTI, FNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (PCDT, FNAM) => FieldType::Binary,
            (RACE, FNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (_, FNAM) => FieldType::String(StringCoerce::None),
            (CELL, FRMR) => FieldType::Int,
            (TES3, HEDR) => FieldType::Header,
            (_, HSND) => FieldType::String(StringCoerce::None),
            (_, HVFX) => FieldType::String(StringCoerce::None),
            (_, INAM) => FieldType::String(StringCoerce::None),
            (ARMO, INDX) => FieldType::Byte,
            (CLOT, INDX) => FieldType::Byte,
            (_, INDX) => FieldType::Int,
            (LAND, INTV) => FieldType::Long,
            (LEVC, INTV) => FieldType::Short,
            (LEVI, INTV) => FieldType::Short,
            (_, INTV) => FieldType::Int,
            (INGR, IRDT) => FieldType::Ingredient,
            (_, ITEX) => FieldType::String(StringCoerce::None),
            (NPC_, KNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (PCDT, KNAM) => FieldType::Binary,
            (_, KNAM) => FieldType::String(StringCoerce::None),
            (PCDT, LNAM) => FieldType::Long,
            (CELL, LSHN) => FieldType::String(StringCoerce::None),
            (CELL, LSTN) => FieldType::String(StringCoerce::None),
            (_, LVCR) => FieldType::Byte,
            (FMAP, MAPD) => FieldType::Compressed,
            (FMAP, MAPH) => FieldType::Long,
            (TES3, MAST) => FieldType::String(StringCoerce::None),
            (PCDT, MNAM) => FieldType::String(StringCoerce::None),
            (CELL, MNAM) => FieldType::Byte,
            (LIGH, MODL) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (_, MODL) => FieldType::String(StringCoerce::None),
            (CELL, NAM0) => FieldType::Int,
            (SPLM, NAM0) => FieldType::Byte,
            (CELL, NAM5) => FieldType::Int,
            (CELL, NAM9) => FieldType::Int,
            (PCDT, NAM9) => FieldType::Int,
            (CELL, NAME) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (JOUR, NAME) => FieldType::Multiline(LinebreakStyle::Unix, StringCoerce::None),
            (SPLM, NAME) => FieldType::Int,
            (SSCR, NAME) => FieldType::String(StringCoerce::CutTailZeros),
            (_, NAME) => FieldType::String(StringCoerce::None),
            (_, ND3D) => FieldType::Byte,
            (INFO, NNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (LEVC, NNAM) => FieldType::Byte,
            (LEVI, NNAM) => FieldType::Byte,
            (_, NNAM) => FieldType::String(StringCoerce::None),
            (_, NPCO) => FieldType::Reference,
            (NPC_, NPDT) => FieldType::Npc,
            (NPCC, NPDT) => FieldType::SavedNpc,
            (BSGN, NPCS) => FieldType::FixedString(32),
            (NPC_, NPCS) => FieldType::FixedString(32),
            (RACE, NPCS) => FieldType::FixedString(32),
            (_, NPCS) => FieldType::String(StringCoerce::None),
            (_, ONAM) => FieldType::String(StringCoerce::None),
            (INFO, PNAM) => FieldType::String(StringCoerce::CutTailZerosExceptOne),
            (PCDT, PNAM) => FieldType::Binary,
            (_, PNAM) => FieldType::String(StringCoerce::None),
            (_, PTEX) => FieldType::String(StringCoerce::None),
            (_, RGNN) => FieldType::String(StringCoerce::None),
            (FACT, RNAM) => FieldType::FixedString(32),
            (_, RNAM) => FieldType::String(StringCoerce::None),
            (SCPT, SCHD) => FieldType::Script,
            (_, SCRI) => FieldType::String(StringCoerce::None),
            (_, SCTX) => FieldType::Multiline(LinebreakStyle::Dos, StringCoerce::CutTailZeros),
            (SCPT, SCVR) => FieldType::MultiString,
            (_, SCVR) => FieldType::String(StringCoerce::None),
            (CELL, SLSD) => FieldType::Binary,
            (PCDT, SNAM) => FieldType::Binary,
            (REGN, SNAM) => FieldType::Binary,
            (_, SNAM) => FieldType::String(StringCoerce::None),
            (_, STRV) => FieldType::String(StringCoerce::None),
            (ALCH, TEXT) => FieldType::String(StringCoerce::None),
            (BOOK, TEXT) => FieldType::Multiline(LinebreakStyle::Dos, StringCoerce::CutTailZeros),
            (_, TEXT) => FieldType::Multiline(LinebreakStyle::Dos, StringCoerce::None),
            (_, TNAM) => FieldType::String(StringCoerce::None),
            (_, VCLR) => FieldType::Compressed,
            (_, VHGT) => FieldType::Compressed,
            (_, VNML) => FieldType::Compressed,
            (_, VTEX) => FieldType::Compressed,
            (_, WEAT) => FieldType::Binary,
            (CELL, WHGT) => FieldType::Int,
            (_, WIDX) => FieldType::Long,
            (_, WNAM) => FieldType::Compressed,
            (_, XCHG) => FieldType::Int,
            (_, XHLT) => FieldType::Int,
            (_, XIDX) => FieldType::Int,
            (_, XSOL) => FieldType::String(StringCoerce::None),
            (SPLM, XNAM) => FieldType::Byte,
            (CELL, XSCL) => FieldType::Int,
            (CELL, ZNAM) => FieldType::Byte,
            _ => FieldType::Binary
        }
    }
}

#[derive(Debug)]
pub struct Ingredient {
    pub weight: f32,
    pub value: u32,
    pub effects: [i32; 4],
    pub skills: [i32; 4],
    pub attributes: [i32; 4]
}

#[derive(Debug)]
pub struct ScriptMetadata {
    pub name: String,
    pub shorts: u32,
    pub longs: u32,
    pub floats: u32,
    pub data_size: u32,
    pub var_table_size: u32
}

#[derive(Debug)]
pub struct FileMetadata {
    pub version: u32,
    pub file_type: FileType,
    pub author: String,
    pub description: Vec<String>
}

#[derive(Debug)]
pub struct Effect {
    pub id: i16,
    pub skill: i8,
    pub attribute: i8,
    pub range: EffectRange,
    pub area: i32,
    pub duration: i32,
    pub magnitude_min: i32,
    pub magnitude_max: i32
}

#[derive(Debug)]
pub struct SavedNpc {
    pub disposition: i16,
    pub reputation: i16,
    pub index: u32,
}

#[derive(Debug)]
pub struct NpcCharacteristics {
    pub strength: u8,
    pub intelligence: u8,
    pub willpower: u8,
    pub agility: u8,
    pub speed: u8,
    pub endurance: u8,
    pub personality: u8,
    pub luck: u8,
    pub block: u8,
    pub armorer: u8,
    pub medium_armor: u8,
    pub heavy_armor: u8,
    pub blunt_weapon: u8,
    pub long_blade: u8,
    pub axe: u8,
    pub spear: u8,
    pub athletics: u8,
    pub enchant: u8,
    pub destruction: u8,
    pub alteration: u8,
    pub illusion: u8,
    pub conjuration: u8,
    pub mysticism: u8,
    pub restoration: u8,
    pub alchemy: u8,
    pub unarmored: u8,
    pub security: u8,
    pub sneak: u8,
    pub acrobatics: u8,
    pub light_armor: u8,
    pub short_blade: u8,
    pub marksman: u8,
    pub mercantile: u8,
    pub speechcraft: u8,
    pub hand_to_hand: u8,
    pub faction: u8,
    pub health: i16,
    pub magicka: i16,
    pub fatigue: i16
}

#[derive(Debug)]
pub struct Npc {
    pub level: u16,
    pub disposition: i8,
    pub reputation: i8,
    pub rank: i8,
    pub gold: i32,
    pub characteristics: Either<u32, NpcCharacteristics>
}

#[derive(Debug)]
pub enum Field {
    Binary(Vec<u8>),
    String(String),
    Multiline(Vec<String>),
    MultiString(Vec<String>),
    Reference(i32, String),
    Float(Either<u32, f32>),
    Int(i32),
    Short(i16),
    Long(i64),
    Byte(u8),
    Compressed(Vec<u8>),
    Ingredient(Ingredient),
    ScriptMetadata(ScriptMetadata),
    DialogMetadata(Either<u32, DialogType>),
    None,
    FileMetadata(FileMetadata),
    SavedNpc(SavedNpc),
    Npc(Npc),
    Effect(Effect),
}

bitflags! {
    pub struct RecordFlags: u64 {
        const PERSISTENT = 0x40000000000;
        const BLOCKED = 0x200000000000;
        const DELETED = 0x2000000000;
    }
}

impl Display for RecordFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/*impl FromStr for RecordFlags {
    type Err = ();
    
    fn from_str(s: &str) -> Result<RecordFlags, Self::Err> {
        
    }
}*/

#[cfg(test)]
mod tests {
    use crate::*;
    use num_traits::cast::FromPrimitive;
    use std::str::FromStr;

    #[test]
    fn debug_and_display_tag() {
        assert_eq!("TES3", format!("{}", TES3));
        assert_eq!("TES3", format!("{:?}", TES3));
        assert_eq!(Ok(SCPT), Tag::from_str("SCPT"));
    }

    #[test]
    fn test_file_type() {
        assert_eq!("ESM", format!("{}", FileType::ESM));
        assert_eq!("ESS", format!("{:?}", FileType::ESS));
        assert_eq!(Some(FileType::ESP), FileType::from_u32(0));
        assert_eq!(None, FileType::from_u32(2));
        assert_eq!(32, FileType::ESS as u32);
        assert_eq!(Ok(FileType::ESP), FileType::from_str("ESP"));
    }

    #[test]
    fn test_record_flags() {
        assert_eq!("PERSISTENT", format!("{}", RecordFlags::PERSISTENT));
        assert_eq!("PERSISTENT", format!("{:?}", RecordFlags::PERSISTENT));
        assert_eq!("PERSISTENT | DELETED", format!("{}", RecordFlags::PERSISTENT | RecordFlags::DELETED));
        assert_eq!(0x202000000000, (RecordFlags::BLOCKED | RecordFlags::DELETED).bits);
        assert_eq!(Some(RecordFlags::BLOCKED | RecordFlags::DELETED), RecordFlags::from_bits(0x202000000000));
        //assert_eq!(Ok(RecordFlags::DELETED | RecordFlags::PERSISTENT), RecordFlags::from_str("DELETED | PERSISTENT"));
        //assert_eq!(Ok(RecordFlags::DELETED | RecordFlags::PERSISTENT), RecordFlags::from_str("PERSISTENT | DELETED"));
        //assert_eq!(Some(Tes3::ESP), Tes3::from_u32(0));
        //assert_eq!(None, Tes3::from_u32(2));
        //assert_eq!(32, Tes3::ESS as u32);
    }
}
