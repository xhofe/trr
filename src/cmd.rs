use clap::{Parser, ArgEnum};

#[derive(Parser,Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// The directory to display
    #[clap(default_value = ".")]
    pub path: String,

    // ------- Listing options -------
    /// All files are listed, even hidden files
    #[clap(short='a')]
    pub all: bool,
    /// Only directories are listed
    #[clap(short='d')]
    pub directories: bool,
    /// Follow symbolic links like directories
    #[clap(short='l')]
    pub follow_links: bool,
    /// Print the full path prefix for each file
    #[clap(short='f')]
    pub full_path: bool,
    /// Stay on current filesystem
    #[clap(short='x')]
    pub stay_on_fs: bool,
    /// Descend only level directories deep
    #[clap(short='L')]
    pub level: Option<usize>,
    /// Rerun tree when max dir level reached
    #[clap(short='R')]
    pub rerun: bool,
    /// List only those files that match the pattern given
    #[clap(short='P')]
    pub pattern: Option<String>,
    /// Do not list files that match the given pattern
    #[clap(short='I')]
    pub ignore: Option<String>,
    /// Ignore case when pattern matching
    #[clap(long)]
    pub ignore_case: bool,
    /// Include directory names in -P pattern matching
    #[clap(long)]
    pub matchdirs: bool,
    /// Turn off file/directory count at end of tree listing
    #[clap(long)]
    pub noreport: bool,
    /// Use charset <CHARSET> for terminal/HTML and indentation line output
    #[clap(long)]
    pub charset: Option<char>,
    /// Do not descend dirs with more than <FILELIMIT> files in them
    #[clap(long)]
    pub filelimit: Option<usize>,
    /// Print and format time according to the format <TIMEFMT>
    #[clap(long)]
    pub timefmt: Option<String>,
    /// Output to file instead of stdout
    #[clap(short)]
    pub output: Option<String>,

    // ------- File options -------
    /// Print non-printable characters as '?'
    #[clap(short='q')]
    pub question: bool,
    /// Print non-printable characters as is
    #[clap(short='N')]
    pub n: bool,
    /// Quote filenames with double quotes
    #[clap(short='Q')]
    pub quote: bool,
    /// Print the protections for each file
    #[clap(short='p')]
    pub protections: bool,
    /// Displays file owner or UID number
    #[clap(short='u')]
    pub user: bool,
    /// Displays file group or GID number
    #[clap(short='g')]
    pub group: bool,
    /// Print the size in bytes of each file
    #[clap(short='s')]
    pub size: bool,
    /// Print the size in a more human readable way
    #[clap(short='h')]
    pub human_size: bool,
    /// Like -h, but use in SI units (powers of 1000)
    #[clap(long)]
    pub si: bool,
    /// Print the date of last modification or (-c) status change
    #[clap(short='D')]
    pub date: bool,
    /// Appends '/', '=', '*', '@', '|' or '>' as per ls -F
    #[clap(short='F')]
    pub file_type: bool,
    /// Print inode number of each file
    #[clap(long)]
    pub inodes: bool,
    /// Print device ID number to which each file belongs
    #[clap(long)]
    pub device: bool,

    // ------- Sorting options -------
    /// Sort files alphanumerically by version
    #[clap(short='v')]
    pub version: bool,
    /// Sort files by last modification time
    #[clap(short='t')]
    pub time: bool,
    /// Sort files by last status change time
    #[clap(short='c')]
    pub change: bool,
    /// Leave files unsorted
    #[clap(short='U')]
    pub unsorted: bool,
    /// Reverse the order of the sort
    #[clap(short='r')]
    pub reverse: bool,
    /// List directories before files (-U disables)
    #[clap(long)]
    pub dirsfirst: bool,
    /// Select sort: name,version,size,mtime,ctime
    #[clap(long,arg_enum)]
    pub sort: Option<Sort>,

    // ------- Graphics options -------
    /// Don't print indentation lines
    #[clap(short='i')]
    pub indentation: bool,
    /// Print ANSI lines graphic indentation lines
    #[clap(short='A')]
    pub ansi: bool,
    /// Print with CP437 (console) graphics indentation lines
    #[clap(short='S')]
    pub cp437: bool,
    /// Turn colorization off always (-C overrides)
    #[clap(short='n')]
    pub no_color: bool,
    /// Turn colorization on always
    #[clap(short='C')]
    pub color: bool,

    //  ------- XML/HTML/JSON options -------
    /// Prints out an XML representation of the tree
    #[clap(short='X')]
    pub xml: bool,
    /// Prints out an JSON representation of the tree
    #[clap(short='J')]
    pub json: bool,
    /// Prints out HTML format with baseHREF as top directory
    #[clap(short='H')]
    pub html: Option<String>,
    /// Replace the default HTML title and H1 header with string
    #[clap(short='T')]
    pub title: Option<String>,
    /// Turn off hyperlinks in HTML output
    #[clap(long)]
    pub nolinks: bool,
    // ------- Input options -------
    /// Reads paths from files (.=stdin)
    #[clap(long)]
    pub fromfile: Option<String>,
    
    // ------- Miscellaneous options -------
    // auto generate help and version
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum Sort {
    Name,
    Version,
    Size,
    Mtime,
    Ctime,
}