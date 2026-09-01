#include <iostream>

extern "C" {
    void* LoadLibraryA(const char* libname) {
        std::cout << "[Win32 Compat] Interceptado LoadLibraryA: " << (libname ? libname : "NULL") << std::endl;
        return (void*)0x1000; 
    }

    void* GetProcAddress(void* hModule, const char* procName) {
        std::cout << "[Win32 Compat] Interceptado GetProcAddress para: " << (procName ? procName : "NULL") << std::endl;
        return nullptr;
    }

    int MessageBoxA(void* hWnd, const char* text, const char* caption, unsigned int type) {
        std::cout << "[Carlinhos OS - MessageBox] " << (caption ? caption : "Aviso") << ": " << (text ? text : "") << std::endl;
        return 1; 
    }
}
