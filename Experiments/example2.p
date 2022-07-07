fof(a, axiom, ! [X, U] : (e(X, U) => ((~w(X)) & w(U)))).
fof(b, axiom, ! [X, U] : ((e(X, U) => e(X, f1(U))))).
fof(b, axiom, ! [X, U] : (e(X, U) => e(X, f2(U)))).
fof(b, axiom, ! [X] : ? [Y] : (e(X, Y) | e(Y, X))).

fof(b, axiom, ! [X, U] : ((p(X, U) => p(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p(X, U) => p(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p(X, U) => p(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((p1(X, U) => p1(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p1(X, U) => p1(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p1(X, U) => p1(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((p2(X, U) => p2(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p2(X, U) => p2(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p2(X, U) => p2(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((p3(X, U) => p3(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p3(X, U) => p3(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p3(X, U) => p3(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((p4(X, U) => p4(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p4(X, U) => p4(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p4(X, U) => p4(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((p5(X, U) => p5(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p5(X, U) => p5(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p5(X, U) => p5(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((p6(X, U) => p6(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((p6(X, U) => p6(X, f1(U))))).
fof(b, axiom, ! [X, U] : (p6(X, U) => p6(X, f2(U)))).
fof(b, axiom, ! [X, U] : ((a(X, U) => a(X, f0(U))))).
fof(b, axiom, ! [X, U] : ((a(X, U) => a(X, f1(U))))).
fof(b, axiom, ! [X, U] : (a(X, U) => a(X, f2(U)))).

fof(b, axiom, ! [U] : ((p1(f0(U)) => p2(f0(U))) => p(U))).
fof(b, axiom, ! [U] : (p1(U) => ! [X] : (e(T, U) => p3(X, U)))).
fof(b, axiom, ! [T, U] : (e(T, U) => (~(p3(T, U) & p4(T, U))))).
fof(b, axiom, ! [T, U] : (e(T, U) => ((~a(T, U)) => p4(T, U)))).
fof(b, axiom, ! [U] : ((~(p5(f1(U)))) => p2(U))).
fof(b, axiom, ! [U] : (~(p5(U) & p6(U)))).
fof(b, axiom, ! [U] : ((! [X] : (e(T, U) => a(X, f2(U)))) => p6(U))).
fof(d, conjecture, ? [X] : p(s)).
