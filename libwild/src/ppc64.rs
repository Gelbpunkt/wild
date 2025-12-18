use crate::arch::Arch;
use crate::elf::PLT_ENTRY_SIZE;
use crate::ensure;
use crate::error;
use crate::error::Result;
use crate::layout::Layout;
use crate::layout::PropertyClass;
use linker_utils::elf::DynamicRelocationKind;
use linker_utils::elf::RelocationKind;
use linker_utils::elf::RelocationKindInfo;
use linker_utils::elf::ppc64_rel_type_to_string;
use linker_utils::elf::shf;
use linker_utils::ppc64::relocation_type_from_raw;
use linker_utils::ppc64::RelaxationKind;
use linker_utils::relaxation::RelocationModifier;

/// We only implement ELFv2.
/// See https://files.openpower.foundation/s/cfA2oFPXbbZwEBK/download/64biteflv2abi-v1.5.pdf

pub(crate) struct PPC64;

impl crate::arch::Arch for PPC64 {
    type Relaxation = Relaxation;

    const KIND: crate::arch::Architecture = crate::arch::Architecture::PPC64;

    fn elf_header_arch_magic() -> u16 {
        object::elf::EM_PPC64
    }

    #[inline(always)]
    fn relocation_from_raw(r_type: u32) -> Result<RelocationKindInfo> {
        linker_utils::ppc64::relocation_type_from_raw(r_type).ok_or_else(|| {
            error!(
                "Unsupported relocation type {}",
                Self::rel_type_to_string(r_type)
            )
        })
    }

    fn get_dynamic_relocation_type(relocation: DynamicRelocationKind) -> u32 {
        relocation.ppc64_r_type()
    }

    fn rel_type_to_string(r_type: u32) -> std::borrow::Cow<'static, str> {
        ppc64_rel_type_to_string(r_type)
    }

    fn write_plt_entry(
        plt_entry: &mut [u8],
        got_address: u64,
        plt_address: u64,
    ) -> crate::error::Result {
        unimplemented!();
        Ok(())
    }

    fn local_symbols_in_debug_info() -> bool {
        false
    }

    fn tp_offset_start(layout: &Layout<'_>) -> u64 {
        layout.tls_start_address_ppc64()
    }

    fn get_property_class(_property_type: u32) -> Option<PropertyClass> {
        None
    }

    fn merge_eflags(_eflags: &[u32]) -> Result<u32> {
        Ok(0)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Relaxation {
    kind: RelaxationKind,
    rel_info: RelocationKindInfo,
    mandatory: bool,
}

const TLSDESC_ADR_PAGE21_INSN_SEQUENCE: &[u8] = &[
    0x0, 0x0, 0x0, 0x90, // adrp    x0, 0
];

const TLSDESC_ADD_LO12_INSN_SEQUENCE: &[u8] = &[
    0x0, 0x0, 0x0, 0x91, // add     x0, x0, #0x0
];

macro_rules! rel_info_from_type {
    ($r_type:expr) => {
        const { relocation_type_from_raw($r_type).unwrap() }
    };
}

impl crate::arch::Relaxation for Relaxation {
    #[allow(unused_variables)]
    #[inline(always)]
    fn new(
        relocation_kind: u32,
        section_bytes: &[u8],
        offset_in_section: u64,
        flags: crate::value_flags::ValueFlags,
        output_kind: crate::output_kind::OutputKind,
        section_flags: linker_utils::elf::SectionFlags,
        non_zero_address: bool,
    ) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        None
    }

    fn apply(&self, section_bytes: &mut [u8], offset_in_section: &mut u64, addend: &mut i64) {
        //self.kind.apply(section_bytes, offset_in_section, addend);
        unimplemented!()
    }

    fn rel_info(&self) -> RelocationKindInfo {
        self.rel_info
    }

    fn debug_kind(&self) -> impl std::fmt::Debug {
        &self.kind
    }

    fn next_modifier(&self) -> RelocationModifier {
        //self.kind.next_modifier()
        unimplemented!()
    }

    fn is_mandatory(&self) -> bool {
        self.mandatory
    }
}
