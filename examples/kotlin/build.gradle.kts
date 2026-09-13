plugins {
    kotlin("jvm") version "2.4.10"
    kotlin("plugin.serialization") version "2.4.10"
    application
}

group = "io.ekodb.examples"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    // Use the local client library (dynamically find the built JAR)
    implementation(fileTree("../../ekodb-client-kt/build/libs") {
        include("ekodb-client-kt-*.jar")
    })
    
    // Kotlin stdlib
    implementation(kotlin("stdlib"))
    
    // Coroutines (match client library versions)
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
    
    // Serialization (match client library versions)
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.11.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-cbor:1.11.0")
    // MessagePack core: the client depends on it for the WebSocket binary
    // transport. The examples consume the client as a flat jar (no transitive
    // deps), so this must mirror the client's dependency.
    implementation("org.msgpack:msgpack-core:0.9.8")

    // Ktor client (match client library versions)
    implementation("io.ktor:ktor-client-core:3.5.2")
    implementation("io.ktor:ktor-client-cio:3.5.2")
    implementation("io.ktor:ktor-client-content-negotiation:3.5.2")
    implementation("io.ktor:ktor-serialization-kotlinx-json:3.5.2")
    implementation("io.ktor:ktor-client-websockets:3.5.2")
    implementation("io.ktor:ktor-client-logging:3.5.2")
    implementation("io.ktor:ktor-client-encoding:3.5.2")

    // DateTime
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.8.0-0.6.x-compat")
    
    // Dotenv for environment variables
    implementation("io.github.cdimascio:dotenv-kotlin:6.4.1")
}

kotlin {
    jvmToolchain(17)
}

// Configure which example to run
val exampleClass = project.findProperty("mainClass") as String? ?: "io.ekodb.client.examples.ClientSimpleCrudKt"

application {
    mainClass.set(exampleClass)
}

tasks.named<JavaExec>("run") {
    // Forward these ONLY when the invoking shell actually set them. Each example
    // calls dotenv() to read this project's .env, and dotenv-kotlin gives an
    // already-set process environment variable precedence over the .env file --
    // so unconditionally setting a fallback here (as this used to do) permanently
    // shadows .env's real values with these placeholders, regardless of what's
    // configured there.
    listOf("API_BASE_URL", "WS_BASE_URL", "API_BASE_KEY").forEach { name ->
        System.getenv(name)?.let { environment(name, it) }
    }
}

// Add source directory
sourceSets {
    main {
        kotlin.srcDir("examples")
    }
}
