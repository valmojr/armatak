const DLL_NAME_32 = "armatak.dll";
const DLL_NAME_64 = "armatak_x64.dll";
const SO_NAME_32 = "armatak.so";
const SO_NAME_64 = "armatak_x64.so";

let dllDir = HEMTT_RFS.join("extensions").join("armatak").join("dist");
let dll = dllDir.join(DLL_NAME_32);
let dll_x64 = dllDir.join(DLL_NAME_64);
let so = dllDir.join(SO_NAME_32);
let so_x64 = dllDir.join(SO_NAME_64);

if dllDir.exists() && dll.is_file() && dll.exists() {
  print("Adding x86 DLL file to Release");
  dll.copy(HEMTT_OUT.join(DLL_NAME_32));
  print("Added x86 DLL file to Release");
};

if dllDir.exists() && dll_x64.is_file() && dll_x64.exists() {
  print("Adding x64 DLL file to Release");
  dll_x64.copy(HEMTT_OUT.join(DLL_NAME_64));
  print("Added x64 DLL file to Release");
};

if dllDir.exists() && so.is_file() && so.exists() {
  print("Adding x86 SO file to Release");
  so.copy(HEMTT_OUT.join(SO_NAME_32));
  print("Added x86 SO file to Release");
};

if dllDir.exists() && so_x64.is_file() && so_x64.exists() {
  print("Adding x64 SO file to Release");
  so_x64.copy(HEMTT_OUT.join(SO_NAME_64));
  print("Added x64 SO file to Release");
};
