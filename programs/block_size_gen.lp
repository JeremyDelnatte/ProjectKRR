width(4).
height(4).
depth(4).
blocks(1..13).

{ block_size(B, Size) : Size = 1..H*W*D, width(W), height(H), depth(D) } = 1 :- blocks(B).
:- #sum { Size, B : block_size(B, Size) } != H*W*D, width(W), height(H), depth(D).
