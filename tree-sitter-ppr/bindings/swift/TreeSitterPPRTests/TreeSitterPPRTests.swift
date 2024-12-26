import XCTest
import SwiftTreeSitter
import TreeSitterPPR

final class TreeSitterPPRTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_ppr())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading PPR grammar")
    }
}
