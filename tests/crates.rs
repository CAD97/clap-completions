use clap::{App, AppSettings, Arg, ArgAction, Command};

mod shared;
use shared::*;

macro_rules! tests {
    ($name:literal, $app:ident) => {
        #[cfg(feature = "nu")]
        insta::assert_display_snapshot!(
            concat!($name, "/nu"),
            clap_completions::nu::Completions::new(&$app)
        );

        #[cfg(feature = "nu")]
        if let Ok(help) = get_nu_help(&$app) {
            insta::assert_snapshot!(concat!($name, "/nu/help"), help);
        }

        insta::assert_snapshot!(concat!($name, "/help"), get_clap_help(&mut $app));
    };
}

#[test]
fn addr2line() {
    let mut app = Command::new("addr2line")
        .version("0.1")
        .about("A fast addr2line Rust port")
        .args(&[
            Arg::new("exe")
                .short('e')
                .long("exe")
                .value_name("filename")
                .help(
                    "Specify the name of the executable for which addresses should be translated.",
                )
                .required(true),
            Arg::new("sup")
                .long("sup")
                .value_name("filename")
                .help("Path to supplementary object file."),
            Arg::new("functions")
                .short('f')
                .long("functions")
                .help("Display function names as well as file and line number information."),
            Arg::new("pretty").short('p').long("pretty-print").help(
                "Make the output more human friendly: each location are printed on one line.",
            ),
            Arg::new("inlines").short('i').long("inlines").help(
                "If the address belongs to a function that was inlined, the source information for \
                all enclosing scopes back to the first non-inlined function will also be printed.",
            ),
            Arg::new("addresses").short('a').long("addresses").help(
                "Display the address before the function name, file and line number information.",
            ),
            Arg::new("basenames")
                .short('s')
                .long("basenames")
                .help("Display only the base of each file name."),
            Arg::new("demangle").short('C').long("demangle").help(
                "Demangle function names. \
                Specifying a specific demangling style (like GNU addr2line) is not supported. \
                (TODO)"
            ),
            Arg::new("llvm")
                .long("llvm")
                .help("Display output in the same format as llvm-symbolizer."),
            Arg::new("addrs")
                .takes_value(true)
                .multiple_occurrences(true)
                .help("Addresses to use instead of reading from stdin."),
        ]);
    app.build();

    tests!("addr2line", app);
}

