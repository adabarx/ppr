/**
 * @file parser for paper—ppr—projects
 * @author Adamina Barx <adaminabarx@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "ppr",

  rules: {
    comp_set_var: $ => seq('@', /[a-zA-Z\d]+/, '=', seq(repeat1(choice($.word, $.punctuation))), '\n'),

    comp_get_var: $ => seq('@', /[a-zA-Z\d]+/),

    bookmark: $ => seq(
      '[',
      choice(
        $._content,
        $.bookmark,
      ),
      ']'
    ),

    bold: $ => seq(
      '**',
      $._content,
      '**'
    ),

    italic: $ => seq(
      '//', 
      $._content,
      '//'
    ),

    underline: $ => seq(
      '__',
      $._content,
      '__'
    ),

    strikethrough: $ => seq(
      '--',
      $._content,
      '--'
    ),

    word: $ => /[a-zA-Z\d']+/,

    linked_word: $ => seq(repeat1(seq($.word, '-')), $.word),

    sentence: $ => seq(
      repeat1(choice(
        $.word,
        $.linked_word,
        $.punctuation,
      )),
      $.sentence_end
    ),

    paragraph: $ => seq(repeat1($.sentence), '\n'),

    title: $ => seq('#', '!', $._content),

    heading: $ => seq(repeat1('#'), $._content),

    footnote: $ => seq(
      '{',
      $._content,
      '}', '^', '{',
      $._content,
      '}',
    ),

    sentence_end: $ => choice(
      $.period,
      $.question_mark,
      $.exclamation_point,
    ),

    punctuation: $ => choice(
      $.semicolon,
      $.colon,
      $.comma,
      $.emdash,
    ),

    period: $ => '.',

    question_mark: $ => '?',

    exclamation_point: $ => '!',

    semicolon: $ => ';',

    colon: $ => ':',

    comma: $ => ',',

    emdash: $ => '-',

    _content: $ => seq(repeat1(choice(
      $.word,
      $.linked_word,
      $.punctuation,
      $.sentence_end,
    ))),
  }
});
