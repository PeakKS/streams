rust_config = configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: config,
)

cargo_bin = find_program('cargo')

cargo_env = get_option('cargo_env')
cargo_env += ['CARGO_HOME=' + meson.project_build_root()/'cargo-home']
cargo_env += ['BUILD_ROOT=' + meson.project_build_root()]

cargo_opt = [
    '--manifest-path', meson.project_source_root()/'Cargo.toml',
    '--target-dir', meson.project_build_root()/'src',
]

if get_option('buildtype') == 'release'
    cargo_opt += ['--release']
    rust_target = 'release'
else
    rust_target = 'debug'
endif

custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    output: meson.project_name(),
    console: true,
    command: [
        'env', cargo_env,
        cargo_bin, 'build',
        cargo_opt,
        '&&', 'cp', 'src'/rust_target/meson.project_name(), '@OUTPUT@',
    ],
    depend_files: [
        rust_config,
    ],
    depends: [
        resources,
    ]
)