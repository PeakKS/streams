blueprints = custom_target('blueprints',
    input: files(
        'ui/window.blp',
        'ui/preferences/dialog.blp',
        'ui/preferences/general.blp',
    ),
    output: '.',
    command: [
        find_program('blueprint-compiler'),
        'batch-compile',
        '@OUTPUT@',
        '@CURRENT_SOURCE_DIR@',
        '@INPUT@',
    ],
)

resources = gnome.compile_resources('streams',
    configure_file(
        input: 'streams.gresource.xml.in',
        output: 'streams.gresource.xml',
        configuration: config,
    ),
    gresource_bundle: true,
    dependencies: [
        blueprints,
    ],
)