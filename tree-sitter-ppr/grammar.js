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
    document: $ => repeat(choice(
      $.function,
      $.title,
      $.heading,
      $.bookmark,
      $.paragraph,
    )),

    title: $ => seq('\\', 'T',
      seq(
        repeat($.sentence),
        optional($.fragment),
      )
    ), 
    heading: $ => seq('\\', choice($.h1, $.h2, $.h3, $.h4, $.h5, $.h6),
      seq(
        repeat($.sentence),
        optional($.fragment),
      )
    ),
    bookmark: $ => seq('\\', 'B',
      seq(
        repeat($.sentence),
        optional($.fragment),
      )
    ), 
    paragraph: $ => seq('\\', 'P', repeat($.sentence), optional($.fragment)),

    identifier: $ => seq(/[a-z]/, /[a-zA-Z\d]*/),
    parameter: $ => seq(/[a-z]/, /[a-zA-Z\d]*/),
    value: $ => seq('"', /[^"]+/, '"'),
    function: $ => seq('\\', $.identifier, '(', repeat(seq($.parameter, ':', $.value, ',')), optional(seq($.parameter, ':', $.value)), ')'),

    //word: $ => /[a-zA-Z'\d]+(-[a-zA-Z'\d]+)*/,
    word: $ => /[a-zA-Z'\d]+/,
    hyphen: $ => seq($.word, repeat1(seq('-', $.word))),

    style_marker: $ => choice(
      $.bold,
      $.italics,
      $.underline,
      $.reference,
    ),

    sentence: $ => prec(3, seq(repeat1(choice(
      $.hyphen,
      $.word,
      $.punctuation,
      $._surround,
      $.style_marker,
      $.other,
    )), $.end)),
    
    fragment: $ => prec(2, repeat1(choice(
      $.hyphen,
      $.word,
      $.punctuation,
      $._surround,
      $.style_marker,
      $.other,
    ))),
    
    end: $ => choice(
      $.period,
      $.question_mark,
      $.exclamation_point,
    ),

    punctuation: $ => choice(
      $.comma,
      $.semicolon,
      $.colon,
      $.fslash,
      $.emdash
    ),

    _surround: $ => choice(
      $.quote,
      $.parentheses,
    ),

    quote: $ => prec.left(seq("\"", repeat($.sentence), optional($.fragment), "\"")),

    parentheses: $ => prec.left(seq("(", repeat($.sentence), optional($.fragment), ")")),

    other: $ => choice('@', '$', '%', '^', '&', '*', '&', '_', '+', '=', '~', '`'),

    h1: $ => 'H1',

    h2: $ => 'H2',

    h3: $ => 'H3',

    h4: $ => 'H4',

    h5: $ => 'H5',

    h6: $ => 'H6',

    period: $ => '.',

    comma: $ => ',',

    semicolon: $ => ';',

    colon: $ => ':',

    fslash: $ => '/',

    question_mark: $ => '?',

    exclamation_point: $ => '!',

    bold: $ => '**',

    italics: $ => '//',

    emdash: $ => '--',

    underline: $ => '__',

    reference: $ => '&&',

  }
});
