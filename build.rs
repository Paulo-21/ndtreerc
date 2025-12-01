//println!("cargo:rustc-link-lib=dylib=stdc++"); // This line may be unnecessary for some environment.
//println!("cargo:rustc-link-search=NDTree-C");
fn main() {
    cc::Build::new()
        .file("NDTree-C/treeNDS.c")
        .file("NDTree-C/tabNDS.c")
        .file("NDTree-C/tabSol.c")
        .file("NDTree-C/dominance.c")
        .include("NDTree-C")
        .compile("ndtree"); // nom de la lib générée
    let sources = [
        "NDTree-C/treeNDS.c",
        "NDTree-C/tabNDS.c",
        "NDTree-C/tabSol.c",
        "NDTree-C/dominance.c",
    ];

    for file in &sources {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Si tu as des headers dans NDTree-C
    println!("cargo:rerun-if-changed=NDTree-C");
}
