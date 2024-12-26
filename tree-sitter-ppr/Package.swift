// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterPPR",
    products: [
        .library(name: "TreeSitterPPR", targets: ["TreeSitterPPR"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterPPR",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                // NOTE: if your language has an external scanner, add it here.
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterPPRTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterPPR",
            ],
            path: "bindings/swift/TreeSitterPPRTests"
        )
    ],
    cLanguageStandard: .c11
)
