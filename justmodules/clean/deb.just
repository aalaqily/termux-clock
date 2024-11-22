default:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/deb' -exec rm -rf {} +

packages:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/deb/packages' -exec rm -rf {} +

manifests:
    find {{ justfile_directory() }} -type d -wholename '{{ justfile_directory() }}/deb/manifests' -exec rm -rf {} +
