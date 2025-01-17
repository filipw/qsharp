// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#![allow(clippy::too_many_lines)]

use expect_test::{expect, Expect};
use indoc::indoc;
use qsc_frontend::compile::{self, compile, PackageStore, SourceMap};
use qsc_hir::mut_visit::MutVisitor;

use crate::loop_unification::LoopUni;

fn check(file: &str, expect: &Expect) {
    let store = PackageStore::new(compile::core());
    let sources = SourceMap::new([("test".into(), file.into())], None);
    let mut unit = compile(&store, &[], sources);
    assert!(unit.errors.is_empty(), "{:?}", unit.errors);
    LoopUni {
        core: store.core(),
        assigner: &mut unit.assigner,
    }
    .visit_package(&mut unit.package);
    expect.assert_eq(&unit.package.to_string());
}

#[test]
fn convert_for_array() {
    check(
        indoc! {r#"
        namespace test {
            operation Main(arr : Int[]) : Unit {
                for i in arr {
                    let x = "Hello World";
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-133] (Public):
                    Namespace (Ident 16 [10-14] "test"): Item 1
                Item 1 [21-131] (Public):
                    Parent: 0
                    Callable 0 [21-131] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [36-47] [Type (Int)[]]: Bind: Ident 3 [36-39] "arr"
                        output: Unit
                        functors: empty set
                        body: SpecDecl 4 [21-131]: Impl:
                            Block 5 [56-131] [Type Unit]:
                                Stmt 6 [66-125]: Expr: Expr _id_ [66-125] [Type Unit]: Expr Block: Block _id_ [66-125] [Type Unit]:
                                    Stmt _id_ [75-78]: Local (Immutable):
                                        Pat _id_ [75-78] [Type (Int)[]]: Bind: Ident 17 [75-78] "array_id_17"
                                        Expr 10 [75-78] [Type (Int)[]]: Var: Local 3
                                    Stmt _id_ [75-78]: Local (Immutable):
                                        Pat _id_ [75-78] [Type Int]: Bind: Ident 18 [75-78] "len_id_18"
                                        Expr _id_ [75-78] [Type (Int)[]]: Call:
                                            Expr _id_ [75-78] [Type ((Int)[] -> Int)]: Var:
                                                res: Item 1 (Package 0)
                                                generics:
                                                    Int
                                            Expr _id_ [75-78] [Type (Int)[]]: Var: Local 17
                                    Stmt _id_ [75-78]: Local (Mutable):
                                        Pat _id_ [75-78] [Type Int]: Bind: Ident 19 [75-78] "index_id_19"
                                        Expr _id_ [75-78] [Type Int]: Lit: Int(0)
                                    Stmt _id_ [66-125]: Expr: Expr _id_ [66-125] [Type Unit]: While:
                                        Expr _id_ [75-78] [Type Bool]: BinOp (Lt):
                                            Expr _id_ [75-78] [Type Int]: Var: Local 19
                                            Expr _id_ [75-78] [Type Int]: Var: Local 18
                                        Block 11 [79-125] [Type Unit]:
                                            Stmt _id_ [66-125]: Local (Immutable):
                                                Pat 8 [70-71] [Type Int]: Bind: Ident 9 [70-71] "i"
                                                Expr _id_ [75-78] [Type Int]: Index:
                                                    Expr _id_ [75-78] [Type (Int)[]]: Var: Local 17
                                                    Expr _id_ [75-78] [Type Int]: Var: Local 19
                                            Stmt 12 [93-115]: Local (Immutable):
                                                Pat 13 [97-98] [Type String]: Bind: Ident 14 [97-98] "x"
                                                Expr 15 [101-114] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [75-78]: Semi: Expr _id_ [75-78] [Type Unit]: AssignOp (Add):
                                                Expr _id_ [75-78] [Type Int]: Var: Local 19
                                                Expr _id_ [75-78] [Type Int]: Lit: Int(1)
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_for_array_deconstruct() {
    check(
        indoc! {r#"
        namespace test {
            operation Main(arr : (Int, Double)[]) : Unit {
                for (i, d) in arr {
                    let x = "Hello World";
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-148] (Public):
                    Namespace (Ident 19 [10-14] "test"): Item 1
                Item 1 [21-146] (Public):
                    Parent: 0
                    Callable 0 [21-146] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [36-57] [Type ((Int, Double))[]]: Bind: Ident 3 [36-39] "arr"
                        output: Unit
                        functors: empty set
                        body: SpecDecl 4 [21-146]: Impl:
                            Block 5 [66-146] [Type Unit]:
                                Stmt 6 [76-140]: Expr: Expr _id_ [76-140] [Type Unit]: Expr Block: Block _id_ [76-140] [Type Unit]:
                                    Stmt _id_ [90-93]: Local (Immutable):
                                        Pat _id_ [90-93] [Type ((Int, Double))[]]: Bind: Ident 20 [90-93] "array_id_20"
                                        Expr 13 [90-93] [Type ((Int, Double))[]]: Var: Local 3
                                    Stmt _id_ [90-93]: Local (Immutable):
                                        Pat _id_ [90-93] [Type Int]: Bind: Ident 21 [90-93] "len_id_21"
                                        Expr _id_ [90-93] [Type ((Int, Double))[]]: Call:
                                            Expr _id_ [90-93] [Type (((Int, Double))[] -> Int)]: Var:
                                                res: Item 1 (Package 0)
                                                generics:
                                                    (Int, Double)
                                            Expr _id_ [90-93] [Type ((Int, Double))[]]: Var: Local 20
                                    Stmt _id_ [90-93]: Local (Mutable):
                                        Pat _id_ [90-93] [Type Int]: Bind: Ident 22 [90-93] "index_id_22"
                                        Expr _id_ [90-93] [Type Int]: Lit: Int(0)
                                    Stmt _id_ [76-140]: Expr: Expr _id_ [76-140] [Type Unit]: While:
                                        Expr _id_ [90-93] [Type Bool]: BinOp (Lt):
                                            Expr _id_ [90-93] [Type Int]: Var: Local 22
                                            Expr _id_ [90-93] [Type Int]: Var: Local 21
                                        Block 14 [94-140] [Type Unit]:
                                            Stmt _id_ [76-140]: Local (Immutable):
                                                Pat 8 [80-86] [Type (Int, Double)]: Tuple:
                                                    Pat 9 [81-82] [Type Int]: Bind: Ident 10 [81-82] "i"
                                                    Pat 11 [84-85] [Type Double]: Bind: Ident 12 [84-85] "d"
                                                Expr _id_ [90-93] [Type (Int, Double)]: Index:
                                                    Expr _id_ [90-93] [Type ((Int, Double))[]]: Var: Local 20
                                                    Expr _id_ [90-93] [Type Int]: Var: Local 22
                                            Stmt 15 [108-130]: Local (Immutable):
                                                Pat 16 [112-113] [Type String]: Bind: Ident 17 [112-113] "x"
                                                Expr 18 [116-129] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [90-93]: Semi: Expr _id_ [90-93] [Type Unit]: AssignOp (Add):
                                                Expr _id_ [90-93] [Type Int]: Var: Local 22
                                                Expr _id_ [90-93] [Type Int]: Lit: Int(1)
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_for_slice() {
    check(
        indoc! {r#"
        namespace test {
            operation Main(arr : Int[]) : Unit {
                for i in arr[6..-2..2] {
                    let x = "Hello World";
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-143] (Public):
                    Namespace (Ident 22 [10-14] "test"): Item 1
                Item 1 [21-141] (Public):
                    Parent: 0
                    Callable 0 [21-141] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [36-47] [Type (Int)[]]: Bind: Ident 3 [36-39] "arr"
                        output: Unit
                        functors: empty set
                        body: SpecDecl 4 [21-141]: Impl:
                            Block 5 [56-141] [Type Unit]:
                                Stmt 6 [66-135]: Expr: Expr _id_ [66-135] [Type Unit]: Expr Block: Block _id_ [66-135] [Type Unit]:
                                    Stmt _id_ [75-88]: Local (Immutable):
                                        Pat _id_ [75-88] [Type (Int)[]]: Bind: Ident 23 [75-88] "array_id_23"
                                        Expr 10 [75-88] [Type (Int)[]]: Index:
                                            Expr 11 [75-78] [Type (Int)[]]: Var: Local 3
                                            Expr 12 [79-87] [Type Range]: Range:
                                                Expr 13 [79-80] [Type Int]: Lit: Int(6)
                                                Expr 14 [82-84] [Type Int]: UnOp (Neg):
                                                    Expr 15 [83-84] [Type Int]: Lit: Int(2)
                                                Expr 16 [86-87] [Type Int]: Lit: Int(2)
                                    Stmt _id_ [75-88]: Local (Immutable):
                                        Pat _id_ [75-88] [Type Int]: Bind: Ident 24 [75-88] "len_id_24"
                                        Expr _id_ [75-88] [Type (Int)[]]: Call:
                                            Expr _id_ [75-88] [Type ((Int)[] -> Int)]: Var:
                                                res: Item 1 (Package 0)
                                                generics:
                                                    Int
                                            Expr _id_ [75-88] [Type (Int)[]]: Var: Local 23
                                    Stmt _id_ [75-88]: Local (Mutable):
                                        Pat _id_ [75-88] [Type Int]: Bind: Ident 25 [75-88] "index_id_25"
                                        Expr _id_ [75-88] [Type Int]: Lit: Int(0)
                                    Stmt _id_ [66-135]: Expr: Expr _id_ [66-135] [Type Unit]: While:
                                        Expr _id_ [75-88] [Type Bool]: BinOp (Lt):
                                            Expr _id_ [75-88] [Type Int]: Var: Local 25
                                            Expr _id_ [75-88] [Type Int]: Var: Local 24
                                        Block 17 [89-135] [Type Unit]:
                                            Stmt _id_ [66-135]: Local (Immutable):
                                                Pat 8 [70-71] [Type Int]: Bind: Ident 9 [70-71] "i"
                                                Expr _id_ [75-88] [Type Int]: Index:
                                                    Expr _id_ [75-88] [Type (Int)[]]: Var: Local 23
                                                    Expr _id_ [75-88] [Type Int]: Var: Local 25
                                            Stmt 18 [103-125]: Local (Immutable):
                                                Pat 19 [107-108] [Type String]: Bind: Ident 20 [107-108] "x"
                                                Expr 21 [111-124] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [75-88]: Semi: Expr _id_ [75-88] [Type Unit]: AssignOp (Add):
                                                Expr _id_ [75-88] [Type Int]: Var: Local 25
                                                Expr _id_ [75-88] [Type Int]: Lit: Int(1)
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_for_range() {
    check(
        indoc! {r#"
        namespace test {
            operation Main() : Unit {
                for i in 0..4 {
                    let x = "Hello World";
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-123] (Public):
                    Namespace (Ident 17 [10-14] "test"): Item 1
                Item 1 [21-121] (Public):
                    Parent: 0
                    Callable 0 [21-121] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [35-37] [Type Unit]: Unit
                        output: Unit
                        functors: empty set
                        body: SpecDecl 3 [21-121]: Impl:
                            Block 4 [45-121] [Type Unit]:
                                Stmt 5 [55-115]: Expr: Expr _id_ [55-115] [Type Unit]: Expr Block: Block _id_ [55-115] [Type Unit]:
                                    Stmt _id_ [64-68]: Local (Immutable):
                                        Pat _id_ [64-68] [Type Range]: Bind: Ident 18 [64-68] "range_id_18"
                                        Expr 9 [64-68] [Type Range]: Range:
                                            Expr 10 [64-65] [Type Int]: Lit: Int(0)
                                            <no step>
                                            Expr 11 [67-68] [Type Int]: Lit: Int(4)
                                    Stmt _id_ [64-68]: Local (Mutable):
                                        Pat _id_ [64-68] [Type Int]: Bind: Ident 19 [64-68] "index_id_19"
                                        Expr _id_ [64-68] [Type Int]: Field:
                                            Expr _id_ [64-68] [Type Range]: Var: Local 18
                                            Prim(Start)
                                    Stmt _id_ [64-68]: Local (Immutable):
                                        Pat _id_ [64-68] [Type Int]: Bind: Ident 20 [64-68] "step_id_20"
                                        Expr _id_ [64-68] [Type Int]: Field:
                                            Expr _id_ [64-68] [Type Range]: Var: Local 18
                                            Prim(Step)
                                    Stmt _id_ [64-68]: Local (Immutable):
                                        Pat _id_ [64-68] [Type Int]: Bind: Ident 21 [64-68] "end_id_21"
                                        Expr _id_ [64-68] [Type Int]: Field:
                                            Expr _id_ [64-68] [Type Range]: Var: Local 18
                                            Prim(End)
                                    Stmt _id_ [55-115]: Expr: Expr _id_ [55-115] [Type Unit]: While:
                                        Expr _id_ [64-68] [Type Bool]: BinOp (OrL):
                                            Expr _id_ [64-68] [Type Bool]: BinOp (AndL):
                                                Expr _id_ [64-68] [Type Bool]: BinOp (Gt):
                                                    Expr _id_ [64-68] [Type Int]: Var: Local 20
                                                    Expr _id_ [64-68] [Type Int]: Lit: Int(0)
                                                Expr _id_ [64-68] [Type Bool]: BinOp (Lte):
                                                    Expr _id_ [64-68] [Type Int]: Var: Local 19
                                                    Expr _id_ [64-68] [Type Int]: Var: Local 21
                                            Expr _id_ [64-68] [Type Bool]: BinOp (AndL):
                                                Expr _id_ [64-68] [Type Bool]: BinOp (Lt):
                                                    Expr _id_ [64-68] [Type Int]: Var: Local 20
                                                    Expr _id_ [64-68] [Type Int]: Lit: Int(0)
                                                Expr _id_ [64-68] [Type Bool]: BinOp (Gte):
                                                    Expr _id_ [64-68] [Type Int]: Var: Local 19
                                                    Expr _id_ [64-68] [Type Int]: Var: Local 21
                                        Block 12 [69-115] [Type Unit]:
                                            Stmt _id_ [55-115]: Local (Immutable):
                                                Pat 7 [59-60] [Type Int]: Bind: Ident 8 [59-60] "i"
                                                Expr _id_ [64-68] [Type Int]: Var: Local 19
                                            Stmt 13 [83-105]: Local (Immutable):
                                                Pat 14 [87-88] [Type String]: Bind: Ident 15 [87-88] "x"
                                                Expr 16 [91-104] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [64-68]: Semi: Expr _id_ [64-68] [Type Unit]: AssignOp (Add):
                                                Expr _id_ [64-68] [Type Int]: Var: Local 19
                                                Expr _id_ [64-68] [Type Int]: Var: Local 20
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_for_reverse_range() {
    check(
        indoc! {r#"
        namespace test {
            operation Main() : Unit {
                for i in 4..-1..0 {
                    let x = "Hello World";
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-127] (Public):
                    Namespace (Ident 19 [10-14] "test"): Item 1
                Item 1 [21-125] (Public):
                    Parent: 0
                    Callable 0 [21-125] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [35-37] [Type Unit]: Unit
                        output: Unit
                        functors: empty set
                        body: SpecDecl 3 [21-125]: Impl:
                            Block 4 [45-125] [Type Unit]:
                                Stmt 5 [55-119]: Expr: Expr _id_ [55-119] [Type Unit]: Expr Block: Block _id_ [55-119] [Type Unit]:
                                    Stmt _id_ [64-72]: Local (Immutable):
                                        Pat _id_ [64-72] [Type Range]: Bind: Ident 20 [64-72] "range_id_20"
                                        Expr 9 [64-72] [Type Range]: Range:
                                            Expr 10 [64-65] [Type Int]: Lit: Int(4)
                                            Expr 11 [67-69] [Type Int]: UnOp (Neg):
                                                Expr 12 [68-69] [Type Int]: Lit: Int(1)
                                            Expr 13 [71-72] [Type Int]: Lit: Int(0)
                                    Stmt _id_ [64-72]: Local (Mutable):
                                        Pat _id_ [64-72] [Type Int]: Bind: Ident 21 [64-72] "index_id_21"
                                        Expr _id_ [64-72] [Type Int]: Field:
                                            Expr _id_ [64-72] [Type Range]: Var: Local 20
                                            Prim(Start)
                                    Stmt _id_ [64-72]: Local (Immutable):
                                        Pat _id_ [64-72] [Type Int]: Bind: Ident 22 [64-72] "step_id_22"
                                        Expr _id_ [64-72] [Type Int]: Field:
                                            Expr _id_ [64-72] [Type Range]: Var: Local 20
                                            Prim(Step)
                                    Stmt _id_ [64-72]: Local (Immutable):
                                        Pat _id_ [64-72] [Type Int]: Bind: Ident 23 [64-72] "end_id_23"
                                        Expr _id_ [64-72] [Type Int]: Field:
                                            Expr _id_ [64-72] [Type Range]: Var: Local 20
                                            Prim(End)
                                    Stmt _id_ [55-119]: Expr: Expr _id_ [55-119] [Type Unit]: While:
                                        Expr _id_ [64-72] [Type Bool]: BinOp (OrL):
                                            Expr _id_ [64-72] [Type Bool]: BinOp (AndL):
                                                Expr _id_ [64-72] [Type Bool]: BinOp (Gt):
                                                    Expr _id_ [64-72] [Type Int]: Var: Local 22
                                                    Expr _id_ [64-72] [Type Int]: Lit: Int(0)
                                                Expr _id_ [64-72] [Type Bool]: BinOp (Lte):
                                                    Expr _id_ [64-72] [Type Int]: Var: Local 21
                                                    Expr _id_ [64-72] [Type Int]: Var: Local 23
                                            Expr _id_ [64-72] [Type Bool]: BinOp (AndL):
                                                Expr _id_ [64-72] [Type Bool]: BinOp (Lt):
                                                    Expr _id_ [64-72] [Type Int]: Var: Local 22
                                                    Expr _id_ [64-72] [Type Int]: Lit: Int(0)
                                                Expr _id_ [64-72] [Type Bool]: BinOp (Gte):
                                                    Expr _id_ [64-72] [Type Int]: Var: Local 21
                                                    Expr _id_ [64-72] [Type Int]: Var: Local 23
                                        Block 14 [73-119] [Type Unit]:
                                            Stmt _id_ [55-119]: Local (Immutable):
                                                Pat 7 [59-60] [Type Int]: Bind: Ident 8 [59-60] "i"
                                                Expr _id_ [64-72] [Type Int]: Var: Local 21
                                            Stmt 15 [87-109]: Local (Immutable):
                                                Pat 16 [91-92] [Type String]: Bind: Ident 17 [91-92] "x"
                                                Expr 18 [95-108] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [64-72]: Semi: Expr _id_ [64-72] [Type Unit]: AssignOp (Add):
                                                Expr _id_ [64-72] [Type Int]: Var: Local 21
                                                Expr _id_ [64-72] [Type Int]: Var: Local 22
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_repeat() {
    check(
        indoc! {r#"
        namespace test {
            operation Main() : Unit {
                repeat {
                    let x = "Hello World";
                } until true;
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-128] (Public):
                    Namespace (Ident 13 [10-14] "test"): Item 1
                Item 1 [21-126] (Public):
                    Parent: 0
                    Callable 0 [21-126] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [35-37] [Type Unit]: Unit
                        output: Unit
                        functors: empty set
                        body: SpecDecl 3 [21-126]: Impl:
                            Block 4 [45-126] [Type Unit]:
                                Stmt 5 [55-120]: Semi: Expr _id_ [55-119] [Type Unit]: Expr Block: Block _id_ [55-119] [Type Unit]:
                                    Stmt _id_ [115-119]: Local (Mutable):
                                        Pat _id_ [115-119] [Type Bool]: Bind: Ident 14 [115-119] "continue_cond_14"
                                        Expr _id_ [115-119] [Type Bool]: Lit: Bool(true)
                                    Stmt _id_ [55-119]: Expr: Expr _id_ [55-119] [Type Unit]: While:
                                        Expr _id_ [115-119] [Type Bool]: Var: Local 14
                                        Block 7 [62-108] [Type Unit]:
                                            Stmt 8 [76-98]: Local (Immutable):
                                                Pat 9 [80-81] [Type String]: Bind: Ident 10 [80-81] "x"
                                                Expr 11 [84-97] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [115-119]: Semi: Expr _id_ [115-119] [Type Unit]: Assign:
                                                Expr _id_ [115-119] [Type Bool]: Var: Local 14
                                                Expr _id_ [115-119] [Type Bool]: UnOp (NotL):
                                                    Expr 12 [115-119] [Type Bool]: Lit: Bool(true)
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_repeat_fixup() {
    check(
        indoc! {r#"
        namespace test {
            operation Main() : Unit {
                repeat {
                    let x = "Hello World";
                } until true
                fixup {
                    let y = "Fixup";
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-182] (Public):
                    Namespace (Ident 18 [10-14] "test"): Item 1
                Item 1 [21-180] (Public):
                    Parent: 0
                    Callable 0 [21-180] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [35-37] [Type Unit]: Unit
                        output: Unit
                        functors: empty set
                        body: SpecDecl 3 [21-180]: Impl:
                            Block 4 [45-180] [Type Unit]:
                                Stmt 5 [55-174]: Expr: Expr _id_ [55-174] [Type Unit]: Expr Block: Block _id_ [55-174] [Type Unit]:
                                    Stmt _id_ [115-119]: Local (Mutable):
                                        Pat _id_ [115-119] [Type Bool]: Bind: Ident 19 [115-119] "continue_cond_19"
                                        Expr _id_ [115-119] [Type Bool]: Lit: Bool(true)
                                    Stmt _id_ [55-174]: Expr: Expr _id_ [55-174] [Type Unit]: While:
                                        Expr _id_ [115-119] [Type Bool]: Var: Local 19
                                        Block 7 [62-108] [Type Unit]:
                                            Stmt 8 [76-98]: Local (Immutable):
                                                Pat 9 [80-81] [Type String]: Bind: Ident 10 [80-81] "x"
                                                Expr 11 [84-97] [Type String]: String:
                                                    Lit: "Hello World"
                                            Stmt _id_ [115-119]: Semi: Expr _id_ [115-119] [Type Unit]: Assign:
                                                Expr _id_ [115-119] [Type Bool]: Var: Local 19
                                                Expr _id_ [115-119] [Type Bool]: UnOp (NotL):
                                                    Expr 12 [115-119] [Type Bool]: Lit: Bool(true)
                                            Stmt _id_ [134-174]: Expr: Expr _id_ [134-174] [Type Unit]: If:
                                                Expr _id_ [115-119] [Type Bool]: Var: Local 19
                                                Expr _id_ [134-174] [Type Unit]: Expr Block: Block 13 [134-174] [Type Unit]:
                                                    Stmt 14 [148-164]: Local (Immutable):
                                                        Pat 15 [152-153] [Type String]: Bind: Ident 16 [152-153] "y"
                                                        Expr 17 [156-163] [Type String]: String:
                                                            Lit: "Fixup"
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}

#[test]
fn convert_repeat_nested() {
    check(
        indoc! {r#"
        namespace test {
            operation Main() : Unit {
                let a = true;
                let b = false;
                let c = true;
                repeat {
                    repeat {
                        let x = "First";
                    } until a
                    fixup {
                        let y = "Second";
                    }
                } until b
                fixup {
                    repeat {
                        let z = "Third";
                    } until c;
                }
            }
        }
        "#},
        &expect![[r#"
            Package:
                Item 0 [0-403] (Public):
                    Namespace (Ident 43 [10-14] "test"): Item 1
                Item 1 [21-401] (Public):
                    Parent: 0
                    Callable 0 [21-401] (operation):
                        name: Ident 1 [31-35] "Main"
                        input: Pat 2 [35-37] [Type Unit]: Unit
                        output: Unit
                        functors: empty set
                        body: SpecDecl 3 [21-401]: Impl:
                            Block 4 [45-401] [Type Unit]:
                                Stmt 5 [55-68]: Local (Immutable):
                                    Pat 6 [59-60] [Type Bool]: Bind: Ident 7 [59-60] "a"
                                    Expr 8 [63-67] [Type Bool]: Lit: Bool(true)
                                Stmt 9 [77-91]: Local (Immutable):
                                    Pat 10 [81-82] [Type Bool]: Bind: Ident 11 [81-82] "b"
                                    Expr 12 [85-90] [Type Bool]: Lit: Bool(false)
                                Stmt 13 [100-113]: Local (Immutable):
                                    Pat 14 [104-105] [Type Bool]: Bind: Ident 15 [104-105] "c"
                                    Expr 16 [108-112] [Type Bool]: Lit: Bool(true)
                                Stmt 17 [122-395]: Expr: Expr _id_ [122-395] [Type Unit]: Expr Block: Block _id_ [122-395] [Type Unit]:
                                    Stmt _id_ [291-292]: Local (Mutable):
                                        Pat _id_ [291-292] [Type Bool]: Bind: Ident 46 [291-292] "continue_cond_46"
                                        Expr _id_ [291-292] [Type Bool]: Lit: Bool(true)
                                    Stmt _id_ [122-395]: Expr: Expr _id_ [122-395] [Type Unit]: While:
                                        Expr _id_ [291-292] [Type Bool]: Var: Local 46
                                        Block 19 [129-284] [Type Unit]:
                                            Stmt 20 [143-274]: Expr: Expr _id_ [143-274] [Type Unit]: Expr Block: Block _id_ [143-274] [Type Unit]:
                                                Stmt _id_ [205-206]: Local (Mutable):
                                                    Pat _id_ [205-206] [Type Bool]: Bind: Ident 44 [205-206] "continue_cond_44"
                                                    Expr _id_ [205-206] [Type Bool]: Lit: Bool(true)
                                                Stmt _id_ [143-274]: Expr: Expr _id_ [143-274] [Type Unit]: While:
                                                    Expr _id_ [205-206] [Type Bool]: Var: Local 44
                                                    Block 22 [150-198] [Type Unit]:
                                                        Stmt 23 [168-184]: Local (Immutable):
                                                            Pat 24 [172-173] [Type String]: Bind: Ident 25 [172-173] "x"
                                                            Expr 26 [176-183] [Type String]: String:
                                                                Lit: "First"
                                                        Stmt _id_ [205-206]: Semi: Expr _id_ [205-206] [Type Unit]: Assign:
                                                            Expr _id_ [205-206] [Type Bool]: Var: Local 44
                                                            Expr _id_ [205-206] [Type Bool]: UnOp (NotL):
                                                                Expr 27 [205-206] [Type Bool]: Var: Local 7
                                                        Stmt _id_ [225-274]: Expr: Expr _id_ [225-274] [Type Unit]: If:
                                                            Expr _id_ [205-206] [Type Bool]: Var: Local 44
                                                            Expr _id_ [225-274] [Type Unit]: Expr Block: Block 28 [225-274] [Type Unit]:
                                                                Stmt 29 [243-260]: Local (Immutable):
                                                                    Pat 30 [247-248] [Type String]: Bind: Ident 31 [247-248] "y"
                                                                    Expr 32 [251-259] [Type String]: String:
                                                                        Lit: "Second"
                                            Stmt _id_ [291-292]: Semi: Expr _id_ [291-292] [Type Unit]: Assign:
                                                Expr _id_ [291-292] [Type Bool]: Var: Local 46
                                                Expr _id_ [291-292] [Type Bool]: UnOp (NotL):
                                                    Expr 33 [291-292] [Type Bool]: Var: Local 11
                                            Stmt _id_ [307-395]: Expr: Expr _id_ [307-395] [Type Unit]: If:
                                                Expr _id_ [291-292] [Type Bool]: Var: Local 46
                                                Expr _id_ [307-395] [Type Unit]: Expr Block: Block 34 [307-395] [Type Unit]:
                                                    Stmt 35 [321-385]: Semi: Expr _id_ [321-384] [Type Unit]: Expr Block: Block _id_ [321-384] [Type Unit]:
                                                        Stmt _id_ [383-384]: Local (Mutable):
                                                            Pat _id_ [383-384] [Type Bool]: Bind: Ident 45 [383-384] "continue_cond_45"
                                                            Expr _id_ [383-384] [Type Bool]: Lit: Bool(true)
                                                        Stmt _id_ [321-384]: Expr: Expr _id_ [321-384] [Type Unit]: While:
                                                            Expr _id_ [383-384] [Type Bool]: Var: Local 45
                                                            Block 37 [328-376] [Type Unit]:
                                                                Stmt 38 [346-362]: Local (Immutable):
                                                                    Pat 39 [350-351] [Type String]: Bind: Ident 40 [350-351] "z"
                                                                    Expr 41 [354-361] [Type String]: String:
                                                                        Lit: "Third"
                                                                Stmt _id_ [383-384]: Semi: Expr _id_ [383-384] [Type Unit]: Assign:
                                                                    Expr _id_ [383-384] [Type Bool]: Var: Local 45
                                                                    Expr _id_ [383-384] [Type Bool]: UnOp (NotL):
                                                                        Expr 42 [383-384] [Type Bool]: Var: Local 15
                        adj: <none>
                        ctl: <none>
                        ctl-adj: <none>"#]],
    );
}
