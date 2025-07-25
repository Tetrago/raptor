#!/usr/bin/env python3

import re
import pathlib

from railroad import *


def snake_case(value):
    return '_'.join(
        re.sub('([A-Z][a-z]+)', r' \1',
               re.sub('([A-Z]+)', r' \1',
                      value.replace('-', ' '))).split()).lower()


STYLE = DEFAULT_STYLE + """\
        svg.railroad-diagram .identifier > rect {
          fill: hsl(210, 80%, 85%);
        }
        svg.railroad-diagram .literal > rect {
          fill: hsl(60, 80%, 85%);
        }
        svg.railroad-diagram .operator > rect {
          fill: hsl(350, 60%, 85%);
        }
        svg.railroad-diagram .separator > rect {
          fill: hsl(30, 70%, 85%);
        }
"""


def identifier(value=None):
    if value is None:
        return Terminal("Identifier", cls="identifier")
    else:
        return Terminal(value, cls="identifier")


def literal():
    return Terminal("Literal", cls="literal")


def operator(value=None):
    if value is None:
        return Terminal("Operator", cls="operator")
    else:
        return Terminal(value, cls="operator")


def separator(value):
    return Terminal(value, cls="separator")


def item(value):
    return NonTerminal(value)


class Document:

    def __init__(self, output_file):
        self._directory = pathlib.Path(output_file).parent.resolve()
        self._file = open(output_file, "w")

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def close(self):
        if not self._file.closed:
            self._file.close()

    def add(self, name, contents):
        filename = f"{snake_case(name)}.svg"
        path = self._directory / filename

        with open(path, "w") as file:
            Diagram(contents).writeStandalone(file.write, css=STYLE)

        self._file.write(f"## {name}\n\n![](./{filename})\n\n")


DIAGRAMS = {
    "Generic":
    Sequence(operator("<"), ZeroOrMore(item("Type"), separator(",")),
             operator(">")),
    "Type":
    Sequence(
        OneOrMore(Sequence(identifier(), Optional(item("Generic"), skip=True)),
                  operator("::")),
        ZeroOrMore(Sequence(Optional("const", skip=True), operator("*"))),
        Choice(1, operator("?"), Skip(), operator("!"))),
    "Unary Expression":
    Choice(0, Sequence(separator("("), item("Expression"), separator(")")),
           identifier(), literal()),
    "Primary Expression":
    Choice(
        0,
        Group(Sequence(operator(), item("Unary Expression")),
              "Unary Operation"),
        Group(
            Sequence(item("Unary Expression"),
                     Optional(item("Generic"), skip=True), separator("("),
                     ZeroOrMore(item("Singular Expression"), separator(",")),
                     separator(")")), "Invocation"),
        Sequence(item("Unary Expression"),
                 Choice(1, operator("!"), Skip(), operator("?"))),
    ),
    "Singular Expression":
    Choice(
        0,
        Group(
            Sequence(item("Primary Expression"),
                     Choice(0, operator("*"), operator("/")),
                     item("Singular Expression")), "Geometric Operation"),
        Group(
            Sequence(item("Primary Expression"),
                     Choice(0, operator("+"), operator("-")),
                     item("Singular Expression")), "Arithmetic Operation"),
        Group(
            Sequence(
                item("Primary Expression"),
                HorizontalChoice(operator(">"), operator("<"), operator(">="),
                                 operator("<="),
                                 operator("=="), operator("!=")),
                item("Singular Expression")), "Inequality"),
        Group(
            Sequence(item("Primary Expression"),
                     Choice(0, operator("||"), operator("&&")),
                     item("Singular Expression")), "Boolean Operation"),
        Group(
            Sequence(item("Primary Expression"), operator("?"),
                     item("Singular Expression"), operator(":"),
                     item("Singular Expression")), "Ternary Operation"),
        Group(
            Sequence(item("Primary Expression"), operator(),
                     item("Singular Expression")), "Binary Operation")),
    "Expression":
    OneOrMore(item("Singular Expression"), separator(",")),
    "Label":
    Sequence(operator("'"), identifier(), separator(":")),
    "Statement":
    Choice(
        0,
        Sequence(separator("{"), ZeroOrMore(item("Statement")),
                 separator("}")),
        Sequence(identifier("break"),
                 Optional(Group(identifier(), "Label"), skip=True),
                 separator(";")),
        separator(";"),
        Sequence(Optional(item("Label"), skip=True), identifier("do"),
                 item("Statement"), identifier("while"), separator("("),
                 item("Expression"), separator(")"), separator(";")),
        Sequence(item("Expression"), separator(";")),
        Sequence(Optional(item("Label"), skip=True), identifier("for"),
                 separator("("), item("Statement"), item("Expression"),
                 separator(";"), item("Expression"), separator(")"),
                 item("Statement")),
        Sequence(
            Choice(0, identifier("let"), identifier("const")),
            OneOrMore(
                identifier(),
                Optional(Group(Sequence(operator(":"), item("Type")),
                               "Value Type"),
                         skip=True)), operator("="),
            OneOrMore(item("Expression")), separator(";")),
        Sequence(Optional(item("Label"), skip=True), identifier("loop"),
                 item("Statement")),
        Sequence(identifier("return"), item("Expression"), separator(";")),
        Sequence(identifier("if"), separator("("), item("Expression"),
                 separator(")"), item("Statement")),
        Sequence(Optional(item("Label"), skip=True), identifier("until"),
                 separator("("), item("Expression"), separator(")"),
                 item("Statement")),
        Sequence(Optional(item("Label"), skip=True), identifier("while"),
                 separator("("), item("Expression"), separator(")"),
                 item("Statement")),
    ),
    "Function":
    Sequence(
        Optional(identifier("pub"), skip=True),
        Stack(identifier("fn"), identifier(),
              Optional(item("Generic"), skip=True), separator("(")),
        ZeroOrMore(
            Group(Sequence(identifier(), operator(":"), item("Type")),
                  "Parameter"), separator(",")), separator(")"),
        Optional(Group(Sequence(operator(":"), item("Type")), "Value Type")),
        item("Statement")),
    "Struct":
    Sequence(
        Optional(identifier("pub"), skip=True),
        Stack(identifier("struct"), identifier(),
              Optional(item("Generic"), skip=True), separator("{")),
        ZeroOrMore(
            Group(
                Sequence(Optional(identifier("pub"), skip=True), identifier(),
                         operator(":"), item("Type")), "Field"),
            separator(",")), separator("}")),
    "File":
    ZeroOrMore(Choice(0, item("Function"), item("Struct")))
}

if __name__ == '__main__':
    dir = pathlib.Path(__file__).parent.resolve()
    file = dir / "README.md"

    with Document(file) as doc:
        for name, contents in DIAGRAMS.items():
            doc.add(name, contents)
