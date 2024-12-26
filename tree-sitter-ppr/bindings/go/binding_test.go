package tree_sitter_ppr_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_ppr "github.com/tree-sitter/tree-sitter-ppr/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_ppr.Language())
	if language == nil {
		t.Errorf("Error loading PPR grammar")
	}
}
