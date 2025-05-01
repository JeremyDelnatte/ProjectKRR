blocks(1).
blocks(2).
blocks(3).
blocks(4).
blocks(5).
blocks(6).
blocks(7).
blocks(8).
blocks(9).
blocks(10).
width(3).
height(3).
depth(3).
block_size(2,1).
block_size(3,1).
block_size(4,1).
block_size(5,1).
block_size(6,1).
block_size(8,2).
block_size(9,2).
block_size(10,2).
block_size(7,3).
block_size(1,13).

cell(1..W, 1..H, 1..D) :- width(W), height(H), depth(D).

adjacent(X, Y, Z, X2, Y2, Z2, Dir) :- cell(X, Y, Z), dir(Dir, DX, DY, DZ), X2 = X + DX, Y2 = Y + DY, Z2 = Z + DZ, cell(X2, Y2, Z2).

{ cell_value(X, Y, Z, B, 1) : cell(X, Y, Z) } = 1 :- blocks(B).
:- cell_value(X, Y, Z, B1, 1), cell_value(X, Y, Z, B2, 1), B1 != B2.

{ cell_value_adjacent(X2, Y2, Z2, B, N2, N1, Dir) : cell_value(X, Y, Z, B, N1), N2=2..Size, block_size(B, Size), N1 != N2, adjacent(X, Y, Z, X2, Y2, Z2, Dir) } = 1 :- cell(X, Y, Z).
:- block_size(B, Size), N=1..Size, not cell_value(_, _, _, B, N).
:- block_size(B, Size), N=1..Size, #count { X, Y, Z : cell_value(X, Y, Z, B, N) } != 1.

cell_value(X, Y, Z, B, N) :- cell_value_adjacent(X, Y, Z, B, N, _, _).
% adjacent(B, N1, N2, Dir) :- cell_value_adjacent(_, _, _, B, N2, N1, Dir).

dir(a,  0,  1,  0).
dir(b,  0, -1,  0).
dir(n,  0,  0,  1).
dir(s,  0,  0, -1).
dir(e,  1,  0,  0).
dir(w, -1,  0,  0).

% rot((0; 90; 180; 270)).
%
% rot_dir(Dir, R, x, Dir) :- Dir = (e; w), rot(R).
% rot_dir(Dir, R, y, Dir) :- Dir = (a; b), rot(R).
% rot_dir(Dir, R, z, Dir) :- Dir = (n; s), rot(R).
%
% rot_dir(a, 90, x, s).
% rot_dir(s, 90, x, b).
% rot_dir(b, 90, x, n).
% rot_dir(n, 90, x, a).
%
% rot_dir(n, 90, y, e).
% rot_dir(e, 90, y, s).
% rot_dir(s, 90, y, w).
% rot_dir(w, 90, y, n).
%
% rot_dir(a, 90, z, e).
% rot_dir(e, 90, z, b).
% rot_dir(b, 90, z, w).
% rot_dir(w, 90, z, a).
%
% rot_dir(Dir1, 180, Axis, Dir3) :- rot_dir(Dir1, 90, Axis, Dir2), rot_dir(Dir2, 90, Axis, Dir3).
% rot_dir(Dir1, 270, Axis, Dir2) :- rot_dir(Dir2, 90, Axis, Dir1).
% rot_dir(Dir, 0, Axis, Dir) :- Dir=(a; b; n; e; s; w), Axis = (x; y; z).
%
% { rot(Axis, B, R) : rot(R) } = 1 :- blocks(B), Axis = (x; y; z).
%
% rot_x(B, N1, N2, Dir') :- rot(x, B, R), rot_dir(Dir, R, x, Dir'), adjacent(B, N1, N2, Dir).
% rot_y(B, N1, N2, Dir') :- rot(y, B, R), rot_dir(Dir, R, y, Dir'), rot_x(B, N1, N2, Dir).
% block(B, N1, N2, Dir') :- rot(z, B, R), rot_dir(Dir, R, z, Dir'), rot_y(B, N1, N2, Dir).

% #show block/4.
% #show adjacent/4.
#show cell_value/5.
% #show adjacent/4.
% #show block_size/2.
