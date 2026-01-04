plugins {
    kotlin("jvm") version "2.2.0"
    kotlin("plugin.serialization") version "2.2.0"
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
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.10.2")
    
    // Serialization (match client library versions)
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-cbor:1.8.0")
    
    // Ktor client (match client library versions)
    implementation("io.ktor:ktor-client-core:3.3.3")
    implementation("io.ktor:ktor-client-cio:3.3.3")
    implementation("io.ktor:ktor-client-content-negotiation:3.3.3")
    implementation("io.ktor:ktor-serialization-kotlinx-json:3.3.3")
    implementation("io.ktor:ktor-client-websockets:3.3.3")
    implementation("io.ktor:ktor-client-logging:3.3.3")
    implementation("io.ktor:ktor-client-encoding:3.3.3")
    
    // DateTime
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.6.1")
    
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
    environment("API_BASE_URL", System.getenv("API_BASE_URL") ?: "http://localhost:8080")
    environment("WS_BASE_URL", System.getenv("WS_BASE_URL") ?: "ws://localhost:8080")
    environment("API_BASE_KEY", System.getenv("API_BASE_KEY") ?: "a-test-api-key-from-ekodb")
}

// Add source directory
sourceSets {
    main {
        kotlin.srcDir("examples")
        // Exclude composition example - client API incompatible with composition pattern
        kotlin.exclude("**/ClientFunctionComposition.kt")
    }
}
