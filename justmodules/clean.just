default *FLAGS:
    cargo clean {{ FLAGS }}

dir +TARGET_DIR:
    cargo clean --target-dir target/{{ TARGET_DIR }}

backups:
    find {{ justfile_directory() }} -type f -name '*~' -exec rm -f {} +

doc:
    find {{ justfile_directory() }} -type d -wholename "{{ justfile_directory() }}/target/doc" -exec rm -rf {} +

debug:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/debug' -exec rm -rf {} +

release:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/release' -exec rm -rf {} +

aarch64-linux-android:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/aarch64-linux-android' -exec rm -rf {} +

alias aarch64 := aarch64-linux-android

arm-linux-androideabi:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/arm-linux-androideabi' -exec rm -rf {} +

alias arm := arm-linux-androideabi

x86_64-linux-android:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/x86_64-linux-android' -exec rm -rf {} +

alias x86_64 := x86_64-linux-android

i686-linux-android:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/i686-linux-android' -exec rm -rf {} +

alias i686 := i686-linux-android

mod deb 'clean/deb.just'
