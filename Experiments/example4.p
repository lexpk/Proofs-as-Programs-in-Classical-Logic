fof(dn, axiom, ! [X] : (q(X) | ~q(X))).
fof(q1, axiom, ! [X] : (p(X) => q(X))).
fof(q1, axiom, ! [X, Y] : (q(X) => (p(X) | q(Y)))).
fof(c, conjecture, ! [X] : p(X) | ? [X] : ~q(X)).