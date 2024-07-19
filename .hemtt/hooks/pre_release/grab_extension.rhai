const dllDir = HEMTT_RFS.join("extensions").join("armatak").join("dist");

const dll = dllDir.join("armatak.dll");
const dll_x64 = dllDir.join("armatak_x64.dll");

if dllDir.exists() && dll.is_file() && dll.exists() {
  print("Adding x86 DLL file to Release");
  dll.copy(HEMTT_OUT);
  print("Added x86 DLL file to Release");
};

if dllDir.exists() && dll_x64.is_file() && dll_x64.exists() {
  print("Adding x64 DLL file to Release");
  dll_x64.copy(HEMTT_OUT);
  print("Added x64 DLL file to Release");
};