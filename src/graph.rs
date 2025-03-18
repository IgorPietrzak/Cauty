use std::ptr;
use std::os::raw::c_int;
use crate::nauty;

pub struct Graph {
   pub adj: Vec<Vec<usize>>,
   pub n: usize,
}

impl Graph {
   pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut adj = vec![vec![]; n];
        for &(u, v) in edges {
            adj[u].push(v);
            adj[v].push(u);
        }
        Graph { adj, n }
    }

    pub fn to_nauty_dense(&self) -> (Vec<nauty::setword>, c_int, c_int) {
        let n = self.n as c_int;
        let m = ((self.n + 31) / 32) as c_int; // 32-bit setwords
        let mut g = vec![0; (m * n) as usize];

        // MSB-first: bit 31 = vertex 0, bit 30 = vertex 1, etc.
        for u in 0..self.n {
            for &v in &self.adj[u] {
                g[u] |= 1u32 << (31 - v);
            }
        }
        (g, m, n)
    }

    pub fn run_nauty_with_coloring(&self, colors: &[c_int]) -> (Vec<nauty::setword>, nauty::StatsBlk) {
        let (mut g, m, n) = self.to_nauty_dense();
        let mut lab = (0..self.n).map(|i| i as c_int).collect::<Vec<_>>();
        let mut ptn = vec![0; self.n];

        // Sort vertices by color to ensure consistent partition
        let mut vertex_color: Vec<(c_int, usize)> = colors.iter().cloned().zip(0..self.n).collect();
        vertex_color.sort(); // Sort by color
        for i in 0..self.n {
            lab[i] = vertex_color[i].1 as c_int; // Vertex index
            ptn[i] = if i < self.n - 1 && vertex_color[i].0 == vertex_color[i + 1].0 {
                1 // Same color, continue partition
            } else {
                0 // Color boundary
            };
        }
        ptn[self.n - 1] = 0; // Last element always 0

        let mut canon = vec![0; (m * n) as usize];
        let mut orbits = vec![0; self.n];
        let mut worksize = vec![0; 200];
        let mut stats = nauty::StatsBlk {
            grpsize1: 0.0,
            grpsize2: 0,
            numorbits: 0,
            errstatus: 0,
        };
        let mut options = nauty::OptionBlk {
            getcanon: 1,        // Compute canonical form
            digraph: 0,
            writeautoms: 0,
            writemarkers: 0,
            defaultptn: 0,      // Use custom partition
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
            schreier: 0,
            extra_options: ptr::null_mut(),
        };

        unsafe {
            nauty::nauty(
                g.as_mut_ptr(),
                lab.as_mut_ptr(),
                ptn.as_mut_ptr(),
                ptr::null_mut(),
                orbits.as_mut_ptr(),
                &mut options,
                &mut stats,
                worksize.as_mut_ptr(),
                200,
                m,
                n,
                canon.as_mut_ptr(),
            );
        }

        (canon, stats) // Return canonical graph and stats
    }

    pub fn are_colorings_equivalent(&self, colors1: &[c_int], colors2: &[c_int]) -> bool {
        let (canon1, _) = self.run_nauty_with_coloring(colors1);
        let (canon2, _) = self.run_nauty_with_coloring(colors2);
        canon1 == canon2 // Equivalent if canonical forms match
    }
}
