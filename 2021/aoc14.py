from collections import defaultdict


def find_letters(l):
    letters=''
    for (a,b) in l:
        for e in a:
            if e not in letters:
                letters+=e
    return(letters)

def read_data2():
    data=open('data/14.txt').readlines()
    l={}
    d={}
    for e in data:
        e=e.strip()
        if e:
            if '-' in e:
                e=e.split(' -> ')
                l[e[0]]=e[1]
            else:
                start=e
    for double in l:
        d[double]=0
    for i in range(len(start)-1):
        d[start[i:i+2]]+=1
    return(d,l)

def count_letters(d,letters):
    l=[0]*len(letters)
    for ind,e in enumerate(letters):
        for double in d:
            l[ind]+=double.count(e)*d[double]
    l=[(l[i]+1)//2 for i in range(len(l))]
    return(l)

def def_val():return(0)

def count_new_double(d,l):
    newd = defaultdict(def_val)
    for double in d:
        a,b,c=double[0],double[1],l[double]
        newd[a+c]+=d[double]
        newd[c+b]+=d[double]
    return(newd)

def part2():
    d,l = read_data2()
    for _ in range(40):
        d=count_new_double(d,l)
    letters=find_letters(l)
    c=count_letters(d,letters)
    print('Part 2:',max(c)-min(c))
    return()

part2()
