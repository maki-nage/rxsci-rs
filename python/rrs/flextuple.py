from abc import ABC, ABCMeta
from _rrs import ffi, lib


class FlexTupleMeta(ABCMeta):
    #__fields__ = {}
    #__schema__ = None

    def __new__(cls, name, bases, dct):
        print("Meta Init")
        print(name)
        print(bases)
        print(dct)
        fields = {}
        if '__annotations__' in dct: # no annotation when creating FlexTuple class
            schema = lib.flextuple_schema_create(name.encode('utf8'))
            for i,k in enumerate(dct['__annotations__']):
                ftype = dct['__annotations__'][k]
                fields[k] = (i, ftype)
                if ftype == int:
                    lib.flextuple_schema_add_int64(schema, k.encode('utf8'))
            
            dct['__schema__'] = schema
            dct['__fields__'] = fields
        return super().__new__(cls, name, bases, dct)


class FlexTuple(ABC, metaclass=FlexTupleMeta):
    __slots__ = ('__ft')

    def __init__(self, *args, **kwargs):
        super().__init__()
        print("init")
        stype = type(self)
        print(self.__annotations__)
        print(self.__dict__)
        print(stype.__fields__)
        if args:
            print(args)
            if len(args) != len(stype.__fields__):
                raise ValueError("invalid number of arguments")            
            self.__ft = lib.flextuple_create(stype.__schema__)
            print(f"self ft: {self.__ft}")
            print(stype.__fields__)
            for arg, field in zip(args, stype.__fields__.items()):
                
                ftype = field[1][1]
                print(field)
                print(arg)
                if type(arg) != ftype:
                    raise TypeError(f"field {field[0]} of {stype} must be typed {ftype}, got {type(arg)}")
                if ftype is int:
                    lib.flextuple_add_int64(self.__ft, arg)
                elif ftype is float:
                    lib.flextuple_add_float64(self.__ft, arg)

            print("got args")
        elif kwargs:
            if len(kwargs) != len(stype.__fields__):
                raise ValueError("invalid number of arguments")
            self.__ft = lib.flextuple_create(stype.__schema__)
            print("kwargs")
            for k, v in kwargs.items():
                print(k)
                print(v)
                field = stype.__fields__[k]
                arg = v
                ftype = field[1]
                if type(arg) != ftype:
                    raise TypeError(f"field {field[0]} of {stype} must be typed {ftype}")
                if ftype is int:
                    lib.flextuple_add_int64(self.__ft, arg)
                elif ftype is float:
                    lib.flextuple_add_float64(self.__ft, arg)
        else:
            raise ValueError("FlexTuple initialized without arguments")
        

    def __getitem__(self, key):
        print(f"getitem: {name}")
        return 1

    def __getattr__(self, name):
        #print(f"getattr: {name}, index {self.__fields__[name][0]}")
        print(f"getattr: {name}")
        f = type(self).__fields__.get(name, None)
        if f:
            if f[1] is int:
                return lib.flextuple_get_int64_at(self.__ft, f[0])
            elif f[1] is float:
                return lib.flextuple_get_float64_at(self.__ft, f[0])
        elif name == "__ft":
            return self.__ft
        return 1

    #def __getattribute__(self, name):
    #    print(f"getattribute: {name}")
    #    return 1

    def __getstate__(self):
     print("I'm being pickled")
     #return self.__dict__
     return b'flexpickle'  # return bytes

    def __setstate__(self, d):
        print("I'm being unpickled with these values: " + repr(d))
        #self.__dict__ = d
        # create from bytes
