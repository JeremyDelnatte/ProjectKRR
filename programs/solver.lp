cell(1..W, 1..H, 1..D) :- width(W), height(H), depth(D).

{ block_pos(X, Y, Z, B, 1) : cell(X, Y, Z) } = 1 :- block(B).

{ rot(Axis, B, R) : rot(R) } = 1 :- block(B), Axis = (x; y; z).

rot((0; 90; 180; 270)).

dir(a,  0,  1,  0).
dir(b,  0, -1,  0).
dir(n,  0,  0,  1).
dir(s,  0,  0, -1).
dir(e,  1,  0,  0).
dir(w, -1,  0,  0).

dir_oposite(Dir1, Dir2) :- rot_dir(Dir1, 180, _, Dir2), Dir1 != Dir2.

rot_dir(Dir, R, x, Dir) :- Dir = (e; w), rot(R).
rot_dir(Dir, R, y, Dir) :- Dir = (a; b), rot(R).
rot_dir(Dir, R, z, Dir) :- Dir = (n; s), rot(R).

rot_dir(a, 90, x, s).
rot_dir(s, 90, x, b).
rot_dir(b, 90, x, n).
rot_dir(n, 90, x, a).

rot_dir(n, 90, y, e).
rot_dir(e, 90, y, s).
rot_dir(s, 90, y, w).
rot_dir(w, 90, y, n).

rot_dir(a, 90, z, e).
rot_dir(e, 90, z, b).
rot_dir(b, 90, z, w).
rot_dir(w, 90, z, a).

rot_dir(Dir1, 180, Axis, Dir3) :- rot_dir(Dir1, 90, Axis, Dir2), rot_dir(Dir2, 90, Axis, Dir3).
rot_dir(Dir1, 270, Axis, Dir2) :- rot_dir(Dir2, 90, Axis, Dir1).
rot_dir(Dir, 0, Axis, Dir) :- Dir=(a; b; n; e; s; w), Axis = (x; y; z).

rot_x(B, N1, N2, Dir') :- rot(x, B, R), rot_dir(Dir, R, x, Dir'), block(B, N1, N2, Dir).
rot_y(B, N1, N2, Dir') :- rot(y, B, R), rot_dir(Dir, R, y, Dir'), rot_x(B, N1, N2, Dir).
rot_block(B, N1, N2, Dir') :- rot(z, B, R), rot_dir(Dir, R, z, Dir'), rot_y(B, N1, N2, Dir).

pipe_rot_x(B, N, In', Out') :- rot(x, B, R), rot_dir(In, R, x, In'), rot_dir(Out, R, x, Out'), pipe(B, N, In, Out).
pipe_rot_y(B, N, In', Out') :- rot(y, B, R), rot_dir(In, R, y, In'), rot_dir(Out, R, y, Out'), pipe_rot_x(B, N, In, Out).
rot_pipe(B, N, In', Out') :- rot(z, B, R), rot_dir(In, R, z, In'), rot_dir(Out, R, z, Out'), pipe_rot_y(B, N, In, Out).

block_pos(X + DX, Y + DY, Z + DZ, B, N2) :- block_pos(X, Y, Z, B, N1), rot_block(B, N1, N2, Dir), dir(Dir, DX, DY, DZ).
pipe_pos(X, Y, Z, In, Out) :- rot_pipe(B, N, In, Out), block_pos(X, Y, Z, B, N).

% :- pos(W + 1, _, _, _, _), width(W).
% :- pos(0, _, _, _, _).
%
% :- pos(_, H + 1, _, _, _), height(H).
% :- pos(_, 0, _, _, _).
%
% :- pos(_, _, D + 1, _, _), depth(D).
% :- pos(_, _, 0, _, _).

:- block_pos(X, Y, Z, _, _), not cell(X, Y, Z).

:- block_pos(X, Y, Z, B1, _), block_pos(X, Y, Z, B2, _), B1 != B2.


adjacent(X, Y, Z, X2, Y2, Z2, Dir) :- cell(X, Y, Z), dir(Dir, DX, DY, DZ), X2 = X + DX, Y2 = Y + DY, Z2 = Z + DZ.

pipe_pos(X, Y, Z, Out, In) :- pipe_pos(X, Y, Z, In, Out).

:- pipe_pos(X, Y, Z, _, Out), adjacent(X, Y, Z, X2, Y2, Z2, Out), not pipe_pos(X2, Y2, Z2, In, _), dir_oposite(In, Out), not pipe_in(X, Y, Z, _), not pipe_out(X, Y, Z, _).

:- pipe_in(X, Y, Z, In), not pipe_pos(X, Y, Z, In, _).
:- pipe_out(X, Y, Z, Out), not pipe_pos(X, Y, Z, _, Out).

% #show block_pos/5.
% #show pipe_pos/5.
