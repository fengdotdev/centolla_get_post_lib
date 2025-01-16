package com.example

class MyRustLibrary {

    // Cargar la librería nativa
    companion object {
        init {
            System.loadLibrary("my_jni_library") // Nombre de la librería generada
        }
    }

    external fun getRes(url: String, headers: Array<Header>, body: String): String
    external fun postRes(url: String, headers: Array<Header>, body: String): String

    data class Header(val key: String, val value: String)
}
