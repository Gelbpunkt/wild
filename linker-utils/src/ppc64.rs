use crate::elf::AllowedRange;
use crate::elf::PPC64Instruction;
use crate::elf::RelocationKind;
use crate::elf::RelocationKindInfo;
use crate::elf::RelocationSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelaxationKind {}

const LOW24: RelocationSize = RelocationSize::bit_mask_ppc64(6, 30, PPC64Instruction::Low24Type);
const HALF16DS: RelocationSize =
    RelocationSize::bit_mask_ppc64(0, 14, PPC64Instruction::Half16DsType);

#[must_use]
pub const fn relocation_type_from_raw(r_type: u32) -> Option<RelocationKindInfo> {
    let (kind, size, mask, range, alignment) = match r_type {
        // 3.5.3 Relocation Types Table
        object::elf::R_PPC64_NONE => (
            RelocationKind::None,
            RelocationSize::ByteSize(0),
            None,
            AllowedRange::no_check(),
            1,
        ),

        object::elf::R_PPC64_ADDR32 => (
            RelocationKind::Absolute,
            RelocationSize::ByteSize(4),
            None,
            AllowedRange::new(i32::MIN as i64, i32::MAX as i64),
            1,
        ),
        object::elf::R_PPC64_ADDR64 => (
            RelocationKind::Absolute,
            RelocationSize::ByteSize(8),
            None,
            AllowedRange::no_check(),
            8, // ?
        ),

        object::elf::R_PPC64_REL24 => (
            RelocationKind::Relative,
            LOW24,
            None,
            AllowedRange::new(-(2i64.pow(23)), 2i64.pow(23)),
            4,
        ),
        object::elf::R_PPC64_REL32 => (
            RelocationKind::Relative,
            RelocationSize::ByteSize(4),
            None,
            AllowedRange::new(i32::MIN as i64, i32::MAX as i64),
            4,
        ),
        object::elf::R_PPC64_REL64 => (
            RelocationKind::Relative,
            RelocationSize::ByteSize(8),
            None,
            AllowedRange::no_check(),
            8,
        ),

        object::elf::R_PPC64_ADDR24 => (
            RelocationKind::PPC64Addr24,
            LOW24,
            None,
            AllowedRange::new(-(2i64.pow(23)), 2i64.pow(23)),
            4,
        ),

        object::elf::R_PPC64_REL16_HA => (
            RelocationKind::PPC64Rel16Ha,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),
        object::elf::R_PPC64_REL16_LO => (
            RelocationKind::PPC64Rel16Lo,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),

        object::elf::R_PPC64_TOC16_HA => (
            RelocationKind::PPC64Toc16Ha,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),
        object::elf::R_PPC64_TOC16_LO => (
            RelocationKind::PPC64Toc16Lo,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),
        object::elf::R_PPC64_TOC16_LO_DS => (
            RelocationKind::PPC64Toc16LoDs,
            HALF16DS,
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),

        object::elf::R_PPC64_GOT_TLSLD16_HA => (
            RelocationKind::PPC64GotTlsLd16Ha,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),
        object::elf::R_PPC64_GOT_TLSLD16_LO => (
            RelocationKind::PPC64GotTlsLd16Ha,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),

        object::elf::R_PPC64_TLSLD => (
            RelocationKind::None,
            RelocationSize::ByteSize(0),
            None,
            AllowedRange::no_check(),
            1,
        ),

        object::elf::R_PPC64_DTPREL16_HA => (
            RelocationKind::PPC64Dtprel16Ha,
            RelocationSize::ByteSize(2),
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),
        object::elf::R_PPC64_DTPREL16_LO_DS => (
            RelocationKind::PPC64Dtprel16LoDs,
            HALF16DS,
            None,
            AllowedRange::new(i16::MIN as i64, i16::MAX as i64),
            2,
        ),

        _ => return None,
    };

    Some(RelocationKindInfo {
        kind,
        size,
        mask,
        range,
        alignment,
    })
}

impl PPC64Instruction {
    // Encode computed relocation value and store it based on the encoding of an instruction.
    // A handy page where one can easily find instruction encoding:
    // https://msyksphinz-self.github.io/riscv-isadoc/html/index.html.

    // During the build of the static libc.a, there are various places where the immediate operand
    // of an instruction is already filled up. Thus, we zero the bits before a relocation value is
    // applied.
    pub fn write_to_value(self, extracted_value: u64, _negative: bool, dest: &mut [u8]) {
        todo!()
    }

    /// The inverse of `write_to_value`. Returns `(extracted_value, negative)`. Supplied `bytes`
    /// must be at least 4 bytes, otherwise we panic.
    #[must_use]
    pub fn read_value(self, bytes: &[u8]) -> (u64, bool) {
        todo!()
    }
}
