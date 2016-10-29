use std::fs;
use std::mem::size_of;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Read;
use std::io::Write;

mod filesystem;
use filesystem::Disk;

#[repr(u16)]
enum Stat {
    Clean = 1,
    Error = 2,
}

#[repr(u16)]
enum ErrMethod {
    Ignore = 1,
    ReadOnly = 2,
    Panic = 3,
}

#[repr(C,u32)]
enum OSID {
    Linux = 0,
    GNU_HURD = 1,
    MASIX = 2,
    FreeBSD = 3,
    Other = 4,
}

#[repr(C,u32)]
enum FeatureOpt {
    Prealloc = 0x0001,
    AFS = 0x0002,
    Journal = 0x0004,
    ExtendAttr = 0x0008,
    Resize = 0x0010,
    HashDir = 0x0020,
}

#[repr(C,u32)]
enum FeatureExist {
    Compress = 0x0001,
    DirType = 0x0002,
    Replay = 0x0004,
    JournalDev = 0x0008,
}

#[repr(C,u32)]
enum FeatureNot {
    Sparse = 0x0001,
    Fsiz64 = 0x0002,
    BTree = 0x0004,
}

#[repr(C,packed)]
struct SuperBlock {
    ino_total: u32,
    blk_total: u32,
    blk_su_reserve: u32,
    blk_unallocated: u32,
    ino_unallocated: u32,
    blk_ind_this: u32,
    blk_size: u32, // in log2
    frg_size: u32,
    grp_blk_num: u32,
    grp_frg_num: u32,
    gpr_ino_num: u32,
    time_mount: i32,
    time_written: i32,
    cnt_mnt_since_last_check: u16,
    cnt_mnt_allow_before_check: u16,
    sig: u16,
    stat: Stat,
    err_method: ErrMethod,
    version_minor: u16,
    time_last_check: i32,
    time_internal_check: i32,
    osid: OSID,
    version_major: u32,
    uid_allow_reserved: u16,
    gid_allow_reserved: u16,
    ext: SuperBlockExt,
}

#[repr(C,packed)]
struct SuperBlockExt {
    first_non_reserve_inode: u32,
    inode_size: u16,
    grp_ind_this: u16,
    feature_opt: FeatureOpt,
    feature_exist: FeatureExist,
    feature_not: FeatureNot,
    fsid: [u8; 16],
    v_name: [u8; 16],
    path_last_mnt: [u8; 64],
    compress_algo: u32,
    blk_prealloc_file: u8,
    blk_prealloc_dir: u8,
    __unused1: u16,
    journal_id: [u8; 16],
    journal_inode: u32,
    journal_dev: u32,
    orphan_inode_list: u32,
    __unused2: [u8; 788],
}

#[repr(C,packed)]
struct GroupDescriptor {
    blk_bitmap_addr: u32,
    ino_bitmap_addr: u32,
    ino_table_addr: u32,
    blk_free: u16,
    ino_free: u16,
    dir_cnt: u16,
    __unused: [u8; 14],
}

#[repr(C,packed)]
struct InodeData {
    mode: u16,
    uid: u16,
    size_low: u32,
    time_access: i32,
    time_create: i32,
    time_modify: i32,
    time_delete: i32,
    gid: u16,
    hlink: u16,
    sector: u32,
    flag: u32,
    osspec1: u32,
    blk_direct: [u32; 12],
    blk_singly: u32,
    blk_doubly: u32,
    blk_triply: u32,
    gen: u32,
    ex1: u32,
    ex2: u32,
    frg: u32,
    osspec2: [u8; 12],
}

#[repr(u8)]
enum DirectoryType {
    Unknown = 0,
    RegularFile = 1,
    Directory = 2,
    CharacterDev = 3,
    BockDevice = 4,
    FIFO = 5,
    Socket = 6,
    SymLink = 7,
}

#[repr(C,packed)]
struct DirectoryEntryInfo {
    inode: u32,
    size: u16,
    name_len: u8,
    dir_type: DirectoryType,
}

struct DirectoryEntry {
    info: DirectoryEntryInfo,
    name: String,
}

struct Ext2 {
    super_block: SuperBlock,
    disk: &mut Disk,
}

impl Ext2 {
    pub fn open(_disk: &mut Disk) -> Option<Ext2> {
        let ext2: Ext2;
        _disk.seek(SeekFrom::Start(1024));
        match _disk.read(&mut ext2.super_block as &mut [u8; 1024]) {
            Ok(n) => {
                |n| {
                    if n != 1024 {
                        return None;
                    }
                }
            }
            Err(err) => return None,
        }

        ext2
    }
}

#[test]
fn struct_size_check() {
    assert_eq!(2, size_of::<Stat>());
    assert_eq!(2, size_of::<ErrMethod>());
    assert_eq!(940, size_of::<SuperBlockExt>());
    assert_eq!(1024, size_of::<SuperBlock>());
    assert_eq!(32, size_of::<GroupDescriptor>());
    assert_eq!(128, size_of::<InodeData>());
    assert_eq!(8, size_of::<DirectoryEntryInfo>());
}