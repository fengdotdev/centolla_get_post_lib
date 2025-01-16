#include <jni.h>
#include <iostream>
#include <string>
#include <vector>
#include <cstdlib>
#include "your_rust_library.h" // Archivo de cabecera generado para la librería Rust

// Función auxiliar para convertir jstring de Kotlin a std::string
std::string jstringToString(JNIEnv* env, jstring jStr) {
    const char* chars = env->GetStringUTFChars(jStr, nullptr);
    std::string result(chars);
    env->ReleaseStringUTFChars(jStr, chars);
    return result;
}

// Función auxiliar para crear un vector de HeaderC desde Kotlin
std::vector<HeaderC> createHeaders(JNIEnv* env, jobjectArray jHeaders) {
    std::vector<HeaderC> headers;
    jsize len = env->GetArrayLength(jHeaders);

    for (jsize i = 0; i < len; i++) {
        jobject jHeader = env->GetObjectArrayElement(jHeaders, i);

        jclass headerClass = env->GetObjectClass(jHeader);
        jmethodID getKey = env->GetMethodID(headerClass, "getKey", "()Ljava/lang/String;");
        jmethodID getValue = env->GetMethodID(headerClass, "getValue", "()Ljava/lang/String;");

        jstring jKey = (jstring)env->CallObjectMethod(jHeader, getKey);
        jstring jValue = (jstring)env->CallObjectMethod(jHeader, getValue);

        HeaderC header;
        header.key = strdup(jstringToString(env, jKey).c_str()); // Asignación dinámica
        header.value = strdup(jstringToString(env, jValue).c_str());
        headers.push_back(header);

        env->DeleteLocalRef(jHeader);
        env->DeleteLocalRef(jKey);
        env->DeleteLocalRef(jValue);
    }

    return headers;
}

// Liberar memoria de HeaderC
void freeHeaders(std::vector<HeaderC>& headers) {
    for (auto& header : headers) {
        free((void*)header.key);
        free((void*)header.value);
    }
}

// JNI Método para llamar a get_res de la librería Rust
extern "C" JNIEXPORT jstring JNICALL
Java_com_example_MyRustLibrary_getRes(
    JNIEnv* env,
    jobject,
    jstring jUrl,
    jobjectArray jHeaders,
    jstring jBody
) {
    std::string url = jstringToString(env, jUrl);
    std::string body = jstringToString(env, jBody);
    std::vector<HeaderC> headers = createHeaders(env, jHeaders);

    // Crear una instancia del objeto Rust
    GetPostObjC* rustObj = get_post_obj_new(nullptr);

    // Llamar a la función Rust
    ResponseC response = get_res(rustObj, url.c_str(), headers.data(), headers.size(), body.c_str());

    // Convertir el resultado ResponseC a jstring
    jstring result = env->NewStringUTF(response.body);

    // Liberar memoria en Rust
    get_post_obj_free(rustObj);

    // Liberar headers
    freeHeaders(headers);

    return result;
}

// JNI Método para llamar a post_res de la librería Rust
extern "C" JNIEXPORT jstring JNICALL
Java_com_example_MyRustLibrary_postRes(
    JNIEnv* env,
    jobject,
    jstring jUrl,
    jobjectArray jHeaders,
    jstring jBody
) {
    std::string url = jstringToString(env, jUrl);
    std::string body = jstringToString(env, jBody);
    std::vector<HeaderC> headers = createHeaders(env, jHeaders);

    // Crear una instancia del objeto Rust
    GetPostObjC* rustObj = get_post_obj_new(nullptr);

    // Llamar a la función Rust
    ResponseC response = post_res(rustObj, url.c_str(), headers.data(), headers.size(), body.c_str());

    // Convertir el resultado ResponseC a jstring
    jstring result = env->NewStringUTF(response.body);

    // Liberar memoria en Rust
    get_post_obj_free(rustObj);

    // Liberar headers
    freeHeaders(headers);

    return result;
}
