//! This modules handles ACPI's Multiple APIC Description Table (MADT).

use super::ACPITable;

/// The Multiple APIC Description Table.
#[repr(C)]
pub struct Madt {
	/// The signature of the structure.
	signature: [u8; 4],
	/// The length of the structure.
	length: u32,
	/// The revision number of the structure.
	revision: u8,
	/// The checksum to check against all the structure's bytes.
	checksum: u8,
	/// An OEM-supplied string that identifies the OEM.
	oemid: [u8; 6],
	/// TODO doc
	oem_table_id: [u8; 8],
	/// TODO doc
	oemrevision: u32,
	/// TODO doc
	creator_id: u32,
	/// TODO doc
	creator_revision: u32,

	/// TODO doc
	local_apic_addr: u32,
	/// TODO doc
	flags: u32,
}

impl Madt {
	/// Executes the given closure for each entry in the MADT.
	pub fn foreach_entry<F: Fn(&EntryHeader)>(&self, _f: F) {
		// TODO
	}
}

impl ACPITable for Madt {
	fn get_expected_signature() -> [u8; 4] {
		[b'M', b'A', b'D', b'T']
	}

	fn get_signature(&self) -> &[u8; 4] {
		&self.signature
	}

	fn get_length(&self) -> usize {
		self.length as _
	}
}

/// Represents an MADT entry header.
#[repr(C)]
pub struct EntryHeader {
	/// The entry type.
	entry_type: u8,
	/// The entry length.
	length: u8,
}

impl EntryHeader {
	/// Returns the type of the entry.
	pub fn get_type(&self) -> u8 {
		self.entry_type
	}

	/// Returns the length of the entry.
	pub fn get_length(&self) -> u8 {
		self.length
	}
}
