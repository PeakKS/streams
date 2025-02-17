project('streams',
    version: '0.0.1',
    meson_version: '>= 1.6',
    default_options: [ 'warning_level=3', 'werror=true', ],
)

gnome = import('gnome')

config = configuration_data(
    {
        'APP_ID': 'io.github.PeakKS/Streams',
        'PREFIX': '/io/github/PeakKS/Streams',
    }
)

subdir('data')
subdir('src')

gnome.post_install(
    glib_compile_schemas: true,
    gtk_update_icon_cache: true,
    update_desktop_database: true,
)