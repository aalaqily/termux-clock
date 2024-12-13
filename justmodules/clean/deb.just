default:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/deb' -exec rm -rf {} +

packages:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/deb/packages' -exec rm -rf {} +

manifests:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/target/deb/manifests' -exec rm -rf {} +
