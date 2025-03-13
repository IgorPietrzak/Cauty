use std::os::raw::{c_int, c_void};

pub type setword = u32;
pub type graph = setword;
pub type permutation = c_int;

#[repr(C)]
pub struct OptionBlk {
    pub getcanon: c_int,
    pub digraph: c_int,
    pub writeautoms: c_int,
    pub writemarkers: c_int,
    pub defaultptn: c_int,
    pub cartesian: c_int,
    pub linelength: c_int,
    pub outfile: *mut c_void, // FILE*
    pub userrefproc: *mut c_void,
    pub userautomproc: *mut c_void,
    pub userlevelproc: *mut c_void,
    pub usernodeproc: *mut c_void,
    pub usercanonproc: *mut c_void,
    pub invarproc: *mut c_void,
    pub tc_level: c_int,
    pub mininvarlevel: c_int,
    pub maxinvarlevel: c_int,
    pub invararg: c_int,
    pub dispatch: *const DispatchVec,
    pub schreier: *mut c_int,
    pub extra_options: *mut c_void,
}

#[repr(C)]
pub struct DispatchVec {
    pub isautom: extern "C" fn(*mut graph, *mut permutation, c_int, c_int, c_int) -> c_int,
    pub testcanlab: extern "C" fn(*mut graph, *mut graph, *mut c_int, *mut c_int, c_int, c_int) -> c_int,
    pub updatecan: extern "C" fn(*mut graph, *mut graph, *mut permutation, *mut c_int, c_int, c_int),
    pub initgroup: extern "C" fn(*mut graph, c_int, *mut permutation, *mut c_int, *mut c_int, *mut c_int, c_int) -> c_int,
    pub groupautomproc: extern "C" fn(c_int, *mut permutation, *mut c_int, c_int, c_int, c_int),
    pub groupcanonproc: extern "C" fn(*mut graph, *mut c_int, *mut permutation, *mut c_int, c_int, c_int, c_int),
    pub refine: extern "C" fn(*mut graph, *mut c_int, *mut c_int, c_int, *mut c_int, *mut permutation, *mut setword, *mut c_int, c_int, c_int),
    pub cheapautom: extern "C" fn(*mut c_int, c_int, c_int, c_int),
    pub targetcell: extern "C" fn(*mut graph, *mut c_int, *mut c_int, c_int, c_int, c_int, *mut setword, c_int, c_int, c_int) -> c_int,
    pub freedyn: extern "C" fn(),
    pub check: extern "C" fn(c_int, c_int, c_int, c_int) -> c_int,
}

#[repr(C)]
pub struct StatsBlk {
    pub grpsize1: f64,
    pub grpsize2: c_int,
    pub numorbits: c_int,
    pub errstatus: c_int,
}

extern "C" {
    pub fn nauty(
        g: *mut setword,
        lab: *mut c_int,
        ptn: *mut c_int,
        active: *mut setword, // Changed from *mut c_int
        orbits: *mut c_int,
        options: *mut OptionBlk,
        stats: *mut StatsBlk,
        workspace: *mut setword, // Renamed from worksize
        worksize: c_int,        // Added as separate arg
        m: c_int,
        n: c_int,
        canon: *mut setword,
    );

    pub static dispatch_graph: DispatchVec;
}
