from pprint import pprint
from collections import deque

s = "^ENWWW(NEEE|SSE(EE|N))$"
s = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"
s = "^ENNWSWW(|NEWS)SSSEEN(|WNSE)EE(|SWEN)NNN$"
# s = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
# s = "^ESSWWN(E|NNENN(EESS(|WNSE)SSS|WWWSSSSE(SW|NNNE)))$"
# s = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
# with open("data/day20") as f:
#     s = f.read().strip()

DELTAS = {
    "N": (0, -1),
    "S": (0, 1),
    "E": (1, 0),
    "W": (-1, 0),
}

g = {(0, 0): []}
n = (0, 0)
stack = []

for c in s:
    if c in "^$":
        pass
    elif c == "(":
        stack.append(n)
    elif c == ")":
        pass
        # if stack:
        #     n = stack.pop()
    elif c == "|":
        if stack:
            n = stack.pop()
        # else:
        #     stack.append(n)
    else:
        dx, dy = DELTAS[c]
        x, y = n
        g.setdefault(n, []).append((x + dx, y + dy))
        n = (x + dx, y + dy)

# pprint(g)
# print(n)
# print(len(n))
print(stack)

queue = deque([(0, (0, 0))])
seen = {(0, 0)}
max_doors = 0
while queue:
    doors, curr = queue.popleft()
    if curr not in g:
        continue
    for neigh in g[curr]:
        if neigh not in seen:
            seen.add(neigh)
            queue.append((doors + 1, neigh))
            max_doors = max(max_doors, doors + 1)
print(max_doors)
