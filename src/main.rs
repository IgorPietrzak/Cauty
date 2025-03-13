mod nauty;

use std::ptr;
use std::os::raw::c_int;

fn main() {
    let mut g = vec![6, 5, 3]; // K_3: m=1, n=3
    let mut lab = vec![0, 2, 1];
    let mut ptn = vec![1, 0, 0];
    let mut canon = vec![0; 3];
    let mut orbits = vec![0; 3];
    let mut worksize = vec![0; 200];
    let mut stats = nauty::StatsBlk {
        grpsize1: 0.0,
        grpsize2: 0,
        numorbits: 0,
        errstatus: 0,
    };
    let mut options = nauty::OptionBlk {
        getcanon: 1,
        digraph: 0,
        writeautoms: 0,
        writemarkers: 0,
        defaultptn: 1,
        cartesian: 0,
        linelength: 0,
        outfile: ptr::null_mut(),
        userrefproc: ptr::null_mut(),
        userautomproc: ptr::null_mut(),
        userlevelproc: ptr::null_mut(),
        usernodeproc: ptr::null_mut(),
        usercanonproc: ptr::null_mut(),
        invarproc: ptr::null_mut(),
        tc_level: 0,
        mininvarlevel: 0,
        maxinvarlevel: 0,
        invararg: 0,
        dispatch: unsafe { &nauty::dispatch_graph },
        schreier: ptr::null_mut(),
        extra_options: ptr::null_mut(),
    };

    println!("g: {:?}", g);
    println!("m: 1, n: 3");
    println!("lab: {:?}", lab);
    println!("ptn: {:?}", ptn);
    println!("canon len: {}, orbits len: {}, worksize len: {}", canon.len(), orbits.len(), worksize.len());
    println!("dispatch ptr before: {:p}", options.dispatch);

    unsafe {
        let options_ptr = &mut options as *mut nauty::OptionBlk;
        println!("options ptr: {:p}", options_ptr);
        println!("dispatch ptr inside: {:p}", (*options_ptr).dispatch);
        nauty::nauty(
            g.as_mut_ptr(),        // 1
            lab.as_mut_ptr(),      // 2
            ptn.as_mut_ptr(),      // 3
            ptr::null_mut(),       // 4: active (NULL is fine for default)
            orbits.as_mut_ptr(),   // 5
            options_ptr,           // 6
            &mut stats,            // 7
            worksize.as_mut_ptr(), // 8: workspace
            200,                    // 9: worksize (size in words)
            1,                     // 10: m (fixed from 3)
            3,                     // 11: n (fixed from 1)
            canon.as_mut_ptr(),    // 12
        );

        println!("Canonical lab: {:?}", lab);
        println!(
            "Stats: group size = {} * 10^{}, orbits = {}, errstatus = {}",
            stats.grpsize1, stats.grpsize2, stats.numorbits, stats.errstatus
        );
    }
}
