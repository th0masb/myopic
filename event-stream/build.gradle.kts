plugins {
    idea
    kotlin("jvm") version "1.4.10"
}

group = "com.myopic"
version = "1.0-SNAPSHOT"

repositories {
    jcenter()
    mavenCentral()
}

dependencies {
    implementation("org.slf4j:slf4j-simple:1.7.29")
    implementation("io.github.microutils:kotlin-logging:1.12.0")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.11.+")
    runtimeOnly("org.jetbrains.kotlin:kotlin-reflect:1.4.0")

    testImplementation("org.junit.jupiter:junit-jupiter:5.6.0")
}

tasks {
    compileKotlin {
        kotlinOptions.jvmTarget = "11"
        kotlinOptions.freeCompilerArgs = listOf("-Xallow-result-return-type")
    }
    compileTestKotlin {
        kotlinOptions.jvmTarget = "11"
        kotlinOptions.freeCompilerArgs = listOf("-Xallow-result-return-type")
    }

    withType<Test> {
        useJUnitPlatform()
    }

    withType<Jar> {
        manifest {
            attributes["Main-Class"] = "com.myopic.eventstream.MainKt"
        }
    }
}