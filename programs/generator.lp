
% These are the parametors for the size of the puzzle.
width(width).
height(height).
depth(depth).

cell_count(C) :- width(W), height(H), depth(D), C=W*H*D.

% Block size generation
% =====================

% Define the minimum number of blocks to have in average less than 6 cells per block.
min_block_count(Min) :- cell_count(C), Min=C/6, Min >= 2.
min_block_count(Min) :- cell_count(C), C/6 < 2, Min=2.

% Define the maximum number of blocks to have in average more than 3 cells per block.
max_block_count(Max) :- cell_count(C), min_block_count(Min), Max=2 * Min.

% We choose the number of blocks between the minimum and maximum allowed.
{ block_count(C) : min_block_count(Min), max_block_count(Max), C=Min..Max } = 1.

block(1..C) :- block_count(C).

% We choose the size of all the blocks and make sure that the sum of all the sizes
% is equal to the number of cells.
{ block_size(B, Size) : Size = 2..C, cell_count(C) } = 1 :- block(B).
:- #sum { Size, B : block_size(B, Size) } != C, cell_count(C).

% Constraint to limit the number of blocks of the same size. We want to have at most
% 30% of the blocks with the same size. This is to avoid having too many blocks of
% the same size. This is only applied when the grid size is big enough (> 12 cells).
same_size(S,N) :- N = #count { B : block_size(B,S) }, block_size(_, S).
:- same_size(S,N), block_count(BC), cell_count(CC), CC > 12, 10*N > 3*BC.

% Constraint to limit the gap between the minimum and maximum block size.
% This avoids having blocks that are too small or too big.
% This is only applied when the grid size is big enough (> 12 cells).
min_block_size(M) :- M = #min { S, B : block_size(B,S) }, M = 2..C, cell_count(C).
max_block_size(M) :- M = #max { S, B : block_size(B,S) }, M = 2..C, cell_count(C).
:- Max - Min > BC, block_count(BC), cell_count(CC), CC > 12,
    min_block_size(Min), max_block_size(Max).

% Block shape generation
% ======================

% Blocks are defined as a set of unit cubes that are connected together.
% Each unit cube of a block will have a unique number {1, ..., block size}.

% Defines the different directions from a cell
% to its neighbors with relative positions.
dir(a,  0,  1,  0). % Above
dir(b,  0, -1,  0). % Below
dir(n,  0,  0,  1). % North
dir(s,  0,  0, -1). % South
dir(e,  1,  0,  0). % East
dir(w, -1,  0,  0). % West
dir(Dir) :- dir(Dir, _, _, _).

cell(1..W, 1..H, 1..D) :- width(W), height(H), depth(D).

% We choose an initial position for the unit cube 1 for each block.
{ cell_block(X, Y, Z, B, 1) : cell(X, Y, Z) } = 1 :- block(B).

% Gives the adjacent cell (X2, Y2, Z2) of a cell (X, Y, Z) in a given direction Dir.
adjacent_cell(X, Y, Z, X2, Y2, Z2, Dir) :-
    cell(X, Y, Z), dir(Dir, DX, DY, DZ), X2 = X + DX,
    Y2 = Y + DY, Z2 = Z + DZ, cell(X2, Y2, Z2).

% We choose an adjacent cell (X2, Y2, Z2) in a direction Dir of an already placed
% unit cube N1 to place another one N2 for the block B and do it for each unit
% cube in all the block. Using a lower bound 0 and upper bound 1 with a contraint
% was faster than using = 1.
0 {
    cell_block_adjacent(X2, Y2, Z2, B, N2, N1, Dir) :
        adjacent_cell(X, Y, Z, X2, Y2, Z2, Dir), cell_block(X, Y, Z, B, N1)
} 1 :- block_size(B, Size), N2=2..Size.

% We make sure that all the unit cubes of a block are placed. 
:- block_size(B, Size), N=1..Size,
    #count { X, Y, Z : cell_block(X, Y, Z, B, N) } != 1.

cell_block(X, Y, Z, B, N) :- cell_block_adjacent(X, Y, Z, B, N, _, _).

% Defines an adjacent unit cube N1 of another unit cube N2 of a block B in the
% direction Dir. This is what actually defines the shape of the blocks for the
% solver. It defines how the unit cubes are connected together.
adjacent_block(B, N1, N2, Dir) :- cell_block_adjacent(_, _, _, B, N2, N1, Dir).

% We make sure that unit cubes of a block do not overlap with each other.
:- cell_block(X, Y, Z, B, N1), cell_block(X, Y, Z, B, N2), N1 != N2.

% We make sure that two different blocks do not overlap with each other.
:- cell_block(X, Y, Z, B1, _), cell_block(X, Y, Z, B2, _), B1 != B2.

% Pipe generation
% ===============

% The pipe is defined as a sequence of pipe segments that are connected together.
% Pipe segments have a direction In and Out for simplicity, but they are not
% directional. So if we swap the In with the Out, it is exactly the same.
% They are also numbered, from 1 to the number of segments in the pipe,
% in order of placement.

% Gives the cells (X, Y, Z) that are adjacent to the
% border of the grid in a given direction Dir.
adjacent_border(X, Y, Z, Dir) :- cell(X, Y, Z), dir(Dir, DX, DY, DZ), X2 = X + DX,
    Y2 = Y + DY, Z2 = Z + DZ, not cell(X2, Y2, Z2).

