// compile-flags: -Z borrowck=mir

fn guard() -> bool {
    false
}

fn guard2(_:i32) -> bool {
    true
}

// no_mangle to make sure this gets instantiated even in an executable.
#[no_mangle]
pub fn full_tested_match() {
    let _ = match Some(42) {
        Some(x) if guard() => (1, x),
        Some(y) => (2, y),
        None => (3, 3),
    };
}

// no_mangle to make sure this gets instantiated even in an executable.
#[no_mangle]
pub fn full_tested_match2() {
    let _ = match Some(42) {
        Some(x) if guard() => (1, x),
        None => (3, 3),
        Some(y) => (2, y),
    };
}

fn main() {
    let _ = match Some(1) {
        Some(_w) if guard() => 1,
        _x => 2,
        Some(y) if guard2(y) => 3,
        _z => 4,
    };
}

// END RUST SOURCE
//
// START rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
//  bb0: {
//      ...
//      _2 = std::option::Option::<i32>::Some(const 42i32,);
//      FakeRead(ForMatchedPlace, _2);
//      _3 = discriminant(_2);
//      switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb5];
//  }
//  bb1 (cleanup): {
//      resume;
//  }
//  bb2: {
//      falseEdges -> [real: bb6, imaginary: bb3]; //pre_binding1
//  }
//  bb3: {
//      falseEdges -> [real: bb9, imaginary: bb4]; //pre_binding2
//  }
//  bb4: {                                         //pre_binding3 and arm3
//      _1 = (const 3i32, const 3i32);
//      goto -> bb10;
//  }
//  bb5: {
//      unreachable;
//  }
//  bb6: { // binding1 and guard
//      StorageLive(_6);
//      _6 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
//      _4 = &shallow _2;
//      StorageLive(_7);
//      _7 = const guard() -> [return: bb7, unwind: bb1];
//  }
//  bb7: {
//      FakeRead(ForMatchGuard, _4);
//      FakeRead(ForGuardBinding, _6);
//      switchInt(move _7) -> [false: bb3, otherwise: bb8];
//  }
//  bb8: {
//      StorageLive(_5);
//      _5 = ((_2 as Some).0: i32);
//      StorageLive(_8);
//      _8 = _5;
//      _1 = (const 1i32, move _8);
//      StorageDead(_8);
//      goto -> bb10;
//  }
//  bb9: {
//      StorageLive(_9);
//      _9 = ((_2 as Some).0: i32);
//      StorageLive(_10);
//      _10 = _9;
//      _1 = (const 2i32, move _10);
//      StorageDead(_10);
//      goto -> bb10;
//  }
//  bb10: {
//      ...
//      return;
//  }
// END rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
//
// START rustc.full_tested_match2.QualifyAndPromoteConstants.before.mir
//  bb0: {
//      ...
//      _2 = std::option::Option::<i32>::Some(const 42i32,);
//      FakeRead(ForMatchedPlace, _2);
//      _3 = discriminant(_2);
//      switchInt(move _3) -> [0isize: bb3, 1isize: bb2, otherwise: bb5];
//  }
//  bb1 (cleanup): {
//      resume;
//  }
//  bb2: {
//      falseEdges -> [real: bb6, imaginary: bb3];
//  }
//  bb3: {
//      falseEdges -> [real: bb9, imaginary: bb10];
//  }
//  bb4: { // to arm3 (can skip 2 since this is `Some`)
//      falseEdges -> [real: bb10, imaginary: bb3];
//  }
//  bb5: {
//      unreachable;
//  }
//  bb6: { // binding1 and guard
//      StorageLive(_6);
//      _6 = &((_2 as Some).0: i32);
//      _4 = &shallow _2;
//      StorageLive(_7);
//      _7 = const guard() -> [return: bb7, unwind: bb1];
//  }
//  bb7: { // end of guard
//      FakeRead(ForMatchGuard, _4);
//      FakeRead(ForGuardBinding, _6);
//      switchInt(move _7) -> [false: bb4, otherwise: bb8];
//  }
//  bb8: { // arm1
//      StorageLive(_5);
//      _5 = ((_2 as Some).0: i32);
//      StorageLive(_8);
//      _8 = _5;
//      _1 = (const 1i32, move _8);
//      StorageDead(_8);
//      goto -> bb11;
//  }
//  bb9: { // arm2
//      _1 = (const 3i32, const 3i32);
//      goto -> bb11;
//  }
//  bb10: { // binding3 and arm3
//      StorageLive(_9);
//      _9 = ((_2 as Some).0: i32);
//      StorageLive(_10);
//      _10 = _9;
//      _1 = (const 2i32, move _10);
//      StorageDead(_10);
//      goto -> bb11;
//  }
//  bb11: {
//      ...
//      return;
//  }
// END rustc.full_tested_match2.QualifyAndPromoteConstants.before.mir
//
// START rustc.main.QualifyAndPromoteConstants.before.mir
// bb0: {
//     ...
//      _2 = std::option::Option::<i32>::Some(const 1i32,);
//      FakeRead(ForMatchedPlace, _2);
//      _4 = discriminant(_2);
//      switchInt(move _4) -> [1isize: bb2, otherwise: bb3];
//  }
//  bb1 (cleanup): {
//      resume;
//  }
//  bb2: {
//      falseEdges -> [real: bb7, imaginary: bb3];
//  }
//  bb3: {
//      falseEdges -> [real: bb10, imaginary: bb4];
//  }
//  bb4: {
//      falseEdges -> [real: bb11, imaginary: bb14];
//  }
//  bb5: {
//      falseEdges -> [real: bb3, imaginary: bb3];
//  }
//  bb6: {
//      falseEdges -> [real: bb14, imaginary: bb14];
//  }
//  bb7: { // binding1: Some(w) if guard()
//      StorageLive(_7);
//      _7 = &((_2 as Some).0: i32);
//      _5 = &shallow _2;
//      StorageLive(_8);
//      _8 = const guard() -> [return: bb8, unwind: bb1];
//  }
//  bb8: { //end of guard
//      FakeRead(ForMatchGuard, _5);
//      FakeRead(ForGuardBinding, _7);
//      switchInt(move _8) -> [false: bb5, otherwise: bb9];
//  }
//  bb9: { // set up bindings for arm1
//      StorageLive(_6);
//      _6 = ((_2 as Some).0: i32);
//      _1 = const 1i32;
//      goto -> bb15;
//  }
//  bb10: { // binding2 & arm2
//      StorageLive(_9);
//      _9 = _2;
//      _1 = const 2i32;
//      goto -> bb15;
//  }
//  bb11: { // binding3: Some(y) if guard2(y)
//      StorageLive(_11);
//      _11 = &((_2 as Some).0: i32);
//      _5 = &shallow _2;
//      StorageLive(_12);
//      StorageLive(_13);
//      _13 = (*_11);
//      _12 = const guard2(move _13) -> [return: bb12, unwind: bb1];
//  }
//  bb12: { // end of guard2
//      StorageDead(_13);
//      FakeRead(ForMatchGuard, _5);
//      FakeRead(ForGuardBinding, _11);
//      switchInt(move _12) -> [false: bb6, otherwise: bb13];
//  }
//  bb13: { // binding4 & arm4
//      StorageLive(_10);
//      _10 = ((_2 as Some).0: i32);
//      _1 = const 3i32;
//      goto -> bb15;
//  }
//  bb14: {
//      StorageLive(_14);
//      _14 = _2;
//      _1 = const 4i32;
//      goto -> bb15;
//  }
//  bb15: {
//      ...
//      return;
//  }
// END rustc.main.QualifyAndPromoteConstants.before.mir
