(* ::Package:: *)

LoadVPData[files_]:=Module[{dataRaw,data},
dataRaw = Association[ToExpression[StringCases[#,DigitCharacter..]][[{3,4}]]->Import[#]&/@files];
data= <||>;
Do[
If[MemberQ[key,data],Nothing,
data[key[[1]]] = {};
]
,{key,Keys[dataRaw]}];

Do[

data[key[[1]]] = Append[data[key[[1]]] ,dataRaw[key][[;;,1]]]

,{key,Keys[dataRaw]}];

data

]


VolumeProfileCount[vp_]:=ParallelProduct[Binomial[vp[[i]]+vp[[i+1]],vp[[i]]],{i,Length[vp]-1}]


AllVps[V_,T_]:= Flatten[Permutations[#]&/@IntegerPartitions[V,{T}],1]


VPCounts[V_,T_]:=Association[Table[ToString[StringJoin @@ Riffle[ToString /@ vp, "_"]]-> VolumeProfileCount[vp],{vp,AllVps[V,T]}]]


SortCounts[counts_,order_]:=Association[Table[If[KeyExistsQ[counts,vp],vp-> counts[vp],vp-> 0],{vp, order}]]
