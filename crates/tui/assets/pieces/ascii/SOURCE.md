# ASCII piece masks

This directory contains an independent 40-by-40 mask for every ASCII piece.
They derive from the “Chess Pieces in .svg Format” set by OpenGameArt user
`femrek`:

https://opengameart.org/content/chess-pieces-in-svg-format

The source artwork is dedicated to the public domain under CC0 1.0. The masks
normalize the six SVG silhouettes to a shared canvas for sampling into opaque
two-by-two Unicode block cells.

The queen currently restores the detailed five-point silhouette for visual
comparison, mirroring its original right half to keep the result symmetric. The
pawn, rook, and king are also exactly mirrored; the knight is vertically
normalized; and the bishop retains its longer flat base.
