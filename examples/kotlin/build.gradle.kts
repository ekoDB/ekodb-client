plugins {
    kotlin("jvm") version "1.9.22"
    kotlin("plugin.serialization") version "1.9.22"
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
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")
    
    // Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.2")
    
    // Ktor client (same versions as the client library)
    implementation("io.ktor:ktor-client-core:2.3.7")
    implementation("io.ktor:ktor-client-cio:2.3.7")
    implementation("io.ktor:ktor-client-content-negotiation:2.3.7")
    implementation("io.ktor:ktor-serialization-kotlinx-json:2.3.7")
    implementation("io.ktor:ktor-client-websockets:2.3.7")
    implementation("io.ktor:ktor-client-logging:2.3.7")
    
    // DateTime
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.5.0")
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
