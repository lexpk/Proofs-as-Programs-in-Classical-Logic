fof(add_z, axiom, ! [X] : add(z, X) = X).
fof(add_s, axiom, ! [X, Y] : add(s(X), Y) = s(add(X, Y))).
fof(hint, axiom, ! [X, Y] : (add(X, Y) = add(Y, X))).
fof(def_p, axiom, ! [X, Y] : (p(X, Y) <=> (add(Y, Y) = X | s(add(Y, Y)) = X))).
fof(ind_p, axiom,
    ((? [Y] : p(z, Y)) &
    (! [X] : ((? [Y] : p(X, Y)) => (? [Y] : p(s(X), Y)))))
    =>
    (! [X] : ? [Y] : p(X, Y))
).
fof(c, conjecture, ! [X] : ? [Y] : p(X, Y)).
