FROM python:3.9-slim-bullseye as builder
RUN pip install amazon-kclpy==2.0.6


FROM amazoncorretto:16 as runner
COPY --from=builder /usr/local/lib/python3.9/site-packages/amazon_kclpy/jars /usr/local/lib/jars
COPY multilang_daemon/sample.properties sample.properties
CMD ["java", "-cp", "/usr/local/lib/jars/*", "software.amazon.kinesis.multilang.MultiLangDaemon", "--properties-file", "sample.properties"]
