# `fstat-rs`

This project is an exploration of the Rust language / tool chain, using a file system stat checker as a pretense.

This isn't supposed to be used anywhere:
- Hasn't been tested on different hardware/systems
- Still contains a bunch of `unwrap()s`, so won't fail gracefully
- Has very little documentation
- Has no tests (yet, hopefully!)


## `fstat-rs --help`

```
Gathers stats on files/folders, recursively if required,
and outputs them using the provided template.

Available tokens in templates:
- {path} The file/dir path itself
- {name} The file/dir name
- {size_b} The total size in bytes (calculated recursively for directories)
- {size_mb} The total size in mb
- {time_s} The seconds it took to recursively calculate the size of this path

This tool is for learning purposes only.

EXAMPLES:
    # Check a file's size
    `fstat-fs C:/big-file.zip`
    # Output:
    #   Size of C:/big-file.zip is 900mb

    # Check a folder's size with a custom output template
    `fstat-fs -t="name:{name} size:{size_mb}mb time:{time_s}s" C:/folder`
    # Output:
    #   name:folder size:300mb time:1s

    # Check a folder, and it's descendants' sizes with a custom output template
    `fstat-fs -t="{path} = {size_mb}mb ({time_s}s)" -r C:/folder`
    #
    # Output:
    #   C:/folder/file2.zip = 80mb (1s)
    #   C:/folder/file1.zip = 100mb (1s)
    #   C:/folder/file3.zip = 120mb (1s)
    #   C:/folder = 300mb (3s)

    # Disable multi-threading to compare time taken (about 5x longer in my tests)
    `fstat-fs -t="Took {time_s}s" --single-thread C:/folder`
    # Output:
    #   Took 10s

Note: Even without `--recurse` sub-paths will be resursed to calculate sizes, they
just won't be printed out.

USAGE:
    fstat-rs.exe [FLAGS] [OPTIONS] <path>

ARGS:
    <path>    The file/folder path to check

FLAGS:
    -h, --help             Prints help information
    -r, --recurse          The file/folder path to check
    -s, --single-thread    Whether to skip multi-threading (performance check)
    -v, --verbose          Whether to print verbose logs
    -V, --version          Prints version information

OPTIONS:
    -t, --template <template>    Template for output [default: Size of {path} is {size_mb}mb]
```