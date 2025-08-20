// Auto-generated from Microsoft linker options documentation
use crate::args::ArgumentParser;
use crate::error::Result;

pub fn setup_windows_argument_parser() -> ArgumentParser {
    let mut parser = ArgumentParser::new();
    // /ALIGN - Specifies the alignment of each section.
    parser
        .declare_with_param()
        .long("ALIGN")
        .help("/ALIGN - Specifies the alignment of each section.")
        .execute(|_, _, _| Ok(()));
    // /ALLOWBIND - Specifies that a DLL can't be bound.
    parser
        .declare()
        .long("ALLOWBIND")
        .help("/ALLOWBIND - Specifies that a DLL can't be bound.")
        .execute(|_, _, _| Ok(()));
    // /ALLOWISOLATION - Specifies behavior for manifest lookup.
    parser
        .declare()
        .long("ALLOWISOLATION")
        .help("/ALLOWISOLATION - Specifies behavior for manifest lookup.")
        .execute(|_, _, _| Ok(()));
    // /APPCONTAINER - Specifies whether the app must run within an appcontainer process environment.
    parser
        .declare()
        .long("APPCONTAINER")
        .help("/APPCONTAINER - Specifies whether the app must run within an appcontainer process environment.")
        .execute(|_, _, _| Ok(()));
    // /ARM64XFUNCTIONPADMINX64 - Specifies the minimum number of bytes of padding between x64 functions in ARM64X images. 17.8
    parser
        .declare_with_param()
        .long("ARM64XFUNCTIONPADMINX64")
        .help("/ARM64XFUNCTIONPADMINX64 - Specifies the minimum number of bytes of padding between x64 functions in ARM64X images. 17.8")
        .execute(|_, _, _| Ok(()));
    // /ASSEMBLYDEBUG - Adds the DebuggableAttribute to a managed image.
    parser
        .declare()
        .long("ASSEMBLYDEBUG")
        .help("/ASSEMBLYDEBUG - Adds the DebuggableAttribute to a managed image.")
        .execute(|_, _, _| Ok(()));
    // /ASSEMBLYLINKRESOURCE - Creates a link to a managed resource.
    parser
        .declare_with_param()
        .long("ASSEMBLYLINKRESOURCE")
        .help("/ASSEMBLYLINKRESOURCE - Creates a link to a managed resource.")
        .execute(|_, _, _| Ok(()));
    // /ASSEMBLYMODULE - Specifies that a Microsoft intermediate language (MSIL) module should be imported into the assembly.
    parser
        .declare_with_param()
        .long("ASSEMBLYMODULE")
        .help("/ASSEMBLYMODULE - Specifies that a Microsoft intermediate language (MSIL) module should be imported into the assembly.")
        .execute(|_, _, _| Ok(()));
    // /ASSEMBLYRESOURCE - Embeds a managed resource file in an assembly.
    parser
        .declare_with_param()
        .long("ASSEMBLYRESOURCE")
        .help("/ASSEMBLYRESOURCE - Embeds a managed resource file in an assembly.")
        .execute(|_, _, _| Ok(()));
    // /BASE - Sets a base address for the program.
    parser
        .declare_with_param()
        .long("BASE")
        .help("/BASE - Sets a base address for the program.")
        .execute(|_, _, _| Ok(()));
    // /CETCOMPAT - Marks the binary as CET Shadow Stack compatible.
    parser
        .declare()
        .long("CETCOMPAT")
        .help("/CETCOMPAT - Marks the binary as CET Shadow Stack compatible.")
        .execute(|_, _, _| Ok(()));
    // /CGTHREADS - Sets number of cl.exe threads to use for optimization and code generation when link-time code generation is specified.
    parser
        .declare_with_param()
        .long("CGTHREADS")
        .help("/CGTHREADS - Sets number of cl.exe threads to use for optimization and code generation when link-time code generation is specified.")
        .execute(|_, _, _| Ok(()));
    // /CLRIMAGETYPE - Sets the type (IJW, pure, or safe) of a CLR image.
    parser
        .declare_with_param()
        .long("CLRIMAGETYPE")
        .help("/CLRIMAGETYPE - Sets the type (IJW, pure, or safe) of a CLR image.")
        .execute(|_, _, _| Ok(()));
    // /CLRSUPPORTLASTERROR - Preserves the last error code of functions that are called through the P/Invoke mechanism.
    parser
        .declare()
        .long("CLRSUPPORTLASTERROR")
        .help("/CLRSUPPORTLASTERROR - Preserves the last error code of functions that are called through the P/Invoke mechanism.")
        .execute(|_, _, _| Ok(()));
    // /CLRTHREADATTRIBUTE - Specifies the threading attribute to apply to the entry point of your CLR program.
    parser
        .declare_with_param()
        .long("CLRTHREADATTRIBUTE")
        .help("/CLRTHREADATTRIBUTE - Specifies the threading attribute to apply to the entry point of your CLR program.")
        .execute(|_, _, _| Ok(()));
    // /CLRUNMANAGEDCODECHECK - Specifies whether the linker applies the SuppressUnmanagedCodeSecurity attribute to linker-generated P/Invoke stubs that call from managed code into native DLLs.
    parser
        .declare()
        .long("CLRUNMANAGEDCODECHECK")
        .help("/CLRUNMANAGEDCODECHECK - Specifies whether the linker applies the SuppressUnmanagedCodeSecurity attribute to linker-generated P/Invoke stubs that call from managed code into native DLLs.")
        .execute(|_, _, _| Ok(()));
    // /DEBUG - Creates debugging information.
    parser
        .declare_with_optional_param()
        .long("DEBUG")
        .help("/DEBUG - Creates debugging information.")
        .sub_option("FULL", "Full debugging information.", |_, _, _| Ok(()))
        .sub_option("FASTLINK", "Produces a PDB with limited debug information.", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /DEBUGTYPE - Specifies which data to include in debugging information.
    parser
        .declare_with_param()
        .long("DEBUGTYPE")
        .help("/DEBUGTYPE - Specifies which data to include in debugging information.")
        .execute(|_, _, _| Ok(()));
    // /DEF - Passes a module-definition (.def) file to the linker.
    parser
        .declare_with_param()
        .long("DEF")
        .help("/DEF - Passes a module-definition (.def) file to the linker.")
        .execute(|_, _, _| Ok(()));
    // /DEFAULTLIB - Searches the specified library when external references are resolved.
    parser
        .declare_with_optional_param()
        .long("DEFAULTLIB")
        .help("/DEFAULTLIB - Searches the specified library when external references are resolved.")
        .execute(|_, _, _| Ok(()));
    // /DELAY - Controls the delayed loading of DLLs.
    parser
        .declare_with_optional_param()
        .long("DELAY")
        .help("/DELAY - Controls the delayed loading of DLLs.")
        .execute(|_, _, _| Ok(()));
    // /DELAYLOAD - Causes the delayed loading of the specified DLL.
    parser
        .declare_with_optional_param()
        .long("DELAYLOAD")
        .help("/DELAYLOAD - Causes the delayed loading of the specified DLL.")
        .execute(|_, _, _| Ok(()));
    // /DELAYSIGN - Partially signs an assembly.
    parser
        .declare_with_optional_param()
        .long("DELAYSIGN")
        .help("/DELAYSIGN - Partially signs an assembly.")
        .execute(|_, _, _| Ok(()));
    // /DEPENDENTLOADFLAG - Sets default flags on dependent DLL loads.
    parser
        .declare_with_optional_param()
        .long("DEPENDENTLOADFLAG")
        .help("/DEPENDENTLOADFLAG - Sets default flags on dependent DLL loads.")
        .execute(|_, _, _| Ok(()));
    // /DLL - Builds a DLL.
    parser
        .declare()
        .long("DLL")
        .help("/DLL - Builds a DLL.")
        .execute(|_, _, _| Ok(()));
    // /DRIVER - Creates a kernel mode driver.
    parser
        .declare_with_param()
        .long("DRIVER")
        .help("/DRIVER - Creates a kernel mode driver.")
        .sub_option("UPONLY", "Runs only on a uniprocessor system.", |_, _, _| Ok(()))
        .sub_option("WDM", "Creates a Windows Driver Model driver.", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /DYNAMICBASE - Specifies whether to generate an executable image that's rebased at load time by using the address space layout randomization (ASLR) feature.
    parser
        .declare_with_optional_param()
        .long("DYNAMICBASE")
        .help("/DYNAMICBASE - Specifies whether to generate an executable image that's rebased at load time by using the address space layout randomization (ASLR) feature.")
        .execute(|_, _, _| Ok(()));
    // /DYNAMICDEOPT - Enable C++ Dynamic Debugging (Preview) and step in anywhere with on-demand function deoptimization.
    parser
        .declare_with_optional_param()
        .long("DYNAMICDEOPT")
        .help("/DYNAMICDEOPT - Enable C++ Dynamic Debugging (Preview) and step in anywhere with on-demand function deoptimization.")
        .execute(|_, _, _| Ok(()));
    // /ENTRY - Sets the starting address.
    parser
        .declare_with_optional_param()
        .long("ENTRY")
        .help("/ENTRY - Sets the starting address.")
        .execute(|_, _, _| Ok(()));
    // /ERRORREPORT - Deprecated. Error reporting is controlled by Windows Error Reporting (WER) settings.
    parser
        .declare_with_optional_param()
        .long("ERRORREPORT")
        .help("/ERRORREPORT - Deprecated. Error reporting is controlled by Windows Error Reporting (WER) settings.")
        .execute(|_, _, _| Ok(()));
    // /EXPORT - Exports a function.
    parser
        .declare_with_optional_param()
        .long("EXPORT")
        .help("/EXPORT - Exports a function.")
        .execute(|_, _, _| Ok(()));
    // /FILEALIGN - Aligns sections within the output file on multiples of a specified value.
    parser
        .declare_with_optional_param()
        .long("FILEALIGN")
        .help("/FILEALIGN - Aligns sections within the output file on multiples of a specified value.")
        .execute(|_, _, _| Ok(()));
    // /FIXED - Creates a program that can be loaded only at its preferred base address.
    parser
        .declare_with_optional_param()
        .long("FIXED")
        .help("/FIXED - Creates a program that can be loaded only at its preferred base address.")
        .execute(|_, _, _| Ok(()));
    // /FORCE - Forces a link to complete even with unresolved symbols or symbols defined more than once.
    parser
        .declare_with_optional_param()
        .long("FORCE")
        .help("/FORCE - Forces a link to complete even with unresolved symbols or symbols defined more than once.")
        .execute(|_, _, _| Ok(()));
    // /FUNCTIONPADMIN - Creates an image that can be hot patched.
    parser
        .declare_with_optional_param()
        .long("FUNCTIONPADMIN")
        .help("/FUNCTIONPADMIN - Creates an image that can be hot patched.")
        .execute(|_, _, _| Ok(()));
    // /GENPROFILE , /FASTGENPROFILE - Both of these options specify generation of a .pgd file by the linker to support profile-guided optimization (PGO). /GENPROFILE and /FASTGENPROFILE use different default parameters.
    parser
        .declare_with_optional_param()
        .long("GENPROFILE")
        .help("/GENPROFILE , /FASTGENPROFILE - Both of these options specify generation of a .pgd file by the linker to support profile-guided optimization (PGO). /GENPROFILE and /FASTGENPROFILE use different default parameters.")
        .execute(|_, _, _| Ok(()));
    // /GUARD - Enables Control Flow Guard protection.
    parser
        .declare_with_optional_param()
        .long("GUARD")
        .help("/GUARD - Enables Control Flow Guard protection.")
        .execute(|_, _, _| Ok(()));
    // /HEAP - Sets the size of the heap, in bytes.
    parser
        .declare_with_optional_param()
        .long("HEAP")
        .help("/HEAP - Sets the size of the heap, in bytes.")
        .execute(|_, _, _| Ok(()));
    // /HIGHENTROPYVA - Specifies support for high-entropy 64-bit address space layout randomization (ASLR).
    parser
        .declare_with_optional_param()
        .long("HIGHENTROPYVA")
        .help("/HIGHENTROPYVA - Specifies support for high-entropy 64-bit address space layout randomization (ASLR).")
        .execute(|_, _, _| Ok(()));
    // /IDLOUT - Specifies the name of the .idl file and other MIDL output files.
    parser
        .declare_with_optional_param()
        .long("IDLOUT")
        .help("/IDLOUT - Specifies the name of the .idl file and other MIDL output files.")
        .execute(|_, _, _| Ok(()));
    // /IGNORE - Suppresses output of specified linker warnings.
    parser
        .declare_with_optional_param()
        .long("IGNORE")
        .help("/IGNORE - Suppresses output of specified linker warnings.")
        .execute(|_, _, _| Ok(()));
    // /IGNOREIDL - Prevents the processing of attribute information into an .idl file.
    parser
        .declare_with_optional_param()
        .long("IGNOREIDL")
        .help("/IGNOREIDL - Prevents the processing of attribute information into an .idl file.")
        .execute(|_, _, _| Ok(()));
    // /ILK - Overrides the default incremental database file name.
    parser
        .declare_with_optional_param()
        .long("ILK")
        .help("/ILK - Overrides the default incremental database file name.")
        .execute(|_, _, _| Ok(()));
    // /IMPLIB - Overrides the default import library name.
    parser
        .declare_with_optional_param()
        .long("IMPLIB")
        .help("/IMPLIB - Overrides the default import library name.")
        .execute(|_, _, _| Ok(()));
    // /INCLUDE - Forces symbol references.
    parser
        .declare_with_optional_param()
        .long("INCLUDE")
        .help("/INCLUDE - Forces symbol references.")
        .execute(|_, _, _| Ok(()));
    // /INCREMENTAL - Controls incremental linking.
    parser
        .declare_with_optional_param()
        .long("INCREMENTAL")
        .help("/INCREMENTAL - Controls incremental linking.")
        .sub_option("NO", "Disable incremental linking.", |_, _, _| Ok(()))
        .sub_option("YES", "Enable incremental linking.", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /INFERASANLIBS - Uses inferred sanitizer libraries.
    parser
        .declare_with_optional_param()
        .long("INFERASANLIBS")
        .help("/INFERASANLIBS - Uses inferred sanitizer libraries.")
        .execute(|_, _, _| Ok(()));
    // /INTEGRITYCHECK - Specifies that the module requires a signature check at load time.
    parser
        .declare_with_optional_param()
        .long("INTEGRITYCHECK")
        .help("/INTEGRITYCHECK - Specifies that the module requires a signature check at load time.")
        .execute(|_, _, _| Ok(()));
    // /KERNEL - Create a kernel mode binary.
    parser
        .declare_with_optional_param()
        .long("KERNEL")
        .help("/KERNEL - Create a kernel mode binary.")
        .execute(|_, _, _| Ok(()));
    // /KEYCONTAINER - Specifies a key container to sign an assembly.
    parser
        .declare_with_optional_param()
        .long("KEYCONTAINER")
        .help("/KEYCONTAINER - Specifies a key container to sign an assembly.")
        .execute(|_, _, _| Ok(()));
    // /KEYFILE - Specifies a key or key pair to sign an assembly.
    parser
        .declare_with_optional_param()
        .long("KEYFILE")
        .help("/KEYFILE - Specifies a key or key pair to sign an assembly.")
        .execute(|_, _, _| Ok(()));
    // /LARGEADDRESSAWARE - Tells the compiler that the application supports addresses larger than 2 gigabytes
    parser
        .declare_with_optional_param()
        .long("LARGEADDRESSAWARE")
        .help("/LARGEADDRESSAWARE - Tells the compiler that the application supports addresses larger than 2 gigabytes")
        .execute(|_, _, _| Ok(()));
    // /LIBPATH - Specifies a path to search before the environmental library path.
    parser
        .declare_with_optional_param()
        .long("LIBPATH")
        .help("/LIBPATH - Specifies a path to search before the environmental library path.")
        .execute(|_, _, _| Ok(()));
    // /LINKREPRO - Specifies a path to generate link repro artifacts in.
    parser
        .declare_with_optional_param()
        .long("LINKREPRO")
        .help("/LINKREPRO - Specifies a path to generate link repro artifacts in.")
        .execute(|_, _, _| Ok(()));
    // /LINKREPROFULLPATHRSP - Generates a response file containing the absolute paths to all the files that the linker took as input.
    parser
        .declare_with_optional_param()
        .long("LINKREPROFULLPATHRSP")
        .help("/LINKREPROFULLPATHRSP - Generates a response file containing the absolute paths to all the files that the linker took as input.")
        .execute(|_, _, _| Ok(()));
    // /LINKREPROTARGET - Generates a link repro only when producing the specified target. 16.1
    parser
        .declare_with_optional_param()
        .long("LINKREPROTARGET")
        .help("/LINKREPROTARGET - Generates a link repro only when producing the specified target. 16.1")
        .execute(|_, _, _| Ok(()));
    // /LTCG - Specifies link-time code generation.
    parser
        .declare_with_optional_param()
        .long("LTCG")
        .help("/LTCG - Specifies link-time code generation.")
        .sub_option("NOSTATUS", "Do not display progress.", |_, _, _| Ok(()))
        .sub_option("STATUS", "Display progress.", |_, _, _| Ok(()))
        .sub_option("INCREMENTAL", "Enable incremental LTCG.", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /MACHINE - Specifies the target platform.
    parser
        .declare_with_param()
        .long("MACHINE")
        .help("/MACHINE - Specifies the target platform.")
        .sub_option("ARM", "ARM", |_, _, _| Ok(()))
        .sub_option("ARM64", "ARM64", |_, _, _| Ok(()))
        .sub_option("ARM64EC", "ARM64EC", |_, _, _| Ok(()))
        .sub_option("EBC", "EBC", |_, _, _| Ok(()))
        .sub_option("X64", "X64", |_, _, _| Ok(()))
        .sub_option("X86", "X86", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /MANIFEST - Creates a side-by-side manifest file and optionally embeds it in the binary.
    parser
        .declare_with_optional_param()
        .long("MANIFEST")
        .help("/MANIFEST - Creates a side-by-side manifest file and optionally embeds it in the binary.")
        .execute(|_, _, _| Ok(()));
    // /MANIFESTDEPENDENCY - Specifies a <dependentAssembly> section in the manifest file.
    parser
        .declare_with_optional_param()
        .long("MANIFESTDEPENDENCY")
        .help("/MANIFESTDEPENDENCY - Specifies a <dependentAssembly> section in the manifest file.")
        .execute(|_, _, _| Ok(()));
    // /MANIFESTFILE - Changes the default name of the manifest file.
    parser
        .declare_with_optional_param()
        .long("MANIFESTFILE")
        .help("/MANIFESTFILE - Changes the default name of the manifest file.")
        .execute(|_, _, _| Ok(()));
    // /MANIFESTINPUT - Specifies a manifest input file for the linker to process and embed in the binary. You can use this option multiple times to specify more than one manifest input file.
    parser
        .declare_with_optional_param()
        .long("MANIFESTINPUT")
        .help("/MANIFESTINPUT - Specifies a manifest input file for the linker to process and embed in the binary. You can use this option multiple times to specify more than one manifest input file.")
        .execute(|_, _, _| Ok(()));
    // /MANIFESTUAC - Specifies whether User Account Control (UAC) information is embedded in the program manifest.
    parser
        .declare_with_optional_param()
        .long("MANIFESTUAC")
        .help("/MANIFESTUAC - Specifies whether User Account Control (UAC) information is embedded in the program manifest.")
        .execute(|_, _, _| Ok(()));
    // /MAP - Creates a mapfile.
    parser
        .declare_with_optional_param()
        .long("MAP")
        .help("/MAP - Creates a mapfile.")
        .execute(|_, _, _| Ok(()));
    // /MAPINFO - Includes the specified information in the mapfile.
    parser
        .declare_with_optional_param()
        .long("MAPINFO")
        .help("/MAPINFO - Includes the specified information in the mapfile.")
        .execute(|_, _, _| Ok(()));
    // /MERGE - Combines sections.
    parser
        .declare_with_optional_param()
        .long("MERGE")
        .help("/MERGE - Combines sections.")
        .execute(|_, _, _| Ok(()));
    // /MIDL - Specifies MIDL command-line options.
    parser
        .declare_with_optional_param()
        .long("MIDL")
        .help("/MIDL - Specifies MIDL command-line options.")
        .execute(|_, _, _| Ok(()));
    // /NATVIS - Adds debugger visualizers from a Natvis file to the program database (PDB).
    parser
        .declare_with_optional_param()
        .long("NATVIS")
        .help("/NATVIS - Adds debugger visualizers from a Natvis file to the program database (PDB).")
        .execute(|_, _, _| Ok(()));
    // /NOASSEMBLY - Suppresses the creation of a .NET Framework assembly.
    parser
        .declare_with_optional_param()
        .long("NOASSEMBLY")
        .help("/NOASSEMBLY - Suppresses the creation of a .NET Framework assembly.")
        .execute(|_, _, _| Ok(()));
    // /NODEFAULTLIB - Ignores all (or the specified) default libraries when external references are resolved.
    parser
        .declare_with_optional_param()
        .long("NODEFAULTLIB")
        .help("/NODEFAULTLIB - Ignores all (or the specified) default libraries when external references are resolved.")
        .execute(|_, _, _| Ok(()));
    // /NOENTRY - Creates a resource-only DLL.
    parser
        .declare_with_optional_param()
        .long("NOENTRY")
        .help("/NOENTRY - Creates a resource-only DLL.")
        .execute(|_, _, _| Ok(()));
    // /NOFUNCTIONPADSECTION - Disables function padding for functions in the specified section. 17.8
    parser
        .declare_with_optional_param()
        .long("NOFUNCTIONPADSECTION")
        .help("/NOFUNCTIONPADSECTION - Disables function padding for functions in the specified section. 17.8")
        .execute(|_, _, _| Ok(()));
    // /NOLOGO - Suppresses the startup banner.
    parser
        .declare_with_optional_param()
        .long("NOLOGO")
        .help("/NOLOGO - Suppresses the startup banner.")
        .execute(|_, _, _| Ok(()));
    // /NXCOMPAT - Marks an executable as verified to be compatible with the Windows Data Execution Prevention feature.
    parser
        .declare_with_optional_param()
        .long("NXCOMPAT")
        .help("/NXCOMPAT - Marks an executable as verified to be compatible with the Windows Data Execution Prevention feature.")
        .execute(|_, _, _| Ok(()));
    // /OPT - Controls LINK optimizations.
    parser
        .declare_with_param()
        .long("OPT")
        .help("/OPT - Controls LINK optimizations.")
        .sub_option("REF", "Eliminate unreferenced functions and data.", |_, _, _| Ok(()))
        .sub_option("NOREF", "Keep unreferenced functions and data.", |_, _, _| Ok(()))
        .sub_option("ICF", "Fold identical COMDATs.", |_, _, _| Ok(()))
        .sub_option("NOICF", "Disable identical COMDAT folding.", |_, _, _| Ok(()))
        .sub_option("LBR", "Enable profile guided optimizations (LBR).", |_, _, _| Ok(()))
        .sub_option("NOLBR", "Disable profile guided optimizations (no LBR).", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /ORDER - Places COMDATs into the image in a predetermined order.
    parser
        .declare_with_optional_param()
        .long("ORDER")
        .help("/ORDER - Places COMDATs into the image in a predetermined order.")
        .execute(|_, _, _| Ok(()));
    // /OUT - Specifies the output file name.
    parser
        .declare_with_optional_param()
        .long("OUT")
        .help("/OUT - Specifies the output file name.")
        .execute(|_, _, _| Ok(()));
    // /PDB - Creates a PDB file.
    parser
        .declare_with_optional_param()
        .long("PDB")
        .help("/PDB - Creates a PDB file.")
        .execute(|_, _, _| Ok(()));
    // /PDBALTPATH - Uses an alternate location to save a PDB file.
    parser
        .declare_with_optional_param()
        .long("PDBALTPATH")
        .help("/PDBALTPATH - Uses an alternate location to save a PDB file.")
        .execute(|_, _, _| Ok(()));
    // /PDBSTRIPPED - Creates a PDB file that has no private symbols.
    parser
        .declare_with_optional_param()
        .long("PDBSTRIPPED")
        .help("/PDBSTRIPPED - Creates a PDB file that has no private symbols.")
        .execute(|_, _, _| Ok(()));
    // /PGD - Specifies a .pgd file for profile-guided optimizations.
    parser
        .declare_with_optional_param()
        .long("PGD")
        .help("/PGD - Specifies a .pgd file for profile-guided optimizations.")
        .execute(|_, _, _| Ok(()));
    // /POGOSAFEMODE - Obsolete Creates a thread-safe PGO instrumented build.
    parser
        .declare_with_optional_param()
        .long("POGOSAFEMODE")
        .help("/POGOSAFEMODE - Obsolete Creates a thread-safe PGO instrumented build.")
        .execute(|_, _, _| Ok(()));
    // /PROFILE - Produces an output file that can be used with the Performance Tools profiler.
    parser
        .declare_with_optional_param()
        .long("PROFILE")
        .help("/PROFILE - Produces an output file that can be used with the Performance Tools profiler.")
        .execute(|_, _, _| Ok(()));
    // /RELEASE - Sets the Checksum in the .exe header.
    parser
        .declare_with_optional_param()
        .long("RELEASE")
        .help("/RELEASE - Sets the Checksum in the .exe header.")
        .execute(|_, _, _| Ok(()));
    // /SAFESEH - Specifies that the image will contain a table of safe exception handlers.
    parser
        .declare_with_optional_param()
        .long("SAFESEH")
        .help("/SAFESEH - Specifies that the image will contain a table of safe exception handlers.")
        .execute(|_, _, _| Ok(()));
    // /SECTION - Overrides the attributes of a section.
    parser
        .declare_with_optional_param()
        .long("SECTION")
        .help("/SECTION - Overrides the attributes of a section.")
        .execute(|_, _, _| Ok(()));
    // /SOURCELINK - Specifies a SourceLink file to add to the PDB.
    parser
        .declare_with_optional_param()
        .long("SOURCELINK")
        .help("/SOURCELINK - Specifies a SourceLink file to add to the PDB.")
        .execute(|_, _, _| Ok(()));
    // /STACK - Sets the size of the stack in bytes.
    parser
        .declare_with_optional_param()
        .long("STACK")
        .help("/STACK - Sets the size of the stack in bytes.")
        .execute(|_, _, _| Ok(()));
    // /STUB - Attaches an MS-DOS stub program to a Win32 program.
    parser
        .declare_with_optional_param()
        .long("STUB")
        .help("/STUB - Attaches an MS-DOS stub program to a Win32 program.")
        .execute(|_, _, _| Ok(()));
    // /SUBSYSTEM - Tells the operating system how to run the .exe file.
    parser
        .declare_with_param()
        .long("SUBSYSTEM")
        .help("/SUBSYSTEM - Tells the operating system how to run the .exe file.")
        .sub_option("BOOT_APPLICATION", "Boot application", |_, _, _| Ok(()))
        .sub_option("CONSOLE", "Console", |_, _, _| Ok(()))
        .sub_option("WINDOWS", "Windows GUI", |_, _, _| Ok(()))
        .sub_option("NATIVE", "Native", |_, _, _| Ok(()))
        .sub_option("POSIX", "POSIX", |_, _, _| Ok(()))
        .sub_option("EFI_APPLICATION", "EFI application", |_, _, _| Ok(()))
        .sub_option("EFI_BOOT_SERVICE_DRIVER", "EFI boot service driver", |_, _, _| Ok(()))
        .sub_option("EFI_ROM", "EFI ROM", |_, _, _| Ok(()))
        .sub_option("EFI_RUNTIME_DRIVER", "EFI runtime driver", |_, _, _| Ok(()))
        .execute(|_, _, _| Ok(()));
    // /SWAPRUN - Tells the operating system to copy the linker output to a swap file before it's run.
    parser
        .declare_with_optional_param()
        .long("SWAPRUN")
        .help("/SWAPRUN - Tells the operating system to copy the linker output to a swap file before it's run.")
        .execute(|_, _, _| Ok(()));
    // /TIME - Output linker pass timing information.
    parser
        .declare_with_optional_param()
        .long("TIME")
        .help("/TIME - Output linker pass timing information.")
        .execute(|_, _, _| Ok(()));
    // /TLBID - Specifies the resource ID of the linker-generated type library.
    parser
        .declare_with_optional_param()
        .long("TLBID")
        .help("/TLBID - Specifies the resource ID of the linker-generated type library.")
        .execute(|_, _, _| Ok(()));
    // /TLBOUT - Specifies the name of the .tlb file and other MIDL output files.
    parser
        .declare_with_optional_param()
        .long("TLBOUT")
        .help("/TLBOUT - Specifies the name of the .tlb file and other MIDL output files.")
        .execute(|_, _, _| Ok(()));
    // /TSAWARE - Creates an application that is designed specifically to run under Terminal Server.
    parser
        .declare_with_optional_param()
        .long("TSAWARE")
        .help("/TSAWARE - Creates an application that is designed specifically to run under Terminal Server.")
        .execute(|_, _, _| Ok(()));
    // /USEPROFILE - Uses profile-guided optimization training data to create an optimized image.
    parser
        .declare_with_optional_param()
        .long("USEPROFILE")
        .help("/USEPROFILE - Uses profile-guided optimization training data to create an optimized image.")
        .execute(|_, _, _| Ok(()));
    // /VERBOSE - Prints linker progress messages.
    parser
        .declare_with_optional_param()
        .long("VERBOSE")
        .help("/VERBOSE - Prints linker progress messages.")
        .execute(|_, _, _| Ok(()));
    // /VERSION - Assigns a version number.
    parser
        .declare_with_optional_param()
        .long("VERSION")
        .help("/VERSION - Assigns a version number.")
        .execute(|_, _, _| Ok(()));
    // /WHOLEARCHIVE - Includes every object file from specified static libraries.
    parser
        .declare_with_optional_param()
        .long("WHOLEARCHIVE")
        .help("/WHOLEARCHIVE - Includes every object file from specified static libraries.")
        .execute(|_, _, _| Ok(()));
    // /WINMD - Enables generation of a Windows Runtime Metadata file.
    parser
        .declare_with_optional_param()
        .long("WINMD")
        .help("/WINMD - Enables generation of a Windows Runtime Metadata file.")
        .execute(|_, _, _| Ok(()));
    // /WINMDFILE - Specifies the file name for the Windows Runtime Metadata (winmd) output file that's generated by the /WINMD linker option.
    parser
        .declare_with_optional_param()
        .long("WINMDFILE")
        .help("/WINMDFILE - Specifies the file name for the Windows Runtime Metadata (winmd) output file that's generated by the /WINMD linker option.")
        .execute(|_, _, _| Ok(()));
    // /WINMDKEYFILE - Specifies a key or key pair to sign a Windows Runtime Metadata file.
    parser
        .declare_with_optional_param()
        .long("WINMDKEYFILE")
        .help("/WINMDKEYFILE - Specifies a key or key pair to sign a Windows Runtime Metadata file.")
        .execute(|_, _, _| Ok(()));
    // /WINMDKEYCONTAINER - Specifies a key container to sign a Windows Metadata file.
    parser
        .declare_with_optional_param()
        .long("WINMDKEYCONTAINER")
        .help("/WINMDKEYCONTAINER - Specifies a key container to sign a Windows Metadata file.")
        .execute(|_, _, _| Ok(()));
    // /WINMDDELAYSIGN - Partially signs a Windows Runtime Metadata ( .winmd ) file by placing the public key in the winmd file.
    parser
        .declare_with_optional_param()
        .long("WINMDDELAYSIGN")
        .help("/WINMDDELAYSIGN - Partially signs a Windows Runtime Metadata ( .winmd ) file by placing the public key in the winmd file.")
        .execute(|_, _, _| Ok(()));
    // /WX - Treats linker warnings as errors.
    parser
        .declare_with_optional_param()
        .long("WX")
        .help("/WX - Treats linker warnings as errors.")
        .execute(|_, _, _| Ok(()));
    parser
}
