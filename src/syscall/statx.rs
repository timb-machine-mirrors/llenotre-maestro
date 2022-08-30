//! The statx system call returns the extended status of a file.

use super::util;
use crate::errno::Errno;
use crate::file::mountpoint::MountSource;
use crate::file::FileContent;
use crate::process::mem_space::ptr::SyscallPtr;
use crate::process::mem_space::ptr::SyscallString;
use crate::process::regs::Regs;
use crate::process::Process;
use crate::util::io::IO;

/// Structure representing a timestamp with the statx syscall.
#[repr(C)]
struct StatxTimestamp {
	/// Seconds since the Epoch (UNIX time)
	tv_sec: i64,
	/// Nanoseconds since tv_sec
	tv_nsec: u32,
	/// Reserved field.
	__reserved: i32,
}

/// Structure containing the extended attributes for a file.
#[repr(C)]
struct Statx {
	/// Mask of bits indicating filled fields
	stx_mask: u32,
	/// Block size for filesystem I/O
	stx_blksize: u32,
	/// Extra file attribute indicators
	stx_attributes: u64,
	/// Number of hard links
	stx_nlink: u32,
	/// User ID of owner
	stx_uid: u32,
	/// Group ID of owner
	stx_gid: u32,
	/// File type and mode
	stx_mode: u16,

	/// Padding.
	__padding0: u16,

	/// Inode number
	stx_ino: u64,
	/// Total size in bytes
	stx_size: u64,
	/// Number of 512B blocks allocated
	stx_blocks: u64,
	/// Mask to show what's supported in stx_attributes
	stx_attributes_mask: u64,

	/// Last access
	stx_atime: StatxTimestamp,
	/// Creation
	stx_btime: StatxTimestamp,
	/// Last status change
	stx_ctime: StatxTimestamp,
	/// Last modification
	stx_mtime: StatxTimestamp,

	/// Major ID (if the file is a device)
	stx_rdev_major: u32,
	/// Minor ID (if the file is a device)
	stx_rdev_minor: u32,
	/// Major ID of the device containing the filesystem where the file resides
	stx_dev_major: u32,
	/// Minor ID of the device containing the filesystem where the file resides
	stx_dev_minor: u32,

	/// Mount ID.
	stx_mnt_id: u64,

	/// Padding.
	__padding1: [u64; 13],
}

/// The implementation of the `statx` syscall.
pub fn statx(regs: &Regs) -> Result<i32, Errno> {
	let dirfd = regs.ebx as i32;
	let pathname: SyscallString = (regs.ecx as usize).into();
	let flags = regs.edx as i32;
	let _mask = regs.esi as u32;
	let statxbuff: SyscallPtr<Statx> = (regs.edi as usize).into();

	if pathname.is_null() || statxbuff.is_null() {
		return Err(errno!(EINVAL));
	}

	// TODO Implement all flags

	// Whether symbolic links are to be followed
	let follow_links = flags & super::access::AT_SYMLINK_NOFOLLOW == 0;

	// Getting the file
	let file_mutex = {
		let proc_mutex = Process::get_current().unwrap();
		let proc_guard = proc_mutex.lock();
		let proc = proc_guard.get_mut();

		let mem_space = proc.get_mem_space().unwrap();
		let mem_space_guard = mem_space.lock();

		let pathname = pathname
			.get(&mem_space_guard)?
			.ok_or_else(|| errno!(EFAULT))?;
		util::get_file_at(proc_guard, follow_links, dirfd, pathname, flags)?
	};
	let file_guard = file_mutex.lock();
	let file = file_guard.get();

	// TODO Use mask?

	// If the file is a device, get the major and minor numbers
	let (stx_rdev_major, stx_rdev_minor) = match file.get_file_content() {
		FileContent::BlockDevice { major, minor } | FileContent::CharDevice { major, minor } => {
			(*major, *minor)
		}
		_ => (0, 0),
	};

	// Getting the major and minor numbers of the device of the file's filesystem
	let (stx_dev_major, stx_dev_minor) = {
		if let Some(mountpoint_mutex) = file.get_location().get_mountpoint() {
			let mountpoint_guard = mountpoint_mutex.lock();
			let mountpoint = mountpoint_guard.get();

			match mountpoint.get_source() {
				MountSource::Device { major, minor, .. } => (*major, *minor),
				MountSource::File(_) | MountSource::NoDev(_) => (0, 0),
			}
		} else {
			(0, 0)
		}
	};

	// Filling the structure
	let statx_val = Statx {
		stx_mask: !0,      // TODO
		stx_blksize: 512,  // TODO
		stx_attributes: 0, // TODO
		stx_nlink: file.get_hard_links_count() as _,
		stx_uid: file.get_uid() as _,
		stx_gid: file.get_gid() as _,
		stx_mode: file.get_mode() as _,

		__padding0: 0,

		stx_ino: file.get_location().inode,
		stx_size: file.get_size(),
		stx_blocks: file.get_blocks_count(),
		stx_attributes_mask: 0, // TODO

		stx_atime: StatxTimestamp {
			tv_sec: file.get_atime() as _,
			tv_nsec: 0, // TODO
			__reserved: 0,
		},
		stx_btime: StatxTimestamp {
			tv_sec: 0, // TODO
			tv_nsec: 0, // TODO
			__reserved: 0,
		},
		stx_ctime: StatxTimestamp {
			tv_sec: file.get_ctime() as _,
			tv_nsec: 0, // TODO
			__reserved: 0,
		},
		stx_mtime: StatxTimestamp {
			tv_sec: file.get_mtime() as _,
			tv_nsec: 0, // TODO
			__reserved: 0,
		},

		stx_rdev_major,
		stx_rdev_minor,
		stx_dev_major,
		stx_dev_minor,

		stx_mnt_id: 0, // TODO

		__padding1: [0; 13],
	};

	{
		let proc_mutex = Process::get_current().unwrap();
		let proc_guard = proc_mutex.lock();
		let proc = proc_guard.get_mut();

		let mem_space = proc.get_mem_space().unwrap();
		let mem_space_guard = mem_space.lock();

		let statx = statxbuff.get_mut(&mem_space_guard)?.ok_or(errno!(EFAULT))?;
		*statx = statx_val;
	}

	Ok(0)
}