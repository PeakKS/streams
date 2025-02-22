project('streams',
    version: '0.0.1',
    meson_version: '>= 1.6',
    default_options: [ 'warning_level=3', 'werror=true', ],
)

gnome = import('gnome')

app_id = get_option('app_id')
prefix = '/' + app_id.replace('.', '/')
config = configuration_data(
    {
        'APP_ID': app_id,
        'PREFIX': prefix,
        'TWITCH_CLIENT_ID': get_option('twitch_client_id'),
    }
)

subdir('resources')
subdir('src')

gnome.post_install(
    glib_compile_schemas: true,
    gtk_update_icon_cache: true,
    update_desktop_database: true,
)