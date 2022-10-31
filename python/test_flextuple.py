import pickle
from rrs import FlexTuple


class Item(FlexTuple):
    foo: int
    bar: float

class Item2(FlexTuple):
    biz: int

print("gffffffffff")
print(Item.__dict__['__schema__'])
print(Item2.__dict__['__schema__'])
print(Item2.__schema__)

i = Item(foo=42, bar=4.2)
#i2 = Item()
#i3 = Item()
#i4 = Item(1, 4.2)

print(i)
print(i.foo)
print(i.bar)
print(f"fields: {Item.__fields__}")


data = pickle.dumps(i)
print(data)
i_new = pickle.loads(data)
#print(i_new)
