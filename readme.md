library to get relative hand rank based on the board for omaha poker.

known bugs:
1) straights are not computed properly, cant break them into difference boards as it causes the final result to give wrong winners. @finish cfr trainer before handling this
STILL WIP!


uses the approach, without monte carlo from https://paginas.fe.up.pt/~prodei/dsie11/images/pdfs/s3-4.pdf

uses a deterministic filter on boards to reduce search space.