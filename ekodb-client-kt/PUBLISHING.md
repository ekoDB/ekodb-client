# Publishing ekoDB Kotlin Client to Maven Central

This guide explains how to publish the Kotlin client to Maven Central.

## Prerequisites

### 1. Sonatype Account

- Create account at: https://central.sonatype.com/
- Request access to `io.ekodb` namespace
- Get your username and password/token

### 2. GPG Key for Signing

```bash
# Generate GPG key
gpg --gen-key

# List keys
gpg --list-keys

# Export public key (replace KEY_ID with your key ID)
gpg --keyserver keyserver.ubuntu.com --send-keys KEY_ID

# Export private key for Gradle
gpg --export-secret-keys KEY_ID > secring.gpg
```

### 3. Configure Gradle Properties

Create or edit `~/.gradle/gradle.properties`:

```properties
# Sonatype credentials
ossrhUsername=YOUR_USERNAME
ossrhPassword=YOUR_PASSWORD_OR_TOKEN

# Signing configuration
signing.keyId=YOUR_KEY_ID (last 8 chars)
signing.password=YOUR_GPG_PASSPHRASE
signing.secretKeyRingFile=/path/to/secring.gpg

# Or use in-memory signing (recommended for CI/CD)
signingInMemoryKey=YOUR_BASE64_ENCODED_KEY
signingInMemoryKeyPassword=YOUR_GPG_PASSPHRASE
```

## Publishing Steps

### 1. Update Version

Edit `build.gradle.kts`:

```kotlin
version = "0.1.0"  // Update this
```

### 2. Build and Test

```bash
cd ekodb-client-kt

# Clean build
./gradlew clean

# Run tests
./gradlew test

# Build
./gradlew build
```

### 3. Publish to Maven Central

```bash
# Publish to staging repository
./gradlew publish

# Or publish and release in one step
./gradlew publishToSonatype closeAndReleaseSonatypeStagingRepository
```

### 4. Verify Publication

After publishing, it takes:

- **~10 minutes** to appear on Maven Central search
- **~2 hours** to sync to all mirrors

Check at:

- https://central.sonatype.com/artifact/io.ekodb/ekodb-client-kt
- https://repo1.maven.org/maven2/io/ekodb/ekodb-client-kt/

## Using the Published Library

### Gradle (Kotlin DSL)

```kotlin
dependencies {
    implementation("io.ekodb:ekodb-client-kt:0.1.0")
}
```

### Gradle (Groovy)

```groovy
dependencies {
    implementation 'io.ekodb:ekodb-client-kt:0.1.0'
}
```

### Maven

```xml
<dependency>
    <groupId>io.ekodb</groupId>
    <artifactId>ekodb-client-kt</artifactId>
    <version>0.1.0</version>
</dependency>
```

## Automated Publishing with GitHub Actions

Create `.github/workflows/publish-kotlin.yml`:

```yaml
name: Publish Kotlin Client

on:
  push:
    tags:
      - "kt-v*"

jobs:
  publish:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Set up JDK 17
        uses: actions/setup-java@v4
        with:
          java-version: "17"
          distribution: "temurin"

      - name: Publish to Maven Central
        env:
          OSSRH_USERNAME: ${{ secrets.OSSRH_USERNAME }}
          OSSRH_PASSWORD: ${{ secrets.OSSRH_PASSWORD }}
          SIGNING_KEY: ${{ secrets.SIGNING_KEY }}
          SIGNING_PASSWORD: ${{ secrets.SIGNING_PASSWORD }}
        run: |
          cd ekodb-client-kt
          ./gradlew publish closeAndReleaseSonatypeStagingRepository
```

## Troubleshooting

### "401 Unauthorized"

- Check your Sonatype credentials in `gradle.properties`
- Verify you have access to `io.ekodb` namespace

### "No valid OpenPGP data found"

- Ensure GPG key is properly configured
- Check `signing.secretKeyRingFile` path
- Verify GPG passphrase is correct

### "Repository not found"

- Ensure you've requested access to `io.ekodb` namespace
- Wait for Sonatype approval (can take 1-2 business days)

### Publication not appearing

- Check staging repository at: https://central.sonatype.com/
- Verify all required metadata is present in POM
- Ensure artifacts are properly signed

## Version Management

Follow semantic versioning:

- **0.1.0** - Initial release
- **0.1.1** - Patch (bug fixes)
- **0.2.0** - Minor (new features, backward compatible)
- **1.0.0** - Major (breaking changes)

## Checklist Before Publishing

- [ ] All tests pass (`./gradlew test`)
- [ ] Version updated in `build.gradle.kts`
- [ ] CHANGELOG.md updated
- [ ] README.md updated with new version
- [ ] Examples tested with new version
- [ ] GPG signing configured
- [ ] Sonatype credentials configured
- [ ] Git tag created: `git tag kt-v0.1.0`
- [ ] Tag pushed: `git push origin kt-v0.1.0`

## Post-Publication

1. Update README.md badge with new version
2. Create GitHub release with changelog
3. Announce on social media/blog
4. Update documentation site
5. Notify users of new features

## Resources

- [Maven Central Portal](https://central.sonatype.com/)
- [Publishing Guide](https://central.sonatype.org/publish/publish-guide/)
- [Gradle Signing Plugin](https://docs.gradle.org/current/userguide/signing_plugin.html)
- [Gradle Maven Publish Plugin](https://docs.gradle.org/current/userguide/publishing_maven.html)
