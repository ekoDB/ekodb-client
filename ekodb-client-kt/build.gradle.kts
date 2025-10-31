plugins {
    kotlin("jvm") version "2.0.21"
    kotlin("plugin.serialization") version "2.0.21"
    `maven-publish`
    signing
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0"
}

group = "io.ekodb"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    // Kotlin
    implementation(kotlin("stdlib"))
    
    // Coroutines for async operations
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.9.0")
    
    // Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.3")
    
    // HTTP client
    implementation("io.ktor:ktor-client-core:3.0.1")
    implementation("io.ktor:ktor-client-cio:3.0.1")
    implementation("io.ktor:ktor-client-content-negotiation:3.0.1")
    implementation("io.ktor:ktor-serialization-kotlinx-json:3.0.1")
    implementation("io.ktor:ktor-client-websockets:3.0.1")
    implementation("io.ktor:ktor-client-logging:3.0.1")
    implementation("io.ktor:ktor-client-encoding:3.0.1")
    
    // DateTime
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.6.1")
    
    // Testing
    testImplementation(kotlin("test"))
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.9.0")
    testImplementation("io.ktor:ktor-client-mock:3.0.1")
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
