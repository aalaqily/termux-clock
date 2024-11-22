backups:
    find {{ justfile_directory() }} -type f -name '*~' -exec rm -f {} +

mod deb 'clean/deb.just'

mod target 'clean/target.just'
