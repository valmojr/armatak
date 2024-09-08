const DLL_NAME_32 = "armatak.dll";
const DLL_NAME_64 = "armatak_x64.dll";
const SO_NAME_32 = "armatak.so";
const SO_NAME_64 = "armatak_x64.so";

let dllDir = HEMTT_RFS.join("target").join("release");
let dll = dllDir.join(DLL_NAME_32);

if dllDir.exists() && dll.is_file() && dll.exists() {
  print("Adding x86 DLL file to Release");
  dll.copy(HEMTT_OUT.join(DLL_NAME_32));
  HEMTT_OUT.join(DLL_NAME_32).copy(HEMTT_OUT.join(DLL_NAME_64));
  print("Added x86 DLL file to Release");
};
