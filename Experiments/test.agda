module test where
open import Data.Sum.Base
postulate
    X : Set
    a : X
    b : X
    p : X → X → Set
    φ : (x : X) → (p x a ⊎ p x b)

example : (x : X) → 
