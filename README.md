## Command line utility for mass moving or renaming files according to a template

 Moves files that match template from \<SOURCE> directory to \<DESTINATION> directory and renames
 it according to the template. Both arguments shoud contain existing directory path.
 It is forbidden to use ``^, $, +, -, ?, (, ), [, ], {, }, |`` in templates.

 \<SOURCE> contains a template with symbols ``*`` that refer to zero or more proceeding symbols.
 Each star matches its sequential marker number: ``#1, #2, ..., #{10}, #{11}, ... ``

 \<DESTINATION> contains markers and symbols. Each marker refers to the symbols sequence beyond its ``*``.

 Prints files that were successfully moved in format:

 ```console
 source_dir/filename1 -> destination_dir/filename2
 ```
 # Usage

 ```console
 $ mmv 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
 path/to/some_A_filename.bin -> path2/to/changed_A_filename.bin
 path/to/some_A_filename.jpg -> path2/to/changed_A_filename.jpg
 path/to/some_B_filename.bin -> path2/to/changed_B_filename.bin
path/to/some_B_filename.jpg -> path2/to/changed_B_filename.jpg
 ```
*for more information use ``mmv --help``*