plugins {
    kotlin("jvm") version "2.0.21"
    kotlin("plugin.serialization") version "2.0.21"
    application
}

group = "io.ekodb.examples"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    // Use the local client library
    implementation(files("../../ekodb-client-kt/build/libs/ekodb-client-kt-0.1.0.jar"))
    
    // Kotlin stdlib
    implementation(kotlin("stdlib"))
    
    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.9.0")
    
    // Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.3")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-cbor:1.7.3") // CBOR for binary serialization (required by client library)
    
    // Ktor client (same versions as the client library)
    implementation("io.ktor:ktor-client-core:3.0.1")
    implementation("io.ktor:ktor-client-cio:3.0.1")
    implementation("io.ktor:ktor-client-content-negotiation:3.0.1")
    implementation("io.ktor:ktor-serialization-kotlinx-json:3.0.1")
    implementation("io.ktor:ktor-client-websockets:3.0.1")
    implementation("io.ktor:ktor-client-logging:3.0.1")
    implementation("io.ktor:ktor-client-encoding:3.0.1")
    
    // DateTime
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.6.1")
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
    environment("API_BASE_URL", System.getenv("API_BASE_URL") ?: "http://localhost:8080")
    environment("WS_BASE_URL", System.getenv("WS_BASE_URL") ?: "ws://localhost:8080")
    environment("API_BASE_KEY", System.getenv("API_BASE_KEY") ?: "")
}

// Add source directory
sourceSets {
    main {
        kotlin.srcDir("examples")
    }
}
