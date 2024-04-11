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


AllVpsOld[V_,T_]:= Flatten[Permutations[#]&/@IntegerPartitions[V,{T}],1]


VPCounts[V_,T_]:=Association[Table[ToString[StringJoin @@ Riffle[ToString /@ vp, "_"]]-> VolumeProfileCount[vp],{vp,AllVps[V,T]}]]


SortCounts[counts_,order_]:=Association[Table[If[KeyExistsQ[counts,vp],vp-> counts[vp],vp-> 0],{vp, order}]]


SortNormCounts[counts_,order_]:=Association[Table[If[KeyExistsQ[counts,vp],vp-> counts[vp],vp-> 0],{vp, order}]]/Total[counts]


AllVps[V_,T_]:= Module[{boundaryPossibilities,bulkPossibilities,boundarySizes},
boundarySizes = Table[i,{i,2,V-2*(T-2),2}];


Flatten[Table[
boundaryPossibilities =Flatten[ Permutations[#]&/@IntegerPartitions[boundarySize,{2}],1];

bulkPossibilities = Flatten[ Permutations[#]&/@IntegerPartitions[(V-boundarySize)/2,{T-2}],1];

Table[
Table[{bound[[1]]}~Join~bulk~Join~{bound[[-1]]}


,{bulk,bulkPossibilities}]


,{bound,boundaryPossibilities}]

,{boundarySize ,boundarySizes}],2]
]


calcVolume[l_]:=Total[l[[2;;-2]]]*2+l[[1]]+l[[-1]]
