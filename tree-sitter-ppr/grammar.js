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
    comp_set_var: $ => seq('@', /[a-zA-Z\d]+/, '=', '"', seq(repeat1(choice($.word, $.punctuation))), '"', '\n'),

    comp_get_var: $ => seq('@', /[a-zA-Z\d]+/),

    bookmark: $ => seq(
      '[',
      choice(
        repeat1($.word),
        $.bookmark,
      ),
      ']'
    ),

    bold: $ => seq('**', seq(repeat1(choice($.word, $.punctuation))), '**'),

    italic: $ => seq('*/', seq(repeat1(choice($.word, $.punctuation))), '/*'),

    underline: $ => seq('*_', seq(repeat1(choice($.word, $.punctuation))), '_*'),

    strikethrough: $ => seq('*—', seq(repeat1(choice($.word, $.punctuation))), '—*'),

    word: $ => /[a-zA-Z\d']+/,

    sentence: $ => seq(repeat1(choice(
      $.word,
      $.semicolon,
      $.colon,
      $.comma,
      $.question_mark,
      $.exclaimation_point,
      $.emdash,
    )), $.period),

    paragraph: $ => seq(repeat1($.sentence), '\n'),

    title: $ => seq('#', '!', $.paragraph),

    heading: $ => seq('#', repeat('#'), $.paragraph),

    footnote: $ => seq(
      '{',
      seq(repeat1(choice($.word, $.punctuation))),
      '}', '^', '{',
      seq(repeat1(choice($.word, $.punctuation))),
      '}',
    ),

    punctuation: $ => choice(
      $.semicolon,
      $.colon,
      $.comma,
      $.period,
      $.question_mark,
      $.exclaimation_point,
      $.emdash,
    ),

    semicolon: $ => ';',

    colon: $ => ':',

    comma: $ => ',',

    period: $ => '.',

    question_mark: $ => '?',

    exclaimation_point: $ => '!',

    emdash: $ => '—',
  }
});