% We choose the position of the pipe's extremities (X, Y, Z).
% The direction Dir is the direction to a cell outside the grid.
{ pipe_in(X, Y, Z, Dir) : adjacent_border(X, Y, Z, Dir) } = 1.
{ pipe_out(X, Y, Z, Dir) : adjacent_border(X, Y, Z, Dir) } = 1.

% Gives the opposite direction Dir2 of a direction Dir1.
dir_opposite(Dir1, Dir2) :- rot_dir(Dir1, 180, _, Dir2), Dir1 != Dir2.

% Chooses the direction Out of the first pipe segment positionned at one end of
% the pipe. The direction In is the same as the one of the pipe's extremity.
{ pipe_unrotated(X, Y, Z, In, Out, 1) : dir(Out), Out != In } = 1 :-
    pipe_in(X, Y, Z, In).

% Positions the next pipe segment in the direction Out of the previous one.
% Chooses the direction Out of this pipe segment.
0 {
    pipe_unrotated(X, Y, Z, In, Out, N) :
        pipe_unrotated(X2, Y2, Z2, _, Out2, N-1),
        adjacent_cell(X, Y, Z, X2, Y2, Z2, In),
        dir_opposite(In, Out2), dir(Out), Out != In
} 1 :- N=2..C, cell_count(C).

% We make sure that there is a pipe segment at the other end of the pipe.
:- pipe_out(X, Y, Z, Out), not pipe_unrotated(X, Y, Z, _, Out, _).

% We make sure that there are not pipe segments that overlap with each other.
:- pipe_unrotated(X, Y, Z, _, _, N1), pipe_unrotated(X, Y, Z, _, _, N2), N1 != N2.

% Connects the pipe segments to the blocks, by attaching a pipe
% segment to the unit cube of block at the same position.
pipe_unrotated(B, N, In, Out) :-
    pipe_unrotated(X, Y, Z, In, Out, _), cell_block(X, Y, Z, B, N).

% We make sure that each block has at least one pipe segment attached to it.
:- block(B), not pipe_unrotated(B, _, _, _).

% Rotation generation
% ===================

% The blocks and pipe segments are rotated to avoid giving the good rotation to
% the solver. We want to make it harder for the solver to find the solution.

% Defines the 4 possible rotations on a axis.
rot((0; 90; 180; 270)).

% rot_dir(Dir, R, Axis, Dir') defines the rotation of
% a direction Dir by an angle R on the axis Axis.
rot_dir(Dir, R, x, Dir) :- Dir = (e; w), rot(R).
rot_dir(Dir, R, y, Dir) :- Dir = (a; b), rot(R).
rot_dir(Dir, R, z, Dir) :- Dir = (n; s), rot(R).

rot_dir(a, 90, x, s). rot_dir(n, 90, y, e). rot_dir(a, 90, z, e).
rot_dir(s, 90, x, b). rot_dir(e, 90, y, s). rot_dir(e, 90, z, b).
rot_dir(b, 90, x, n). rot_dir(s, 90, y, w). rot_dir(b, 90, z, w).
rot_dir(n, 90, x, a). rot_dir(w, 90, y, n). rot_dir(w, 90, z, a).

rot_dir(Dir1, 180, Axis, Dir3) :-
    rot_dir(Dir1, 90, Axis, Dir2), rot_dir(Dir2, 90, Axis, Dir3).

rot_dir(Dir1, 270, Axis, Dir2) :- rot_dir(Dir2, 90, Axis, Dir1).
rot_dir(Dir, 0, Axis, Dir) :- Dir=(a; b; n; e; s; w), Axis = (x; y; z).

% For each block, we choose a rotation for each axis.
{ rot(Axis, B, R) : rot(R) } = 1 :- block(B), Axis = (x; y; z).

% We rotate each block in the three axes with the chosen rotations.
rot_x(B, N1, N2, Dir') :-
    rot(x, B, R), rot_dir(Dir, R, x, Dir'), adjacent_block(B, N1, N2, Dir).

rot_y(B, N1, N2, Dir') :-
    rot(y, B, R), rot_dir(Dir, R, y, Dir'), rot_x(B, N1, N2, Dir).

block(B, N1, N2, Dir') :-
    rot(z, B, R), rot_dir(Dir, R, z, Dir'), rot_y(B, N1, N2, Dir).

% We rotate the pipe segments in the three axes using
% the same rotation as their corresponding block.
pipe_rot_x(B, N, In', Out') :- rot(x, B, R), rot_dir(In, R, x, In'),
    rot_dir(Out, R, x, Out'), pipe_unrotated(B, N, In, Out).

pipe_rot_y(B, N, In', Out') :- rot(y, B, R), rot_dir(In, R, y, In'),
    rot_dir(Out, R, y, Out'), pipe_rot_x(B, N, In, Out).

pipe_rot_z(B, N, In', Out') :- rot(z, B, R), rot_dir(In, R, z, In'),
    rot_dir(Out, R, z, Out'), pipe_rot_y(B, N, In, Out).

% We randomize the pipe segments In and Out to avoid 
% giving information about the pipe to the solver.
{ pipe(B, N, In, Out); pipe(B, N, Out, In)} = 1 :- pipe_rot_z(B, N, In, Out).

#show width/1. #show height/1. #show depth/1. #show block/1.
#show block/4. #show pipe/4. #show pipe_in/4. #show pipe_out/4.