#[test]
fn bindgen() {
    static RUST_TARGET_STRINGS: &[&str] = &[
        "1.0", "1.17", "1.19", "1.20", "1.21", "1.25", "1.26", "1.27", "1.28", "1.30", "1.33",
        "1.36", "1.40", "1.47",
    ];

    let rust_target_help = format!(
        "Version of the Rust compiler to target. Valid options are: {:?}. Defaults to {:?}.",
        RUST_TARGET_STRINGS, "1.47",
    );

    let mut app = App::new("bindgen")
        .about("Generates Rust bindings from C/C++ headers.")
        .setting(clap::AppSettings::NoAutoVersion)
        .override_usage("bindgen [FLAGS] [OPTIONS] <header> -- <clang-args>...")
        .args(&[
            Arg::new("header")
                .help("C or C++ header file")
                .required_unless_present("V"),
            Arg::new("depfile")
                .long("depfile")
                .takes_value(true)
                .help("Path to write depfile to"),
            Arg::new("default-enum-style")
                .long("default-enum-style")
                .help("The default style of code used to generate enums.")
                .value_name("variant")
                .default_value("consts")
                .possible_values(&[
                    "consts",
                    "moduleconsts",
                    "bitfield",
                    "newtype",
                    "rust",
                    "rust_non_exhaustive",
                ])
                .multiple_occurrences(false),
            Arg::new("bitfield-enum")
                .long("bitfield-enum")
                .help(
                    "Mark any enum whose name matches <regex> as a set of \
                     bitfield flags.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("newtype-enum")
                .long("newtype-enum")
                .help("Mark any enum whose name matches <regex> as a newtype.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("rustified-enum")
                .long("rustified-enum")
                .help("Mark any enum whose name matches <regex> as a Rust enum.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("constified-enum")
                .long("constified-enum")
                .help(
                    "Mark any enum whose name matches <regex> as a series of \
                     constants.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("constified-enum-module")
                .long("constified-enum-module")
                .help(
                    "Mark any enum whose name matches <regex> as a module of \
                     constants.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("default-macro-constant-type")
                .long("default-macro-constant-type")
                .help("The default signed/unsigned type for C macro constants.")
                .value_name("variant")
                .default_value("unsigned")
                .possible_values(&["signed", "unsigned"])
                .multiple_occurrences(false),
            Arg::new("default-alias-style")
                .long("default-alias-style")
                .help("The default style of code used to generate typedefs.")
                .value_name("variant")
                .default_value("type_alias")
                .possible_values(&[
                    "type_alias",
                    "new_type",
                    "new_type_deref",
                ])
                .multiple_occurrences(false),
            Arg::new("normal-alias")
                .long("normal-alias")
                .help(
                    "Mark any typedef alias whose name matches <regex> to use \
                     normal type aliasing.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
             Arg::new("new-type-alias")
                .long("new-type-alias")
                .help(
                    "Mark any typedef alias whose name matches <regex> to have \
                     a new type generated for it.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
             Arg::new("new-type-alias-deref")
                .long("new-type-alias-deref")
                .help(
                    "Mark any typedef alias whose name matches <regex> to have \
                     a new type with Deref and DerefMut to the inner type.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("blocklist-type")
                .alias("blacklist-type")
                .long("blocklist-type")
                .help("Mark <type> as hidden.")
                .value_name("type")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("blocklist-function")
                .alias("blacklist-function")
                .long("blocklist-function")
                .help("Mark <function> as hidden.")
                .value_name("function")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("blocklist-item")
                .alias("blacklist-item")
                .long("blocklist-item")
                .help("Mark <item> as hidden.")
                .value_name("item")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("blocklist-file")
                .alias("blacklist-file")
                .long("blocklist-file")
                .help("Mark all contents of <path> as hidden.")
                .value_name("path")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("no-layout-tests")
                .long("no-layout-tests")
                .help("Avoid generating layout tests for any type."),
            Arg::new("no-derive-copy")
                .long("no-derive-copy")
                .help("Avoid deriving Copy on any type."),
            Arg::new("no-derive-debug")
                .long("no-derive-debug")
                .help("Avoid deriving Debug on any type."),
            Arg::new("no-derive-default")
                .long("no-derive-default")
                .hide(true)
                .help("Avoid deriving Default on any type."),
            Arg::new("impl-debug").long("impl-debug").help(
                "Create Debug implementation, if it can not be derived \
                 automatically.",
            ),
            Arg::new("impl-partialeq")
                .long("impl-partialeq")
                .help(
                    "Create PartialEq implementation, if it can not be derived \
                     automatically.",
                ),
            Arg::new("with-derive-default")
                .long("with-derive-default")
                .help("Derive Default on any type."),
            Arg::new("with-derive-hash")
                .long("with-derive-hash")
                .help("Derive hash on any type."),
            Arg::new("with-derive-partialeq")
                .long("with-derive-partialeq")
                .help("Derive partialeq on any type."),
            Arg::new("with-derive-partialord")
                .long("with-derive-partialord")
                .help("Derive partialord on any type."),
            Arg::new("with-derive-eq")
                .long("with-derive-eq")
                .help(
                    "Derive eq on any type. Enable this option also \
                     enables --with-derive-partialeq",
                ),
            Arg::new("with-derive-ord")
                .long("with-derive-ord")
                .help(
                    "Derive ord on any type. Enable this option also \
                     enables --with-derive-partialord",
                ),
            Arg::new("no-doc-comments")
                .long("no-doc-comments")
                .help(
                    "Avoid including doc comments in the output, see: \
                     https://github.com/rust-lang/rust-bindgen/issues/426",
                ),
            Arg::new("no-recursive-allowlist")
                .long("no-recursive-allowlist")
                .alias("no-recursive-whitelist")
                .help(
                    "Disable allowlisting types recursively. This will cause \
                     bindgen to emit Rust code that won't compile! See the \
                     `bindgen::Builder::allowlist_recursively` method's \
                     documentation for details.",
                ),
            Arg::new("objc-extern-crate")
                .long("objc-extern-crate")
                .help("Use extern crate instead of use for objc."),
            Arg::new("generate-block")
                .long("generate-block")
                .help("Generate block signatures instead of void pointers."),
            Arg::new("block-extern-crate")
                .long("block-extern-crate")
                .help("Use extern crate instead of use for block."),
            Arg::new("distrust-clang-mangling")
                .long("distrust-clang-mangling")
                .help("Do not trust the libclang-provided mangling"),
            Arg::new("builtins").long("builtins").help(
                "Output bindings for builtin definitions, e.g. \
                 __builtin_va_list.",
            ),
            Arg::new("ctypes-prefix")
                .long("ctypes-prefix")
                .help(
                    "Use the given prefix before raw types instead of \
                     ::std::os::raw.",
                )
                .value_name("prefix"),
            Arg::new("anon-fields-prefix")
                .long("anon-fields-prefix")
                .help("Use the given prefix for the anon fields.")
                .value_name("prefix")
                .default_value("__bindgen_anon_"),
            Arg::new("time-phases")
                .long("time-phases")
                .help("Time the different bindgen phases and print to stderr"),
            // All positional arguments after the end of options marker, `--`
            Arg::new("clang-args").last(true).multiple_occurrences(true),
            Arg::new("emit-clang-ast")
                .long("emit-clang-ast")
                .help("Output the Clang AST for debugging purposes."),
            Arg::new("emit-ir")
                .long("emit-ir")
                .help("Output our internal IR for debugging purposes."),
            Arg::new("emit-ir-graphviz")
                .long("emit-ir-graphviz")
                .help("Dump graphviz dot file.")
                .value_name("path"),
            Arg::new("enable-cxx-namespaces")
                .long("enable-cxx-namespaces")
                .help("Enable support for C++ namespaces."),
            Arg::new("disable-name-namespacing")
                .long("disable-name-namespacing")
                .help(
                    "Disable namespacing via mangling, causing bindgen to \
                     generate names like \"Baz\" instead of \"foo_bar_Baz\" \
                     for an input name \"foo::bar::Baz\".",
                ),
            Arg::new("disable-nested-struct-naming")
                .long("disable-nested-struct-naming")
                .help(
                    "Disable nested struct naming, causing bindgen to generate \
                     names like \"bar\" instead of \"foo_bar\" for a nested \
                     definition \"struct foo { struct bar { } b; };\"."
                ),
            Arg::new("disable-untagged-union")
                .long("disable-untagged-union")
                .help(
                    "Disable support for native Rust unions.",
                ),
            Arg::new("disable-header-comment")
                .long("disable-header-comment")
                .help("Suppress insertion of bindgen's version identifier into generated bindings.")
                .multiple_occurrences(true),
            Arg::new("ignore-functions")
                .long("ignore-functions")
                .help(
                    "Do not generate bindings for functions or methods. This \
                     is useful when you only care about struct layouts.",
                ),
            Arg::new("generate")
                .long("generate")
                .help(
                    "Generate only given items, split by commas. \
                     Valid values are \"functions\",\"types\", \"vars\", \
                     \"methods\", \"constructors\" and \"destructors\".",
                )
                .takes_value(true),
            Arg::new("ignore-methods")
                .long("ignore-methods")
                .help("Do not generate bindings for methods."),
            Arg::new("no-convert-floats")
                .long("no-convert-floats")
                .help("Do not automatically convert floats to f32/f64."),
            Arg::new("no-prepend-enum-name")
                .long("no-prepend-enum-name")
                .help("Do not prepend the enum name to constant or newtype variants."),
            Arg::new("no-include-path-detection")
                .long("no-include-path-detection")
                .help("Do not try to detect default include paths"),
            Arg::new("fit-macro-constant-types")
                .long("fit-macro-constant-types")
                .help("Try to fit macro constants into types smaller than u32/i32"),
            Arg::new("unstable-rust")
                .long("unstable-rust")
                .help("Generate unstable Rust code (deprecated; use --rust-target instead).")
                .multiple_occurrences(true), // FIXME: Pass legacy test suite
            Arg::new("opaque-type")
                .long("opaque-type")
                .help("Mark <type> as opaque.")
                .value_name("type")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Write Rust bindings to <output>.")
                .takes_value(true),
            Arg::new("raw-line")
                .long("raw-line")
                .help("Add a raw line of Rust code at the beginning of output.")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("module-raw-line")
                .long("module-raw-line")
                .help("Add a raw line of Rust code to a given module.")
                .multiple_occurrences(true)
                .number_of_values(2)
                .value_names(&["module-name", "raw-line"]),
            Arg::new("rust-target")
                .long("rust-target")
                .help(rust_target_help.as_ref())
                .takes_value(true),
            Arg::new("use-core")
                .long("use-core")
                .help("Use types from Rust core instead of std."),
            Arg::new("conservative-inline-namespaces")
                .long("conservative-inline-namespaces")
                .help(
                    "Conservatively generate inline namespaces to avoid name \
                     conflicts.",
                ),
            Arg::new("use-msvc-mangling")
                .long("use-msvc-mangling")
                .help("MSVC C++ ABI mangling. DEPRECATED: Has no effect."),
            Arg::new("allowlist-function")
                .long("allowlist-function")
                .alias("whitelist-function")
                .help(
                    "Allowlist all the free-standing functions matching \
                     <regex>. Other non-allowlisted functions will not be \
                     generated.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("generate-inline-functions")
                .long("generate-inline-functions")
                .help("Generate inline functions."),
            Arg::new("allowlist-type")
                .long("allowlist-type")
                .alias("whitelist-type")
                .help(
                    "Only generate types matching <regex>. Other non-allowlisted types will \
                     not be generated.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("allowlist-var")
                .long("allowlist-var")
                .alias("whitelist-var")
                .help(
                    "Allowlist all the free-standing variables matching \
                     <regex>. Other non-allowlisted variables will not be \
                     generated.",
                )
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("allowlist-file")
                .alias("allowlist-file")
                .long("allowlist-file")
                .help("Allowlist all contents of <path>.")
                .value_name("path")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("verbose")
                .long("verbose")
                .help("Print verbose error messages."),
            Arg::new("dump-preprocessed-input")
                .long("dump-preprocessed-input")
                .help(
                    "Preprocess and dump the input header files to disk. \
                     Useful when debugging bindgen, using C-Reduce, or when \
                     filing issues. The resulting file will be named \
                     something like `__bindgen.i` or `__bindgen.ii`.",
                ),
            Arg::new("no-record-matches")
                .long("no-record-matches")
                .help(
                    "Do not record matching items in the regex sets. \
                     This disables reporting of unused items.",
                ),
            Arg::new("size_t-is-usize")
                .long("size_t-is-usize")
                .help("Translate size_t to usize."),
            Arg::new("no-rustfmt-bindings")
                .long("no-rustfmt-bindings")
                .help("Do not format the generated bindings with rustfmt."),
            Arg::new("rustfmt-bindings")
                .long("rustfmt-bindings")
                .help(
                    "Format the generated bindings with rustfmt. DEPRECATED: \
                     --rustfmt-bindings is now enabled by default. Disable \
                     with --no-rustfmt-bindings.",
                ),
            Arg::new("rustfmt-configuration-file")
                .long("rustfmt-configuration-file")
                .help(
                    "The absolute path to the rustfmt configuration file. \
                     The configuration file will be used for formatting the bindings. \
                     This parameter is incompatible with --no-rustfmt-bindings.",
                )
                .value_name("path")
                .multiple_occurrences(false)
                .number_of_values(1),
            Arg::new("no-partialeq")
                .long("no-partialeq")
                .help("Avoid deriving PartialEq for types matching <regex>.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("no-copy")
                .long("no-copy")
                .help("Avoid deriving Copy for types matching <regex>.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("no-debug")
                .long("no-debug")
                .help("Avoid deriving Debug for types matching <regex>.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("no-default")
                .long("no-default")
                .help("Avoid deriving/implement Default for types matching <regex>.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("no-hash")
                .long("no-hash")
                .help("Avoid deriving Hash for types matching <regex>.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("must-use-type")
                .long("must-use-type")
                .help("Add #[must_use] annotation to types matching <regex>.")
                .value_name("regex")
                .multiple_occurrences(true)
                .number_of_values(1),
            Arg::new("enable-function-attribute-detection")
                .long("enable-function-attribute-detection")
                .help(
                    "Enables detecting unexposed attributes in functions (slow).
                       Used to generate #[must_use] annotations.",
                ),
            Arg::new("use-array-pointers-in-arguments")
                .long("use-array-pointers-in-arguments")
                .help("Use `*const [T; size]` instead of `*const T` for C arrays"),
            Arg::new("wasm-import-module-name")
                .long("wasm-import-module-name")
                .value_name("name")
                .help("The name to be used in a #[link(wasm_import_module = ...)] statement"),
            Arg::new("dynamic-loading")
                .long("dynamic-loading")
                .takes_value(true)
                .help("Use dynamic loading mode with the given library name."),
            Arg::new("dynamic-link-require-all")
                .long("dynamic-link-require-all")
                .help("Require successful linkage to all functions in the library."),
            Arg::new("respect-cxx-access-specs")
                .long("respect-cxx-access-specs")
                .help("Makes generated bindings `pub` only for items if the items are publically accessible in C++."),
            Arg::new("translate-enum-integer-types")
                .long("translate-enum-integer-types")
                .help("Always translate enum integer types to native Rust integer types."),
            Arg::new("c-naming")
                .long("c-naming")
                .help("Generate types with C style naming."),
            Arg::new("explicit-padding")
                .long("explicit-padding")
                .help("Always output explicit padding fields."),
            Arg::new("vtable-generation")
                .long("vtable-generation")
                .help("Enables generation of vtable functions."),
            Arg::new("V")
                .long("version")
                .help("Prints the version, and exits"),
        ]);

    tests!("bindgen", app);
}

pub trait AppExt: Sized {
    fn _arg(self, arg: Arg<'static>) -> Self;

    /// Do not use this method, it is only for backwards compatibility.
    /// Use `arg_package_spec_no_all` instead.
    fn arg_package_spec(
        self,
        package: &'static str,
        all: &'static str,
        exclude: &'static str,
    ) -> Self {
        self.arg_package_spec_no_all(package, all, exclude)
            ._arg(flag("all", "Alias for --workspace (deprecated)"))
    }

    /// Variant of arg_package_spec that does not include the `--all` flag
    /// (but does include `--workspace`). Used to avoid confusion with
    /// historical uses of `--all`.
    fn arg_package_spec_no_all(
        self,
        package: &'static str,
        all: &'static str,
        exclude: &'static str,
    ) -> Self {
        self.arg_package_spec_simple(package)
            ._arg(flag("workspace", all))
            ._arg(multi_opt("exclude", "SPEC", exclude))
    }

    fn arg_package_spec_simple(self, package: &'static str) -> Self {
        self._arg(optional_multi_opt("package", "SPEC", package).short('p'))
    }

    fn arg_package(self, package: &'static str) -> Self {
        self._arg(
            optional_opt("package", package)
                .short('p')
                .value_name("SPEC"),
        )
    }

    fn arg_jobs(self) -> Self {
        self._arg(
            opt("jobs", "Number of parallel jobs, defaults to # of CPUs")
                .short('j')
                .value_name("N"),
        )
        ._arg(flag(
            "keep-going",
            "Do not abort the build as soon as there is an error (unstable)",
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn arg_targets_all(
        self,
        lib: &'static str,
        bin: &'static str,
        bins: &'static str,
        example: &'static str,
        examples: &'static str,
        test: &'static str,
        tests: &'static str,
        bench: &'static str,
        benches: &'static str,
        all: &'static str,
    ) -> Self {
        self.arg_targets_lib_bin_example(lib, bin, bins, example, examples)
            ._arg(optional_multi_opt("test", "NAME", test))
            ._arg(flag("tests", tests))
            ._arg(optional_multi_opt("bench", "NAME", bench))
            ._arg(flag("benches", benches))
            ._arg(flag("all-targets", all))
    }

    fn arg_targets_lib_bin_example(
        self,
        lib: &'static str,
        bin: &'static str,
        bins: &'static str,
        example: &'static str,
        examples: &'static str,
    ) -> Self {
        self._arg(flag("lib", lib))
            ._arg(optional_multi_opt("bin", "NAME", bin))
            ._arg(flag("bins", bins))
            ._arg(optional_multi_opt("example", "NAME", example))
            ._arg(flag("examples", examples))
    }

    fn arg_targets_bins_examples(
        self,
        bin: &'static str,
        bins: &'static str,
        example: &'static str,
        examples: &'static str,
    ) -> Self {
        self._arg(optional_multi_opt("bin", "NAME", bin))
            ._arg(flag("bins", bins))
            ._arg(optional_multi_opt("example", "NAME", example))
            ._arg(flag("examples", examples))
    }

    fn arg_targets_bin_example(self, bin: &'static str, example: &'static str) -> Self {
        self._arg(optional_multi_opt("bin", "NAME", bin))
            ._arg(optional_multi_opt("example", "NAME", example))
    }

    fn arg_features(self) -> Self {
        self._arg(
            multi_opt(
                "features",
                "FEATURES",
                "Space or comma separated list of features to activate",
            )
            .short('F'),
        )
        ._arg(flag("all-features", "Activate all available features"))
        ._arg(flag(
            "no-default-features",
            "Do not activate the `default` feature",
        ))
    }

    fn arg_release(self, release: &'static str) -> Self {
        self._arg(flag("release", release).short('r'))
    }

    fn arg_profile(self, profile: &'static str) -> Self {
        self._arg(opt("profile", profile).value_name("PROFILE-NAME"))
    }

    fn arg_doc(self, doc: &'static str) -> Self {
        self._arg(flag("doc", doc))
    }

    fn arg_target_triple(self, target: &'static str) -> Self {
        self._arg(multi_opt("target", "TRIPLE", target))
    }

    fn arg_target_dir(self) -> Self {
        self._arg(
            opt("target-dir", "Directory for all generated artifacts").value_name("DIRECTORY"),
        )
    }

    fn arg_manifest_path(self) -> Self {
        self._arg(opt("manifest-path", "Path to Cargo.toml").value_name("PATH"))
    }

    fn arg_message_format(self) -> Self {
        self._arg(multi_opt("message-format", "FMT", "Error format"))
    }

    fn arg_build_plan(self) -> Self {
        self._arg(flag(
            "build-plan",
            "Output the build plan in JSON (unstable)",
        ))
    }

    fn arg_unit_graph(self) -> Self {
        self._arg(flag("unit-graph", "Output build graph in JSON (unstable)"))
    }

    fn arg_new_opts(self) -> Self {
        self._arg(
            opt(
                "vcs",
                "Initialize a new repository for the given version \
                 control system (git, hg, pijul, or fossil) or do not \
                 initialize any version control at all (none), overriding \
                 a global configuration.",
            )
            .value_name("VCS")
            .value_parser(["git", "hg", "pijul", "fossil", "none"]),
        )
        ._arg(flag("bin", "Use a binary (application) template [default]"))
        ._arg(flag("lib", "Use a library template"))
        ._arg(
            opt("edition", "Edition to set for the crate generated")
                .value_parser(["2015", "2018", "2021"])
                .value_name("YEAR"),
        )
        ._arg(
            opt(
                "name",
                "Set the resulting package name, defaults to the directory name",
            )
            .value_name("NAME"),
        )
    }

    fn arg_index(self) -> Self {
        self._arg(opt("index", "Registry index URL to upload the package to").value_name("INDEX"))
    }

    fn arg_dry_run(self, dry_run: &'static str) -> Self {
        self._arg(flag("dry-run", dry_run))
    }

    fn arg_ignore_rust_version(self) -> Self {
        self._arg(flag(
            "ignore-rust-version",
            "Ignore `rust-version` specification in packages",
        ))
    }

    fn arg_future_incompat_report(self) -> Self {
        self._arg(flag(
            "future-incompat-report",
            "Outputs a future incompatibility report at the end of the build",
        ))
    }

    fn arg_quiet(self) -> Self {
        self._arg(flag("quiet", "Do not print cargo log messages").short('q'))
    }

    fn arg_timings(self) -> Self {
        self._arg(
            optional_opt(
                "timings",
                "Timing output formats (unstable) (comma separated): html, json",
            )
            .value_name("FMTS")
            .require_equals(true),
        )
    }
}

impl AppExt for App<'static> {
    fn _arg(self, arg: Arg<'static>) -> Self {
        self.arg(arg)
    }
}

fn flag(name: &'static str, help: &'static str) -> Arg<'static> {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

pub fn opt(name: &'static str, help: &'static str) -> Arg<'static> {
    Arg::new(name).long(name).help(help)
}

pub fn optional_opt(name: &'static str, help: &'static str) -> Arg<'static> {
    opt(name, help).min_values(0)
}
pub fn optional_multi_opt(
    name: &'static str,
    value_name: &'static str,
    help: &'static str,
) -> Arg<'static> {
    opt(name, help)
        .value_name(value_name)
        .action(ArgAction::Append)
        .multiple_values(true)
        .min_values(0)
        .number_of_values(1)
}

pub fn multi_opt(name: &'static str, value_name: &'static str, help: &'static str) -> Arg<'static> {
    opt(name, help)
        .value_name(value_name)
        .action(ArgAction::Append)
}

pub fn subcommand(name: &'static str) -> App {
    App::new(name)
        .dont_collapse_args_in_usage(true)
        .setting(AppSettings::DeriveDisplayOrder)
}

#[test]
fn cargo_add() {
    let mut app = clap::Command::new("add")
            .setting(clap::AppSettings::DeriveDisplayOrder)
            .about("Add dependencies to a Cargo.toml manifest file")
            .override_usage(
                "\
        cargo add [OPTIONS] <DEP>[@<VERSION>] ...
        cargo add [OPTIONS] --path <PATH> ...
        cargo add [OPTIONS] --git <URL> ..."
            )
            .after_help("Run `cargo help add` for more detailed information.\n")
            .group(clap::ArgGroup::new("selected").multiple(true).required(true))
            .args([
                clap::Arg::new("crates")
                    .takes_value(true)
                    .value_name("DEP_ID")
                    .multiple_values(true)
                    .help("Reference to a package to add as a dependency")
                    .long_help(
                    "Reference to a package to add as a dependency
    You can reference a package by:
    - `<name>`, like `cargo add serde` (latest version will be used)
    - `<name>@<version-req>`, like `cargo add serde@1` or `cargo add serde@=1.0.38`"
                )
                    .group("selected"),
                flag("no-default-features",
                    "Disable the default features"),
                flag("default-features",
                    "Re-enable the default features")
                    .overrides_with("no-default-features"),
                clap::Arg::new("features")
                    .short('F')
                    .long("features")
                    .takes_value(true)
                    .value_name("FEATURES")
                    .action(ArgAction::Append)
                    .help("Space or comma separated list of features to activate"),
                flag("optional",
                    "Mark the dependency as optional")
                    .long_help("Mark the dependency as optional
    The package name will be exposed as feature of your crate.")
                    .conflicts_with("dev"),
                flag("no-optional",
                    "Mark the dependency as required")
                    .long_help("Mark the dependency as required
    The package will be removed from your features.")
                    .conflicts_with("dev")
                    .overrides_with("optional"),
                clap::Arg::new("rename")
                    .long("rename")
                    .takes_value(true)
                    .value_name("NAME")
                    .help("Rename the dependency")
                    .long_help("Rename the dependency
    Example uses:
    - Depending on multiple versions of a crate
    - Depend on crates with the same name from different registries"),
            ])
            .arg_manifest_path()
            .args([
                clap::Arg::new("package")
                    .short('p')
                    .long("package")
                    .takes_value(true)
                    .value_name("SPEC")
                    .help("Package to modify"),
            ])
            .arg_quiet()
            .arg_dry_run("Don't actually write the manifest")
            .next_help_heading("SOURCE")
            .args([
                clap::Arg::new("path")
                    .long("path")
                    .takes_value(true)
                    .value_name("PATH")
                    .help("Filesystem path to local crate to add")
                    .group("selected")
                    .conflicts_with("git"),
                clap::Arg::new("git")
                    .long("git")
                    .takes_value(true)
                    .value_name("URI")
                    .help("Git repository location")
                    .long_help("Git repository location
    Without any other information, cargo will use latest commit on the main branch.")
                    .group("selected"),
                clap::Arg::new("branch")
                    .long("branch")
                    .takes_value(true)
                    .value_name("BRANCH")
                    .help("Git branch to download the crate from")
                    .requires("git")
                    .group("git-ref"),
                clap::Arg::new("tag")
                    .long("tag")
                    .takes_value(true)
                    .value_name("TAG")
                    .help("Git tag to download the crate from")
                    .requires("git")
                    .group("git-ref"),
                clap::Arg::new("rev")
                    .long("rev")
                    .takes_value(true)
                    .value_name("REV")
                    .help("Git reference to download the crate from")
                    .long_help("Git reference to download the crate from
    This is the catch all, handling hashes to named references in remote repositories.")
                    .requires("git")
                    .group("git-ref"),
                clap::Arg::new("registry")
                    .long("registry")
                    .takes_value(true)
                    .value_name("NAME")
                    .help("Package registry for this dependency"),
            ])
            .next_help_heading("SECTION")
            .args([
                flag("dev",
                    "Add as development dependency")
                    .long_help("Add as development dependency
    Dev-dependencies are not used when compiling a package for building, but are used for compiling tests, examples, and benchmarks.
    These dependencies are not propagated to other packages which depend on this package.")
                    .group("section"),
                flag("build",
                    "Add as build dependency")
                    .long_help("Add as build dependency
    Build-dependencies are the only dependencies available for use by build scripts (`build.rs` files).")
                    .group("section"),
                clap::Arg::new("target")
                    .long("target")
                    .takes_value(true)
                    .value_name("TARGET")
                    .value_parser(clap::builder::NonEmptyStringValueParser::new())
                    .help("Add as dependency to the given target platform")
            ])
    ;

    tests!("carg-add", app);
}
