use either::{Either};
use std::fmt::{self, Display, Debug};
use std::str::{FromStr};
use ::nom::IResult;
use ::nom::branch::alt;
use ::nom::combinator::{value as nom_value, map, opt};
use ::nom::bytes::complete::tag as nom_tag;
use ::nom::multi::{fold_many0};
use ::nom::sequence::{preceded, terminated, pair};
use ::nom::bytes::complete::take_while;
use encoding::types::Encoding;
use encoding::all::{WINDOWS_1251, WINDOWS_1252};
use std::cell::Cell;
//use serde::{Serialize, Serializer, Deserialize, Deserializer};
//use serde::de::{self, Unexpected};
//use num_traits::cast::{ToPrimitive, FromPrimitive};

pub use crate::tag::*;

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

enum_serde!([
    FileType, FileTypeDeserializer, "file type",
    u32, from_u32, to_u32, visit_u32, serialize_u32, deserialize_u32,
    Unsigned, u64
]);

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

enum_serde!([
    DialogType, DialogTypeDeserializer, "dialog type",
    u8, from_u8, to_u8, visit_u8, serialize_u8, deserialize_u8,
    Unsigned, u64
]);

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

enum_serde!([
    EffectRange, EffectRangeDeserializer, "effect range",
    i32, from_i32, to_i32, visit_i32, serialize_i32, deserialize_i32,
    Signed, i64
]);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum LinebreakStyle {
    Unix,
    Dos
}

