plugins {
    kotlin("jvm") version "2.2.0"
    kotlin("plugin.serialization") version "2.2.0"
    `maven-publish`
    signing
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0"
}

group = "io.ekodb"
version = "0.11.0"

repositories {
    mavenCentral()
}

dependencies {
    // Kotlin
    implementation(kotlin("stdlib"))
    
    // Coroutines for async operations
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.10.2")
    
    // Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-cbor:1.8.0") // CBOR for binary serialization (similar to MessagePack)
    
    // HTTP client
    implementation("io.ktor:ktor-client-core:3.3.3")
    implementation("io.ktor:ktor-client-cio:3.3.3")
    implementation("io.ktor:ktor-client-content-negotiation:3.3.3")
    implementation("io.ktor:ktor-serialization-kotlinx-json:3.3.3")
    implementation("io.ktor:ktor-client-websockets:3.3.3")
    implementation("io.ktor:ktor-client-logging:3.3.3")
    implementation("io.ktor:ktor-client-encoding:3.3.3")
    
    // DateTime
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.6.1")
    
    // Testing
    testImplementation(kotlin("test"))
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.10.2")
    testImplementation("io.ktor:ktor-client-mock:3.3.3")
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(17)
}

// Generate sources JAR
val sourcesJar by tasks.registering(Jar::class) {
    archiveClassifier.set("sources")
    from(sourceSets.main.get().allSource)
}

// Generate javadoc JAR (empty for Kotlin, but required by Maven Central)
val javadocJar by tasks.registering(Jar::class) {
    archiveClassifier.set("javadoc")
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
            artifact(sourcesJar)
            artifact(javadocJar)
            
            pom {
                name.set("ekoDB Kotlin Client")
                description.set("Official Kotlin client library for ekoDB")
                url.set("https://github.com/ekoDB/ekodb-client")
                
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }
                
                developers {
                    developer {
                        id.set("ekodb")
                        name.set("ekoDB Team")
                        email.set("support@ekodb.io")
                    }
                }
                
                scm {
                    connection.set("scm:git:git://github.com/ekoDB/ekodb-client.git")
                    developerConnection.set("scm:git:ssh://github.com/ekoDB/ekodb-client.git")
                    url.set("https://github.com/ekoDB/ekodb-client")
                }
            }
        }
    }
    
    // No custom repositories needed - nexusPublishing handles it
}

signing {
    val signingKey = project.findProperty("signingInMemoryKey") as String? ?: System.getenv("SIGNING_KEY")
    val signingPassword = project.findProperty("signingInMemoryKeyPassword") as String? ?: System.getenv("SIGNING_PASSWORD")
    
    if (signingKey != null && signingPassword != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
    }
    
    sign(publishing.publications["maven"])
}

nexusPublishing {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://central.sonatype.com/api/v1/publisher/"))
            snapshotRepositoryUrl.set(uri("https://central.sonatype.com/api/v1/publisher/"))
            
            username.set(project.findProperty("ossrhUsername") as String? ?: System.getenv("OSSRH_USERNAME"))
            password.set(project.findProperty("ossrhPassword") as String? ?: System.getenv("OSSRH_PASSWORD"))
        }
    }
    
    // Use new publishing API
    useStaging.set(false)
}
