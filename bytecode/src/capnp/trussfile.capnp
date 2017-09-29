@0xe9ae95396aaf49f4;

# Trussfile specification for TRUSS binaries.
#
# Specifically, `.truss` files on-disk are referred to as "components".  One or
# several components are used to build up a system.  They are approxiamately
# the same idea as a Java `.jar` file or a .NET Assembly (some types of
# Windows `.dll` files.).  They are meant to be present on a system and be
# implicitly passed to the VM, by being installed in a central location or by
# being present in the current directory.  One can also initiate loading
# bytecode data from within the VM, if dynamic loading at runtime is necessary.

struct Trussfile {

 	componentName @0 :Text;
	vendorId @1 :Text;
	entries @2 :List(ComponentEntry);

}

struct ComponentEntry {
	name @0 :Text;
	body :union {
		comment @1 :Text;
		classdef @2 :ClassDef;
		methodef @3 :MethodDef;
	}
}

struct ClassDef {
	package @0 :Text;
	# TODO
}

struct MethodDef {
	package :union {
		none @0 :Void;
		memberof @1 :Text;
	}
	# TODO
}
