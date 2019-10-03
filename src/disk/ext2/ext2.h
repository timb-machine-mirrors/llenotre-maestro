#ifndef EXT2_H
# define EXT2_H

# include <kernel.h>
# include <disk/disk.h>

# define EXT2_SIGNATURE	0xef53

# define EXT2_STATE_CLEAN	1
# define EXT2_STATE_ERROR	2

# define EXT2_ERROR_HANDLING_CONTINUE	1
# define EXT2_ERROR_HANDLING_READONLY	2
# define EXT2_ERROR_HANDLING_PANIC		3

# define EXT2_OS_ID_LINUX		0
# define EXT2_OS_ID_GNU_HURD	1
# define EXT2_OS_ID_MASIX		2
# define EXT2_OS_ID_FREEBSD		3
# define EXT2_OS_ID_OTHER		4

__attribute__((packed))
struct ext2_superblock
{
	uint32_t total_inodes;
	uint32_t total_blocks;
	uint32_t superuser_reserved_blocks;
	uint32_t unallocated_blocks;
	uint32_t unallocated_inodes;
	uint32_t superblock_number;
	uint32_t block_size;
	uint32_t fragment_size;
	uint32_t blocks_per_group;
	uint32_t fragments_per_group;
	uint32_t inodes_per_group;
	uint32_t last_mount_time;
	uint32_t last_write_time;
	uint16_t mounts_since_last_check;
	uint16_t max_mounts_between_checks;
	uint16_t signature;
	uint16_t state;
	uint16_t error_handling_method;
	uint16_t minor_version;
	uint32_t last_check_time;
	uint32_t check_interval_time;
	uint32_t os_id;
	uint32_t major_version;
	uint16_t superuser;
	uint16_t supergroup;
};

__attribute__((packed))
struct ext2_extended_superblock
{
	ext2_superblock superblock;
	// TODO
};

typedef struct ext2_superblock ext2_superblock_t;
typedef struct ext2_extended_superblock ext2_extended_superblock_t;

// TODO

#endif
