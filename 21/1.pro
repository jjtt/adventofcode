:- initialization main.

dairy(fvjkl).
dairy(kfcds).
dairy(mxmxvkd).
dairy(nhms).
dairy(sbzzf).
dairy(sqjhc).
dairy(trh).
fish(kfcds).
fish(mxmxvkd).
fish(nhms).
fish(sbzzf).
fish(sqjhc).
soy(fvjkl).
soy(sqjhc).
solve( OK ):-
  dairy(DAIRY), soy(SOY), fish(FISH), not(DAIRY=SOY), not(DAIRY=FISH), not(SOY=FISH),
  not(DAIRY=OK), not(OK=FISH), not(SOY=OK),
  (dairy(OK); soy(OK); fish(OK)).


main :-
  findall(OK,solve(OK),writeln(OK)),
  halt(0).
