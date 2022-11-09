from abc import ABC, ABCMeta
from _rrs import ffi, lib


class FlexTupleMeta(ABCMeta):
    #__fields__ = {}
    #__schema__ = None

    def __new__(cls, name, bases, dct):
        fields = {}
        builder = None
        if '__annotations__' in dct: # no annotation when creating FlexTuple class
            builder = lib.flextuple_schema_builder(name.encode('utf8'))
            for i,k in enumerate(dct['__annotations__']):
                ftype = dct['__annotations__'][k]
                fields[k] = (i, ftype)
                if ftype == int:
                    lib.flextuple_schema_add_int64(builder, k.encode('utf8'))
            
            #dct['__schema__'] = None
            #dct['__type_handle__'] = None
            dct['__fields__'] = fields
        t = super().__new__(cls, name, bases, dct)
        if builder:
            handle = ffi.new_handle(t)
            t.__type_handle__ = handle
            lib.flextuple_schema_set_handle(builder, handle)
            t.__schema__ = lib.flextuple_schema_build(builder)
        return t


class FlexTuple(object, metaclass=FlexTupleMeta):
    __slots__ = ('__ft', 'own')

    def __init__(self, *args, **kwargs):
        stype = type(self)
        builder = None
        if args:
            if len(args) != len(stype.__fields__):
                raise ValueError("invalid number of arguments")            
            builder = lib.flextuple_builder(stype.__schema__)
            for arg, field in zip(args, stype.__fields__.items()):
                ftype = field[1][1]
                if type(arg) != ftype:
                    raise TypeError(f"field {field[0]} of {stype} must be typed {ftype}, got {type(arg)}")
                if ftype is int:
                    lib.flextuple_add_int64(builder, arg)
                elif ftype is float:
                    lib.flextuple_add_float64(builder, arg)

        elif kwargs:
            if len(kwargs) != len(stype.__fields__):
                raise ValueError("invalid number of arguments")
            builder = lib.flextuple_builder(stype.__schema__)
            for k, v in kwargs.items():
                field = stype.__fields__[k]
                arg = v
                ftype = field[1]
                if type(arg) != ftype:
                    raise TypeError(f"field {field[0]} of {stype} must be typed {ftype}")
                if ftype is int:
                    lib.flextuple_add_int64(builder, arg)
                elif ftype is float:
                    lib.flextuple_add_float64(builder, arg)
        #else:
        #    raise ValueError("FlexTuple initialized without arguments")

        if builder:
            self.__ft = lib.flextuple_build(builder)
        self.own = True
        
    def init_from_native(self, ft, own=True):
        self.__ft = ft
        self.own = own

    def __del__(self):
        if self.own:
            lib.flextuple_drop(self.__ft)

    def __repr__(self):
        kwargs = [
            f"{f}={getattr(self, f)}"
            for f in type(self).__fields__
        ]
        return ",".join(kwargs)

    #def __getitem__(self, key):
    #    print(f"getitem: {name}")
    #    return 1

    def __getattr__(self, name):
        #print(f"getattr: {name}, index {self.__fields__[name][0]}")
        #print(f"getattr: {name}")
        if name == "__ft":
            return self.__ft
        f = type(self).__fields__.get(name, None)
        if f:
            if f[1] is int:
                return lib.flextuple_get_int64_at(self.__ft, f[0])
            elif f[1] is float:
                return lib.flextuple_get_float64_at(self.__ft, f[0])
        return None

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
