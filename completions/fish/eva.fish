# Meta-stuff
complete -c eva -s v -l version -d "Show version of eva"
complete -c eva -l help -d "Show list of command-line options"

# Display options
complete -c eva -s 1 -l oneline -d "Display one entry per line"
complete -c eva -s l -l long -d "Display extended file metadata as a table"
complete -c eva -s G -l grid -d "Display entries in a grid"
complete -c eva -s x -l across -d "Sort the grid across, rather than downwards"
complete -c eva -s R -l recurse -d "Recurse into directories"
complete -c eva -s T -l tree -d "Recurse into directories as a tree"
complete -c eva -s X -l dereference -d "Dereference symbolic links when displaying file information"
complete -c eva -s F -l classify -d "Display type indicator by file names"
complete -c eva -l color \
    -l colour -d "When to use terminal colours" -x -a "
    always\t'Always use colour'
    auto\t'Use colour if standard output is a terminal'
    automatic\t'Use colour if standard output is a terminal'
    never\t'Never use colour'
"
complete -c eva -l color-scale \
    -l colour-scale -d "Highlight levels 'field' distinctly" -x -a "
    all\t''
    age\t''
    size\t''
"
complete -c eva -l color-scale-mode \
    -l colour-scale-mode \
    -d "Use gradient or fixed colors in --color-scale" -x -a "
    fixed\t'Highlight based on fixed colors'
    gradient\t'Highlight based \'field\' in relation to other files'
"
complete -c eva -l icons -d "When to display icons" -x -a "
  always\t'Always display icons'
  auto\t'Display icons if standard output is a terminal'
  automatic\t'Display icons if standard output is a terminal'
  never\t'Never display icons'
"
complete -c eva -l no-quotes -d "Don't quote file names with spaces"
complete -c eva -l hyperlink -d "When to display entries as hyperlinks" -x -a "
  always\t'Always display entries as hyperlinks'
  auto\t'Display hyperlinks if standard output is a terminal'
  automatic\t'Display hyperlinks if standard output is a terminal'
  never\t'Never display entries as hyperlinks'
"
complete -c eva -l follow-symlinks -d "Drill down into symbolic links that point to directories"
complete -c eva -l absolute -d "Display entries with their absolute path" -x -a "
  on\t'Show absolute path for listed entries'
  follow\t'Show absolute path with followed symlinks'
  off\t'Do not show the absolute path'
"
complete -c eva -l smart-group -d "Only show group if it has a different name from owner"

# Filtering and sorting options
complete -c eva -l group-directories-first -d "Sort directories before other files"
complete -c eva -l group-directories-last -d "Sort directories after other files"
complete -c eva -l git-ignore -d "Ignore files mentioned in '.gitignore'"
complete -c eva -s a -l all -d "Show hidden and 'dot' files. Use this twice to also show the '.' and '..' directories"
complete -c eva -s A -l almost-all -d "Equivalent to --all; included for compatibility with `ls -A`"
complete -c eva -s d -l treat-dirs-as-files -d "List directories like regular files"
complete -c eva -s L -l level -d "Limit the depth of recursion" -x -a "1 2 3 4 5 6 7 8 9"
complete -c eva -s w -l width -d "Limits column output of grid, 0 implies auto-width"
complete -c eva -s r -l reverse -d "Reverse the sort order"
complete -c eva -s s -l sort -d "Which field to sort by" -x -a "
    accessed\t'Sort by file accessed time'
    age\t'Sort by file modified time (newest first)'
    changed\t'Sort by changed time'
    created\t'Sort by file modified time'
    date\t'Sort by file modified time'
    ext\t'Sort by file extension'
    Ext\t'Sort by file extension (uppercase first)'
    extension\t'Sort by file extension'
    Extension\t'Sort by file extension (uppercase first)'
    filename\t'Sort by filename'
    Filename\t'Sort by filename (uppercase first)'
    inode\t'Sort by file inode'
    modified\t'Sort by file modified time'
    name\t'Sort by filename'
    Name\t'Sort by filename (uppercase first)'
    newest\t'Sort by file modified time (newest first)'
    none\t'Do not sort files at all'
    oldest\t'Sort by file modified time'
    size\t'Sort by file size'
    time\t'Sort by file modified time'
    type\t'Sort by file type'
"

complete -c eva -s I -l ignore-glob -d "Ignore files that match these glob patterns" -r
complete -c eva -s D -l only-dirs -d "List only directories"
complete -c eva -s f -l only-files -d "List only files"
complete -c eva -l show-symlinks -d "Explicitly show symbolic links (For use with --only-dirs | --only-files)"
complete -c eva -l no-symlinks -d "Do not show symbolic links"

# Long view options
complete -c eva -s b -l binary -d "List file sizes with binary prefixes"
complete -c eva -s B -l bytes -d "List file sizes in bytes, without any prefixes"
complete -c eva -s g -l group -d "List each file's group"
complete -c eva -s h -l header -d "Add a header row to each column"
complete -c eva -s H -l links -d "List each file's number of hard links"
complete -c eva -s i -l inode -d "List each file's inode number"
complete -c eva -s S -l blocksize -d "List each file's size of allocated file system blocks"
complete -c eva -s t -l time -d "Which timestamp field to list" -x -a "
    modified\t'Display modified time'
    changed\t'Display changed time'
    accessed\t'Display accessed time'
    created\t'Display created time'
"
complete -c eva -s m -l modified -d "Use the modified timestamp field"
complete -c eva -s n -l numeric -d "List numeric user and group IDs."
complete -c eva -l changed -d "Use the changed timestamp field"
complete -c eva -s u -l accessed -d "Use the accessed timestamp field"
complete -c eva -s U -l created -d "Use the created timestamp field"
complete -c eva -l time-style -d "How to format timestamps" -x -a "
    default\t'Use the default time style'
    iso\t'Display brief ISO timestamps'
    long-iso\t'Display longer ISO timestamps, up to the minute'
    full-iso\t'Display full ISO timestamps, up to the nanosecond'
    relative\t'Display relative timestamps'
    +FORMAT\t'Use custom time style'
"
complete -c eva -l total-size -d "Show recursive directory size (unix only)"
complete -c eva -l no-permissions -d "Suppress the permissions field"
complete -c eva -s o -l octal-permissions -d "List each file's permission in octal format"
complete -c eva -l no-filesize -d "Suppress the filesize field"
complete -c eva -l no-user -d "Suppress the user field"
complete -c eva -l no-time -d "Suppress the time field"
complete -c eva -s M -l mounts -d "Show mount details"
complete -c eva -l stdin -d "When piping to eva. Read file names from stdin"

# Optional extras
complete -c eva -l git -d "List each file's Git status, if tracked"
complete -c eva -l no-git -d "Suppress Git status"
complete -c eva -l git-repos -d "List each git-repos status and branch name"
complete -c eva -l git-repos-no-status -d "List each git-repos branch name (much faster)"
complete -c eva -s '@' -l extended -d "List each file's extended attributes and sizes"
complete -c eva -s Z -l context -d "List each file's security context"
