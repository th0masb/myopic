FROM openjdk:11.0.8-slim AS builder
RUN mkdir -p /build
WORKDIR /build
COPY ./ ./
RUN ./gradlew assemble

FROM openjdk:11.0.8-jre-slim
RUN mkdir -p /app
WORKDIR /app
COPY --from=builder /build/build/libs/event-stream.jar .
CMD ["java", "-jar", "event-stream.jar"]
