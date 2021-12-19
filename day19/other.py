
   
#!/usr/bin/python3
import sys
import itertools as it
import functools as ft
import re
import math
import datetime as dt

#probably a crappy algorithm and even crappier code, i agree. 
#If anyone sees this, please try to optimize as the runtime is huge


file_name=sys.argv[1]
file=open(file_name,"r")
index=-1
l=[]

def rotate(l,x):
  x%=len(l)
  if x==0:
    return l.copy()
  ans=l[x:]+l[0:x]
  return ans
def generate_all():
  all_signs=[(1,1,1),(-1,-1,1),(-1,1,-1),(1,-1,-1)]
  prim1=[(0,1),(1,1),(2,1)]
  prim2=[(1,1),(0,1),(2,-1)]
  all=[]
  for i in all_signs:
    for j in range(len(prim1)):
      cur=rotate(prim1,j)
      all.append([(cur[t][0],cur[t][1]*i[t]) for t in range(3)])

  for i in all_signs:
    for j in range(len(prim2)):
      cur=rotate(prim2,j)
      all.append([(cur[t][0],cur[t][1]*i[t]) for t in range(3)])
  # print(all)
  return all




for i in file.readlines():
  i=i.strip('\n')
  # print(i)
  if len(i)==0:
    continue
  elif i.find("scanner")!=-1:
    index+=1
    l.append([])
  else:
    l[-1].append([int(x) for x in i.split(',')]) #maybe make a tuple

total=index+1

pos=[None]*total
pos[0]=(0,0,0)

all_variants=generate_all()

done=set([0])
undone=set(range(1,total))
match_crit=12

while len(undone)!=0:
  i=done.pop()
  new_done=set()
  for j in undone:
    got_ans=False
    for vari in all_variants:#->[(index,sign)...]
      if got_ans:
          break
      new_points=[]
      for k in l[j]:#k->point (x,y,z)
        # print(k)
        # print(vari)
        new_points.append([k[vari[t][0]]*vari[t][1] for t in range(3)])
      for t0 in l[i]:
        if got_ans:
          break
        for t in new_points:
          x,y,z=[t0[iter]-t[iter] for iter in range(3)] #check here
          all_points_i=set([tuple(T) for T in l[i]])
          all_points_j=set()
          for T in new_points:
            all_points_j.add(tuple([T[0]+x,T[1]+y,T[2]+z]))
          if len(all_points_j.intersection(all_points_i))>=match_crit:
            # print(i,j)
            # print(all_points_j)
            # print(x,y,z)
            new_done.add(j)
            got_ans=True
            pos[j]=(x,y,z)
            l[j]=list(all_points_j)
            break
  undone-=new_done
  done|=new_done      

assert None not in pos, f"values of pos: {pos}"
# for i in range(total):
#   print(i,l[i])



all_the_points=set()
for i in l:
  for j in i:
    all_the_points.add(tuple(j))
print(len(all_the_points))

best=0
for i in pos:
  for j in pos:
    best=max(best,sum([abs(i[t]-j[t]) for t in range(3)]))
print(best)