impl LinebreakStyle {
    pub fn new_line(self) -> &'static str {
        if self == LinebreakStyle::Unix { "\n" } else { "\r\n" }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct StringZ {
    pub str: String,
    pub has_tail_zero: bool
}

impl Default for StringZ {
    fn default() -> StringZ { StringZ { str: String::default(), has_tail_zero: true } }
}

impl<T: Into<String>> From<T> for StringZ {
    fn from(t: T) -> StringZ { StringZ{ str: t.into(), has_tail_zero: true } }
}

#[derive(Copy, Clone, Debug)]
pub enum FieldType {
    Binary,
    String { trim_tail_zeros: bool },
    StringZ,
    Multiline { linebreaks: LinebreakStyle, trim_tail_zeros: bool },
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
    ScriptMetadata,
    DialogMetadata,
    DeletionMark,
    FileMetadata,
    Npc,
    SavedNpc,
    Effect,
}

impl FieldType {
    pub fn from_tags(record_tag: Tag, field_tag: Tag) -> FieldType {
        match (record_tag, field_tag) {
            (INFO, ACDT) => FieldType::String { trim_tail_zeros: false },
            (CELL, ACTN) => FieldType::Int,
            (NPC_, ANAM) => FieldType::StringZ,
            (_, ANAM) => FieldType::String { trim_tail_zeros: false },
            (_, ASND) => FieldType::String { trim_tail_zeros: false },
            (_, AVFX) => FieldType::String { trim_tail_zeros: false },
            (ARMO, BNAM) => FieldType::String { trim_tail_zeros: true },
            (BODY, BNAM) => FieldType::String { trim_tail_zeros: true },
            (CELL, BNAM) => FieldType::StringZ,
            (CLOT, BNAM) => FieldType::String { trim_tail_zeros: true },
            (CONT, BNAM) => FieldType::Multiline { linebreaks: LinebreakStyle::Dos, trim_tail_zeros: true },
            (INFO, BNAM) => FieldType::Multiline { linebreaks: LinebreakStyle::Dos, trim_tail_zeros: true },
            (NPC_, BNAM) => FieldType::StringZ,
            (PCDT, BNAM) => FieldType::String { trim_tail_zeros: false },
            (REGN, BNAM) => FieldType::String { trim_tail_zeros: false },
            (_, BNAM) => FieldType::Multiline { linebreaks: LinebreakStyle::Dos, trim_tail_zeros: false },
            (_, BSND) => FieldType::String { trim_tail_zeros: false },
            (_, BVFX) => FieldType::String { trim_tail_zeros: false },
            (ARMO, CNAM) => FieldType::String { trim_tail_zeros: true },
            (KLST, CNAM) => FieldType::Int,
            (NPC_, CNAM) => FieldType::StringZ,
            (REGN, CNAM) => FieldType::Int,
            (_, CNAM) => FieldType::String { trim_tail_zeros: false },
            (_, CSND) => FieldType::String { trim_tail_zeros: false },
            (_, CVFX) => FieldType::String { trim_tail_zeros: false },
            (DIAL, DATA) => FieldType::DialogMetadata,
            (LAND, DATA) => FieldType::Int,
            (LEVC, DATA) => FieldType::Int,
            (LEVI, DATA) => FieldType::Int,
            (LTEX, DATA) => FieldType::String { trim_tail_zeros: false },
            (SSCR, DATA) => FieldType::String { trim_tail_zeros: true },
            (TES3, DATA) => FieldType::Long,
            (QUES, DATA) => FieldType::String { trim_tail_zeros: false },
            (DIAL, DELE) => FieldType::DeletionMark,
            (_, DESC) => FieldType::String { trim_tail_zeros: false },
            (_, DNAM) => FieldType::String { trim_tail_zeros: false },
            (ALCH, ENAM) => FieldType::Effect,
            (ARMO, ENAM) => FieldType::String { trim_tail_zeros: false },
            (ENCH, ENAM) => FieldType::Effect,
            (PCDT, ENAM) => FieldType::Long,
            (SPEL, ENAM) => FieldType::Effect,
            (CELL, FGTN) => FieldType::String { trim_tail_zeros: false },
            (_, FLAG) => FieldType::Int,
            (_, FLTV) => FieldType::Float,
            (ACTI, FNAM) => FieldType::StringZ,
            (PCDT, FNAM) => FieldType::Binary,
            (RACE, FNAM) => FieldType::StringZ,
            (_, FNAM) => FieldType::String { trim_tail_zeros: false },
            (CELL, FRMR) => FieldType::Int,
            (TES3, HEDR) => FieldType::FileMetadata,
            (_, HSND) => FieldType::String { trim_tail_zeros: false },
            (_, HVFX) => FieldType::String { trim_tail_zeros: false },
            (_, INAM) => FieldType::String { trim_tail_zeros: false },
            (ARMO, INDX) => FieldType::Byte,
            (CLOT, INDX) => FieldType::Byte,
            (_, INDX) => FieldType::Int,
            (LAND, INTV) => FieldType::Long,
            (LEVC, INTV) => FieldType::Short,
            (LEVI, INTV) => FieldType::Short,
            (_, INTV) => FieldType::Int,
            (INGR, IRDT) => FieldType::Ingredient,
            (_, ITEX) => FieldType::String { trim_tail_zeros: false },
            (NPC_, KNAM) => FieldType::StringZ,
            (PCDT, KNAM) => FieldType::Binary,
            (_, KNAM) => FieldType::String { trim_tail_zeros: false },
            (PCDT, LNAM) => FieldType::Long,
            (CELL, LSHN) => FieldType::String { trim_tail_zeros: false },
            (CELL, LSTN) => FieldType::String { trim_tail_zeros: false },
            (_, LVCR) => FieldType::Byte,
            (FMAP, MAPD) => FieldType::Compressed,
            (FMAP, MAPH) => FieldType::Long,
            (TES3, MAST) => FieldType::String { trim_tail_zeros: false },
            (PCDT, MNAM) => FieldType::String { trim_tail_zeros: false },
            (CELL, MNAM) => FieldType::Byte,
            (LIGH, MODL) => FieldType::StringZ,
            (_, MODL) => FieldType::String { trim_tail_zeros: false },
            (CELL, NAM0) => FieldType::Int,
            (SPLM, NAM0) => FieldType::Byte,
            (CELL, NAM5) => FieldType::Int,
            (CELL, NAM9) => FieldType::Int,
            (PCDT, NAM9) => FieldType::Int,
            (CELL, NAME) => FieldType::StringZ,
            (JOUR, NAME) => FieldType::Multiline { linebreaks: LinebreakStyle::Unix, trim_tail_zeros: false },
            (SPLM, NAME) => FieldType::Int,
            (SSCR, NAME) => FieldType::String { trim_tail_zeros: true },
            (_, NAME) => FieldType::String { trim_tail_zeros: false },
            (_, ND3D) => FieldType::Byte,
            (INFO, NNAM) => FieldType::StringZ,
            (LEVC, NNAM) => FieldType::Byte,
            (LEVI, NNAM) => FieldType::Byte,
            (_, NNAM) => FieldType::String { trim_tail_zeros: false },
            (_, NPCO) => FieldType::Reference,
            (NPC_, NPDT) => FieldType::Npc,
            (NPCC, NPDT) => FieldType::SavedNpc,
            (BSGN, NPCS) => FieldType::FixedString(32),
            (NPC_, NPCS) => FieldType::FixedString(32),
            (RACE, NPCS) => FieldType::FixedString(32),
            (_, NPCS) => FieldType::String { trim_tail_zeros: false },
            (_, ONAM) => FieldType::String { trim_tail_zeros: false },
            (INFO, PNAM) => FieldType::StringZ,
            (PCDT, PNAM) => FieldType::Binary,
            (_, PNAM) => FieldType::String { trim_tail_zeros: false },
            (_, PTEX) => FieldType::String { trim_tail_zeros: false },
            (_, RGNN) => FieldType::String { trim_tail_zeros: false },
            (FACT, RNAM) => FieldType::FixedString(32),
            (_, RNAM) => FieldType::String { trim_tail_zeros: false },
            (SCPT, SCHD) => FieldType::ScriptMetadata,
            (_, SCRI) => FieldType::String { trim_tail_zeros: false },
            (_, SCTX) => FieldType::Multiline { linebreaks: LinebreakStyle::Dos, trim_tail_zeros: true },
            (SCPT, SCVR) => FieldType::MultiString,
            (_, SCVR) => FieldType::String { trim_tail_zeros: false },
            (CELL, SLSD) => FieldType::Binary,
            (PCDT, SNAM) => FieldType::Binary,
            (REGN, SNAM) => FieldType::Binary,
            (_, SNAM) => FieldType::String { trim_tail_zeros: false },
            (_, STRV) => FieldType::String { trim_tail_zeros: false },
            (ALCH, TEXT) => FieldType::String { trim_tail_zeros: false },
            (BOOK, TEXT) => FieldType::Multiline { linebreaks: LinebreakStyle::Dos, trim_tail_zeros: true },
            (_, TEXT) => FieldType::Multiline { linebreaks: LinebreakStyle::Dos, trim_tail_zeros: false },
            (_, TNAM) => FieldType::String { trim_tail_zeros: false },
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
            (_, XSOL) => FieldType::String { trim_tail_zeros: false },
            (SPLM, XNAM) => FieldType::Byte,
            (CELL, XSCL) => FieldType::Int,
            (CELL, ZNAM) => FieldType::Byte,
            _ => FieldType::Binary
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    pub weight: f32,
    pub value: u32,
    pub effects: [i32; 4],
    pub skills: [i32; 4],
    pub attributes: [i32; 4]
}

#[derive(Debug, Clone)]
pub struct ScriptMetadata {
    pub name: String,
    pub shorts: u32,
    pub longs: u32,
    pub floats: u32,
    pub data_size: u32,
    pub var_table_size: u32
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub version: u32,
    pub file_type: FileType,
    pub author: String,
    pub description: Vec<String>,
    pub records_count: u32
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct SavedNpc {
    pub disposition: i16,
    pub reputation: i16,
    pub index: u32,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Npc {
    pub level: u16,
    pub disposition: i8,
    pub reputation: i8,
    pub rank: i8,
    pub gold: i32,
    pub characteristics: Either<u32, NpcCharacteristics>
}

#[derive(Debug, Clone)]
pub enum Field {
    Binary(Vec<u8>),
    String(String),
    StringZ(StringZ),
    Multiline(Vec<String>),
    MultiString(Vec<String>),
    Reference(i32, String),
    Float(f32),
    Int(i32),
    Short(i16),
    Long(i64),
    Byte(u8),
    Compressed(Vec<u8>),
    Ingredient(Ingredient),
    ScriptMetadata(ScriptMetadata),
    DialogMetadata(Either<u32, DialogType>),
    DeletionMark,
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

fn pipe(input: &str) -> IResult<&str, (), ()> {
    nom_value((),
        terminated(preceded(take_while(char::is_whitespace), nom_tag("|")), take_while(char::is_whitespace))
    )(input)
}

fn record_flag(input: &str) -> IResult<&str, RecordFlags, ()> {
    alt((
        nom_value(RecordFlags::PERSISTENT, nom_tag("PERSISTENT")),
        nom_value(RecordFlags::BLOCKED, nom_tag("BLOCKED")),
        nom_value(RecordFlags::DELETED, nom_tag("DELETED"))
    ))(input)
}

impl FromStr for RecordFlags {
    type Err = ();
    
    fn from_str(s: &str) -> Result<RecordFlags, Self::Err> {
        let (unconsumed, flags) = map(opt(map(pair(
            record_flag,
            fold_many0(preceded(pipe, record_flag), RecordFlags::empty(), |a, v| a | v)
        ), |(a, b)| a | b)), |m| m.unwrap_or(RecordFlags::empty()))(s).map_err(|_: ::nom::Err<()>| ())?;
        if !unconsumed.is_empty() { return Err(()); }
        Ok(flags)
    }
}

enum_serde!({
    RecordFlags, RecordFlagsDeserializer, "record flags",
    u64, from_bits, bits, visit_u64, serialize_u64, deserialize_u64,
    Unsigned, u64
});

#[derive(Debug, Clone)]
pub struct Record {
    pub tag: Tag,
    pub flags: RecordFlags,
    pub fields: Vec<Field>,
}

macro_attr! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[derive(IterVariants!(CodePageVariants))]
    pub enum CodePage {
        English,
        Russian,
    }
}

impl CodePage {
    pub fn encoding(self) -> &'static dyn Encoding {
        match self {
            CodePage::English => WINDOWS_1252,
            CodePage::Russian => WINDOWS_1251,
        }
    }
}

thread_local!(pub static CODE_PAGE: Cell<CodePage> = Cell::new(CodePage::English));

#[cfg(test)]
mod tests {
    use crate::*;
    use num_traits::cast::FromPrimitive;
    use std::str::FromStr;
    use std::hash::Hash;
    use encoding::{DecoderTrap, EncoderTrap};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn record_flags_traits() {
        assert_eq!(RecordFlags::PERSISTENT, *&RecordFlags::PERSISTENT);
        assert!(RecordFlags::PERSISTENT < RecordFlags::BLOCKED);
        let mut hasher = DefaultHasher::new();
        RecordFlags::DELETED.hash(&mut hasher);
    }
    
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
        assert_eq!(Ok(RecordFlags::DELETED), RecordFlags::from_str("DELETED"));
        assert_eq!(Ok(RecordFlags::DELETED | RecordFlags::PERSISTENT), RecordFlags::from_str("DELETED|PERSISTENT"));
        assert_eq!(Ok(RecordFlags::DELETED | RecordFlags::PERSISTENT), RecordFlags::from_str("PERSISTENT | DELETED"));
        assert_eq!(Ok(RecordFlags::empty()), RecordFlags::from_str(""));
        assert_eq!(Err(()), RecordFlags::from_str(" "));
    }

    #[test]
    fn all_code_pages_are_single_byte_encodings() {
        for code_page in CodePage::iter_variants() {
            let encoding = code_page.encoding();
            for byte in 0u8 ..= 255 {
                let c = encoding.decode(&[byte], DecoderTrap::Strict).unwrap();
                let b = encoding.encode(&c, EncoderTrap::Strict).unwrap();
                assert_eq!(b.len(), 1);
                assert_eq!(b[0], byte);
            }
        }
    }
}